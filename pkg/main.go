package main

import (
	"bufio"
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"
	"sync"
)

type WebDirScanner struct {
	url      string
	wordlist string

	wg *sync.WaitGroup
}

func (w *WebDirScanner) scanDir(dir string) {
	if !strings.HasSuffix(w.url, "/") {
		w.url += "/"
	}
	uri := w.url + dir

	resp, err := http.Get(uri)
	if err != nil {
		fmt.Printf("Http request error: %v\n", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode == 200 {
		fmt.Printf("[*] Directory found: /%s\n", dir)
	}
}

func (w *WebDirScanner) Scan() {
	defer w.wg.Done()

	file, err := os.Open(w.wordlist)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	const numWorkers = 100
	jobs := make(chan string)
	var innerWg sync.WaitGroup

	for i := 0; i < numWorkers; i++ {
		innerWg.Add(1)
		go func() {
			defer innerWg.Done()
			for dir := range jobs {
				w.scanDir(dir)
			}
		}()
	}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line != "" {
			jobs <- line
		}
	}
	close(jobs)

	innerWg.Wait()
}

func main() {
	webScanner := WebDirScanner{
		url:      "http://example.com",
		wordlist: "test.txt",
		wg:       &sync.WaitGroup{},
	}
	log.Printf("Starting gbrute on %s\n", webScanner.url)
	webScanner.Scan()
}
