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
}

func (w *WebDirScanner) scanDir(dir string) {
	if !strings.HasSuffix(w.url, "/") {
		w.url += "/"
	}
	uri := w.url + dir

	//fmt.Printf("Scanning dir /%s\n", dir)
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

	file, err := os.Open(w.wordlist)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Jobs
	const numWorkers = 200
	jobs := make(chan string)
	var innerWg sync.WaitGroup

	// scanning dir
	for i := 0; i < numWorkers; i++ {
		innerWg.Add(1)
		go func() {
			defer innerWg.Done()
			for dir := range jobs {
				w.scanDir(dir)
			}
		}()
	}

	// sending dirs to bust to jobs channel
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
		url:      "http://scanme.nmap.org",
		wordlist: "test.txt",
	}
	log.Printf("Starting gbrute on %s\n", webScanner.url)
	webScanner.Scan()
}
