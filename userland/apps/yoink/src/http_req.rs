use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HttpRequest {
    pub method: String,
    pub host: String,
    pub path: String,
    pub port: u16,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub download: bool,
    pub filename: String,
}

impl HttpRequest {
    pub fn new(args: &Vec<String>) -> Result<HttpRequest, ()> {
        let mut req = HttpRequest::default();

        if args[1] == "-o" {
            req.download = true;
            req.method = String::from("GET");

            if args[2].starts_with("http://") {
                let filename = args[2].strip_prefix("http://");
                req.filename = filename.unwrap_or("").to_string();
                // find path and hosting shi
            } else {
                req.filename = args[2].clone();
            }
        }

        Ok(req)
    }
}
