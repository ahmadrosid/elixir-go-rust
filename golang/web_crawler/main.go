package main

import (
	"flag"
	"fmt"
	"log"
	"net/url"
	"strings"
	"sync"
	"web_crawler/utils"
)

type WebCrawler struct {
	startURL   *url.URL
	visitedURL map[string]bool
	mutex      sync.RWMutex
	wg         sync.WaitGroup
	semaphore  chan struct{}
}

func NewWebCrawler(startURL string, maxGoroutines int) *WebCrawler {
	parsedURL, _ := url.Parse(startURL)
	return &WebCrawler{
		startURL:   parsedURL,
		visitedURL: make(map[string]bool),
		semaphore:  make(chan struct{}, maxGoroutines),
	}
}

func (w *WebCrawler) Start() {
	w.crawl(w.startURL.String())
	w.wg.Wait()
	fmt.Printf("Finished crawling %s\n", w.startURL.String())
}

func (w *WebCrawler) Total() int {
	w.mutex.RLock()
	defer w.mutex.RUnlock()
	total := len(w.visitedURL)
	return total
}

func (w *WebCrawler) crawl(rawURL string) {
	w.mutex.Lock()
	if _, found := w.visitedURL[rawURL]; found {
		w.mutex.Unlock()
		return
	}
	w.visitedURL[rawURL] = true
	w.mutex.Unlock()

	// fmt.Printf("Start crawling: %s \n", rawURL)
	parsedRawUrl, err := url.Parse(rawURL)
	if err != nil {
		fmt.Println(err)
		return
	}

	bodyContent, err := utils.Request(rawURL)
	if err != nil {
		fmt.Println(err)
		return
	}

	links, err := utils.ExtractLinks(string(bodyContent))
	if err != nil {
		log.Fatal(err)
	}

	host := parsedRawUrl.Scheme + "://" + parsedRawUrl.Host
	for _, link := range links {
		parsedLink, err := url.Parse(link)
		if err != nil {
			continue
		}

		absoluteLink := parsedRawUrl.ResolveReference(parsedLink)
		absoluteLink.Fragment = ""
		absoluteLink.RawQuery = ""

		if !strings.HasPrefix(absoluteLink.String(), host) {
			continue
		}

		w.wg.Add(1)
		go func(link string) {
			w.semaphore <- struct{}{} // Acquire semaphore
			defer func() {
				<-w.semaphore // Release semaphore
				w.wg.Done()
			}()
			w.crawl(link)

		}(absoluteLink.String())
	}
}

func main() {
	maxGoroutines := flag.Int("worker", 10, "the maximum number of goroutines for crawling")
	url := flag.String("url", "https://http.dev/", "the URL to crawl")
	flag.Parse()

	result := utils.Measure(func() interface{} {
		crawler := NewWebCrawler(*url, *maxGoroutines)
		crawler.Start()
		return crawler.Total()
	})
	fmt.Printf("Total links scraped: %v, concurrency: %v\n", result, *maxGoroutines)
}
