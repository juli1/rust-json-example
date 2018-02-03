use futures::{Future, Stream};
use hyper_tls::HttpsConnector;
use hyper::Client;
use tokio_core::reactor::Core;

use std::string::String;
use std::str;
use std;


// Return the string that corresponds to the HTTP
// content of the given URL.
pub fn http_get(url : &str) -> String {
    let mut core = Core::new().unwrap();
    //let client = Client::new(&core.handle());
    let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                .build(&core.handle());

    let uri = url.parse().unwrap();
    let work = client.get(uri).and_then (|res| {res.body().concat2()})
        .map(|chunk| std::str::from_utf8(&chunk).expect("error handling").to_string());
    let r = core.run(work).unwrap();
    return r;
}



