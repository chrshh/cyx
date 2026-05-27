use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;

use crate::http_req::HttpRequest;

mod http_req;

/*
* yoink    [-FLAGS]     [PARAMS]      [ENDPOINT]
* yoink       -o        page.html  https://example.com
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: yoink <url>");
        process::exit(1);
    }

    let req = Some(HttpRequest::new(&args));

    let url = &args[1];

    let stripped = url.strip_prefix("http://").unwrap_or_else(|| {
        eprintln!("only supporting http://");
        process::exit(1);
    });

    let (host, path) = match stripped.find('/') {
        Some(i) => (&stripped[..i], &stripped[i..]),
        None => (stripped, "/"),
    };

    if let Err(e) = run(host, path) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run(host: &str, path: &str) -> std::io::Result<()> {
    let addr = format!("{}:80", host);
    let mut stream = TcpStream::connect(&addr)?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         User-Agent: fetchy/0.1\r\n\
         \r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    print!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
