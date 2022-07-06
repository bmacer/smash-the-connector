extern crate reqwest;

use rand::Rng;
use std::error::Error;

use std::time::Duration;
use std::{thread, time};

fn main() {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    loop {
        let a = rand::thread_rng().gen_range(1..256);
        let b = rand::thread_rng().gen_range(1..256);
        let c = rand::thread_rng().gen_range(1..256);
        let d = rand::thread_rng().gen_range(1..256);

        let ip = format!("http://{}.{}.{}.{}", a, b, c, d);
        println!("Calling {}", ip);
        thread::sleep(Duration::from_millis(250));

        let x = match client.get(&ip).send() {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e);
                let y = e.to_string();
                println!("y: {:?}", y);
                if y.contains("Too many open files (os error 24)") {
                    println!("inside");
                    thread::sleep(Duration::from_secs(3));
                }
                continue;
            }
        };

        println!("x: {:?}", x);

        println!("...");
    }

    // let response_text = reqwest::get(&ip)
    //     .expect("Error making request")
    //     .text()
    //     .expect("Couldn't read response");

    // println!("Response Text: {}", response_text);
}
