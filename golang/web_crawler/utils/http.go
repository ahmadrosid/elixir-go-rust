package utils

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"time"
)

const maxRetries = 3

func Request(url string) ([]byte, error) {
	ctx := context.Background()
	client := &http.Client{
		Timeout: 5 * time.Second,
	}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		fmt.Println("Error creating request:", err)
		return nil, err
	}

	var resp *http.Response
	for retries := 0; retries < maxRetries; retries++ {
		if resp != nil {
			resp.Body.Close()
		}

		req = req.WithContext(ctx)

		resp, err = client.Do(req)
		if err != nil {
			// Check if the error is due to a timeout
			if err, ok := err.(net.Error); ok && err.Timeout() {
				fmt.Println("Request timed out")
				continue // Retry the request
			} else {
				fmt.Println("Error making request:", err)
				return nil, err
			}
		}

		defer resp.Body.Close()
		start := time.Now()

		bodyContent, err := io.ReadAll(resp.Body)
		if err != nil {
			fmt.Println(err)
			return nil, err
		}
		elapsed := time.Since(start)
		seconds := elapsed.Seconds()
		if seconds > 2 {
			fmt.Printf("Read response body to string %s in: \033[32m%f s\033[0m\n", url, seconds)
		}

		return bodyContent, nil
	}

	return nil, fmt.Errorf("maximum retries exceeded")
}
