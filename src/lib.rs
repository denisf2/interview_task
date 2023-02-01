use std::thread;
use std::time::{Duration, Instant};

pub fn ping(time_interval: u64, url: String) {
    // to convert async run
    let handle = thread::spawn(move || {
        // let handle = thread::spawn(move || async move {
        // println!("thread closure start");

        loop {
            let time = Instant::now();

            // post get request
            match reqwest::blocking::get(&url) {
                Ok(x) => {
                    // Checking 'http://example.com/'. Result: OK(200)

                    let status = x.status().to_string();
                    let res: Vec<&str> = status.split(char::is_whitespace).collect();

                    println!("Checking '{url}'. Result: {}({})", &res[1], &res[0]);
                }

                Err(x) => {
                    println!("{}", x.to_string());
                    return x;
                }
            }

            // sleep some time
            let dif = time.elapsed().as_secs();
            if dif < time_interval {
                thread::sleep(Duration::from_secs(time_interval - dif));
            }
        }
    });

    let error = handle.join().unwrap();
    eprintln!("GET request error is: {}", error.to_string());
}

// async fn ping(interval: u32, url: &str) {
//     loop {
//         // post get request
//         match reqwest::get(url).await {
//             Ok(x) => {
//                 println!("{}", x.status())
//             }
//             Err(x) => {
//                 println!("{}", x.to_string())
//             }
//         }

//         // let client = reqwest::Client::new();
//         // let mut res = match client.get(url).send().await {
//         //     Ok(x) => x,
//         //     Err(x) => return,
//         // };

//         // sleep some time
//         thread::sleep(Duration::from_secs(interval as u64));
//     }
// }
