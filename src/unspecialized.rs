extern crate http;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde_json;
extern crate serde;

use std::string::String;
use std::process;
use std::env;

mod common;

// Parse the JSON document and convert it to CurrencyPrice
fn parse_content (content : String) -> serde_json::Value {
    let s_slice: &str = &*content;
    return serde_json::from_str(s_slice).unwrap();
}


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of argument");
        process::exit(1);
    }

    let url = args.get(1).unwrap();

    let content = common::http_get(url);
    let values = parse_content(content);
    print!("{:?}",values["USD"]);
}
