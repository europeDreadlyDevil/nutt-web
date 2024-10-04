use std::fmt::Display;

pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    UnAuthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    UnprocessableEntity = 422,
    TooManyRequests = 429,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    InsufficientStorage = 507,
    LoopDetected = 508,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StatusCode::Ok => "200 OK".to_string(),
            StatusCode::Created => "201 Created".to_string(),
            StatusCode::Accepted => "202 Accepted".to_string(),
            StatusCode::NonAuthoritativeInformation => {
                "203 Non-Authoritative Information".to_string()
            }
            StatusCode::NoContent => "204 No Content".to_string(),
            StatusCode::ResetContent => "205 Reset Content".to_string(),
            StatusCode::PartialContent => "206 Partial Content".to_string(),

            StatusCode::MultipleChoices => "300 Multiple Choices".to_string(),
            StatusCode::MovedPermanently => "301 Moved Permanently".to_string(),
            StatusCode::Found => "302 Found".to_string(),
            StatusCode::SeeOther => "303 See Other".to_string(),
            StatusCode::NotModified => "304 Not Modified".to_string(),
            StatusCode::UseProxy => "305 Use Proxy".to_string(),
            StatusCode::TemporaryRedirect => "307 Temporary Redirect".to_string(),
            StatusCode::PermanentRedirect => "308 Permanent Redirect".to_string(),

            StatusCode::BadRequest => "400 Bad Request".to_string(),
            StatusCode::UnAuthorized => "401 Unauthorized".to_string(),
            StatusCode::PaymentRequired => "402 Payment Required".to_string(),
            StatusCode::Forbidden => "403 Forbidden".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
            StatusCode::MethodNotAllowed => "405 Method Not Allowed".to_string(),
            StatusCode::NotAcceptable => "406 Not Acceptable".to_string(),
            StatusCode::ProxyAuthenticationRequired => {
                "407 Proxy Authentication Required".to_string()
            }
            StatusCode::RequestTimeout => "408 Request Timeout".to_string(),
            StatusCode::Conflict => "409 Conflict".to_string(),
            StatusCode::Gone => "410 Gone".to_string(),
            StatusCode::LengthRequired => "411 Length Required".to_string(),
            StatusCode::PreconditionFailed => "412 Precondition Failed".to_string(),
            StatusCode::PayloadTooLarge => "413 Payload Too Large".to_string(),
            StatusCode::UriTooLong => "414 URI Too Long".to_string(),
            StatusCode::UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            StatusCode::RangeNotSatisfiable => "416 Range Not Satisfiable".to_string(),
            StatusCode::ExpectationFailed => "417 Expectation Failed".to_string(),
            StatusCode::ImATeapot => "418 I'm a teapot".to_string(),
            StatusCode::UnprocessableEntity => "422 Unprocessable Entity".to_string(),
            StatusCode::TooManyRequests => "429 Too Many Requests".to_string(),

            StatusCode::InternalServerError => "500 Internal Server Error".to_string(),
            StatusCode::NotImplemented => "501 Not Implemented".to_string(),
            StatusCode::BadGateway => "502 Bad Gateway".to_string(),
            StatusCode::ServiceUnavailable => "503 Service Unavailable".to_string(),
            StatusCode::GatewayTimeout => "504 Gateway Timeout".to_string(),
            StatusCode::HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),
            StatusCode::InsufficientStorage => "507 Insufficient Storage".to_string(),
            StatusCode::LoopDetected => "508 Loop Detected".to_string(),
        };
        write!(f, "{}", str)
    }
}
