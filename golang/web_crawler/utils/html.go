package utils

import (
	"strings"

	"golang.org/x/net/html"
)

func ExtractLinks(htmlContent string) ([]string, error) {
	links := make([]string, 0)

	doc, err := html.Parse(strings.NewReader(htmlContent))
	if err != nil {
		return nil, err
	}

	var traverse func(*html.Node)
	traverse = func(node *html.Node) {
		if node.Type == html.ElementNode && node.Data == "a" {
			for _, attr := range node.Attr {
				if attr.Key == "href" {
					links = append(links, attr.Val)
					break
				}
			}
		}

		for child := node.FirstChild; child != nil; child = child.NextSibling {
			traverse(child)
		}
	}

	traverse(doc)

	return links, nil
}
