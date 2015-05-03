# rust-chunked-transfer-coding

[![Build Status](https://travis-ci.org/frewsxcv/rust-chunked-transfer-coding.svg?branch=master)](https://travis-ci.org/frewsxcv/rust-chunked-transfer-coding)
[![chunked-transfer-encoding on Crates.io](https://meritbadge.herokuapp.com/chunked-transfer-coding)](https://crates.io/crates/chunked-transfer-coding)

Encoder and decoder for HTTP chunked transfer coding. For more information about chunked transfer encoding:

* [RFC 2616 § 3.6.1](http://www.w3.org/Protocols/rfc2616/rfc2616-sec3.html#sec3.6.1)
* [Wikipedia: Chunked transfer encoding](https://en.wikipedia.org/wiki/Chunked_transfer_encoding)

## Example

### Decoding

```rust
use chunked_transfer::Decoder;
use std::io::Read;

let encoded = b"3\r\nhel\r\nb\r\nlo world!!!\r\n0\r\n\r\n";
let mut decoder = Decoder::new(encoded as &[u8]);

let mut decoded = String::new();
decoder.read_to_string(&mut decoded);

assert_eq!(decoded, "hello world!!!");
```

### Encoding

```rust
use chunked_transfer::Encoder;
use std::io::Write;

let mut decoded = b"hello world";
let mut encoded: Vec<u8> = vec![];

{
    let mut encoder = Encoder::with_chunks_size(&mut encoded, 5);
    encoder.write_all(decoded as &[u8]);
}

assert_eq!(encoded, b"5\r\nhello\r\n5\r\n worl\r\n1\r\nd\r\n0\r\n\r\n");
```
