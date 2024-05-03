package main

import (
	"bufio"
	"fmt"
	"net/http"
	"os"
	"sync"
	"time"
)

const (
	concurrency = 128
	timeout     = 1 * time.Second
)

func main() {
	// 读取 IP 列表
	ips, err := readLines("ips.txt")
	if err != nil {
		fmt.Println("Failed to read IP list:", err)
		return
	}

	// 创建等待组
	var wg sync.WaitGroup
	wg.Add(len(ips))

	// 限制并发请求的数量
	semaphore := make(chan struct{}, concurrency)

	// 遍历 IP 列表
	for _, ip := range ips {
		semaphore <- struct{}{} // 占用一个信号量
		go func(ip string) {
			defer wg.Done()
			defer func() { <-semaphore }() // 释放一个信号量

			// 创建带超时的 HTTP 客户端
			client := http.Client{
				Timeout: timeout,
			}

			// 请求 IP:54321
			resp, err := client.Get("http://" + ip + ":54321")
			if err != nil {
				fmt.Printf("Error fetching %s: %s\n", ip, err)
				return
			}
			defer resp.Body.Close()

			// 判断 HTTP 状态码是否为 200
			if resp.StatusCode == http.StatusOK {
				fmt.Printf("%s returned status code 200\n", ip)
				writeLine("ips_new.txt", ip)
			}
		}(ip)
	}

	// 等待所有请求完成
	wg.Wait()
	fmt.Println("All requests completed.")
}

// readLines 从文件中读取每行内容并返回字符串切片
func readLines(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

// writeLine 将字符串写入文件的新一行
func writeLine(filename, line string) error {
	file, err := os.OpenFile(filename, os.O_APPEND|os.O_WRONLY|os.O_CREATE, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	if _, err := file.WriteString(line + "\n"); err != nil {
		return err
	}
	return nil
}
