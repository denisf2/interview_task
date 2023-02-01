use std::io::Error;

// use std::env;
use clap::Parser;
use url::Url;

#[derive(Parser)]
struct CliPingParams {
    // interval to send get request
    interval: u64,
    // checking url
    url: String,
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    
    let params = CliPingParams::parse();

    if !validate_url(&params.url) {
        return;
    }

    rust_interview_test::ping(params.interval, params.url)
}

fn validate_url(url: &str) -> bool {
    let _parsed = match Url::parse(&url) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("URL parsing error: {}", x);
            return false;
        }
    };

    // println!("The path part of the URL is: {}", parsed.path());

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url() {
        assert_eq!(true, validate_url("https://google.com"));
    }

    #[test]
    fn test_validate_invalid_url() {
        assert_ne!(true, validate_url("https//google.com"));
    }
}
