use tokio::io::AsyncReadExt;
use tracing_log::log::{log, Level};
use crate::http::cookie::{CookieJar, CookieReq};
use crate::http::method::Method;
use crate::http::request::{Request, RequestBuilder};
use crate::external::displayable::DisplayableVec;
use crate::server::stream::Stream;
use crate::server::stream::stream_reader::StreamReader;

pub struct StreamHandler;

impl StreamHandler {
    pub async fn handle_stream<T: Stream + AsyncReadExt + Unpin>( mut stream: T, )
        -> anyhow::Result<(Method, String, T, Request)> {
        let request = StreamReader::new(&mut stream).read_req().await;
        let tokens: Vec<&str> = request.lines().nth(0).unwrap().split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(anyhow::Error::msg("Invalid HTTP request line"));
        }

        let method = match tokens[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => return Err(anyhow::Error::msg("Unsupported HTTP method")),
        };

        let path = tokens[1].to_string();

        let mut headers = DisplayableVec(vec![]);
        let mut i = 1;
        let mut is_header = true;
        let mut body = String::new();
        while let Some(line) = request.lines().nth(i) {
            if line.is_empty() {
                is_header = false
            }
            if is_header {
                headers.0.push(line.to_string());
            } else {
                body.push_str(line.trim())
            }
            i += 1;
        }

        let mut cookies = CookieJar::new();

        for header in &headers.0 {
            if header.starts_with("Cookie: ") {
                for cookie in header[8..].split(";") {
                    let eq_pos = cookie.find("=").unwrap();
                    cookies.push_cookie(
                        &cookie[..eq_pos],
                        CookieReq::new(cookie[eq_pos + 1..].to_string()),
                    )
                }
            }
        }

        log!(
            Level::Info,
            "Request Method: {}, Path: {}, Headers: {}, Body: {}",
            method,
            path,
            headers,
            body
        );

        Ok((
            method.clone(),
            path,
            stream,
            RequestBuilder::new(method, serde_json::to_value(body)?)
                .set_cookie_jar(cookies)
                .build(),
        ))
    }

}