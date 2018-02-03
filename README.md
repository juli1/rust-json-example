# Parsing Json document with Rust

When I started to look for parsing Json from a website
using Rust, I was surprised by the lack of resources.
This is an example that shows you how to do it.

This is an example to parse JSON values from the web.
The following program show how to crawl
Json data from the web.

In the following example, we crawl data from https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD,EUR

The Json data looks like this:
```
{
   "USD":9237.64,
   "EUR":7416.1
}
```

# Parsing to a data structure
You can parse the Json data into a specific/formal
structure. This is shown in the source file
```specialized.rs```.

# Parsing to a generic structure
If you do not know the format of the data
structure ahead of time, you can just store
the result in the generic data structure serde_json::Value.
This is shown in ```unspecialized.rs```

# Usage
```
cargo build
cargo run --specialized 'https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD,EUR'
cargo run --unspecialized 'https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD,EUR'
```

# Notes
* [Serde](https://serde.rs/)
