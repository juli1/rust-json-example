extern crate http;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::string::String;
use std::process;
use std::env;

mod common;

// Structure of the JSON document
// We can also just get a serde_json::Value instead
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct CurrencyPrice {
    USD: f64,
    EUR: f64,
}

// Parse the JSON document and convert it to CurrencyPrice
fn parse_content (content : String) -> CurrencyPrice {
    let s_slice: &str = &*content;
    let v: CurrencyPrice = serde_json::from_str(s_slice).unwrap();
    return v;
}


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of argument");
        process::exit(1);
    }

    let url = args.get(1).unwrap();

    let content = common::http_get(url);
    let values : CurrencyPrice  = parse_content(content);
    print!("{:?}",values.USD);
}
