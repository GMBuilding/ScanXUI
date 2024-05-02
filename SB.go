package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"sync"
	"time"
)

func main() {
	// 打开 ips.txt 文件
	file, err := os.Open("ip.txt")
	if err != nil {
		fmt.Println("无法打开 ips.txt 文件:", err)
		return
	}
	defer file.Close()

	// 创建 res.txt 文件
	resFile, err := os.Create("res.txt")
	if err != nil {
		fmt.Println("无法创建 res.txt 文件:", err)
		return
	}
	defer resFile.Close()

	// 创建一个 WaitGroup 用于等待所有请求完成
	var wg sync.WaitGroup

	// 使用 bufio 逐行读取 ips.txt 文件
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		ip := scanner.Text()
		// 增加 WaitGroup 的计数器
		wg.Add(1)
		// 启动一个 goroutine 发送 POST 请求
		go func(ip string) {
			defer wg.Done()
			if err := sendPostRequest(ip); err != nil {
				fmt.Println("IP 无效:", ip)
			} else {
				fmt.Println("IP 是可用的:", ip)
				// 写入到 res.txt 文件中
				fmt.Fprintf(resFile, "%s\n", ip)
			}
		}(ip)
	}

	// 等待所有请求完成
	wg.Wait()
}

// 发送 POST 请求
func sendPostRequest(ip string) error {
	// 构建 JSON 数据
	data := map[string]string{"username": "ares", "password": "aries"}
	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	// 构建请求 URL
	url := fmt.Sprintf("http://%s:11451/login", ip)

	// 创建 HTTP 客户端
	client := &http.Client{
		Timeout: 5 * time.Second, // 设置超时时间为 5 秒
	}

	// 发送 POST 请求
	resp, err := client.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// 读取响应体
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	// 如果状态码为 200 则返回 nil，否则返回错误信息
	if resp.StatusCode == http.StatusOK {
		return nil
	} else {
		return fmt.Errorf("错误响应: %s", string(body))
	}
}
