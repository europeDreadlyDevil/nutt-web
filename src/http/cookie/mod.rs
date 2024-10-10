use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct CookieRes {
    name: String,
    value: String,
    domain: Option<String>,
    path: Option<String>,
    expires: Option<SystemTime>,
    max_age: Option<u64>,
    secure: bool,
    http_only: bool,
    same_site: Option<SameSite>,
}

#[derive(Debug, Clone)]

pub struct CookieReq {
    value: String,
}

impl CookieReq {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl CookieRes {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn set_expires(&mut self, expires: SystemTime) {
        self.expires = Some(expires);
    }

    pub fn set_max_age(&mut self, max_age: u64) {
        self.max_age = Some(max_age);
    }

    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = http_only;
    }

    pub fn set_same_site(&mut self, same_site: SameSite) {
        self.same_site = Some(same_site);
    }

    pub fn to_string(&self) -> String {
        let mut cookie_string = format!("{}={}", self.name, self.value);

        if let Some(ref domain) = self.domain {
            cookie_string.push_str(&format!("; Domain={}", domain));
        }

        if let Some(ref path) = self.path {
            cookie_string.push_str(&format!("; Path={}", path));
        }

        if let Some(ref expires) = self.expires {
            if let Ok(expires_time) = expires.duration_since(SystemTime::UNIX_EPOCH) {
                cookie_string.push_str(&format!(
                    "; Expires={:?}",
                    expires_time.as_secs() // Здесь можно использовать форматирование для красивой даты
                ));
            }
        }

        if let Some(max_age) = self.max_age {
            cookie_string.push_str(&format!("; Max-Age={}", max_age));
        }

        if self.secure {
            cookie_string.push_str("; Secure");
        }

        if self.http_only {
            cookie_string.push_str("; HttpOnly");
        }

        if let Some(ref same_site) = self.same_site {
            let same_site_str = match same_site {
                SameSite::Strict => "Strict",
                SameSite::Lax => "Lax",
                SameSite::None => "None",
            };
            cookie_string.push_str(&format!("; SameSite={}", same_site_str));
        }

        cookie_string
    }
}

pub struct CookieJarIter<'a> {
    slice: std::collections::hash_map::Iter<'a, String, CookieReq>,
}

#[derive(Debug, Clone)]
pub struct CookieJar {
    cookies: HashMap<String, CookieReq>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    pub(crate) fn push_cookie(&mut self, name: &str, cookie: CookieReq) {
        self.cookies.insert(name.to_string(), cookie);
    }

    pub fn iter(&self) -> CookieJarIter {
        CookieJarIter {
            slice: self.cookies.iter(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&CookieReq> {
        self.cookies.get(key)
    }
}

impl<'a> Iterator for CookieJarIter<'a> {
    type Item = (&'a String, &'a CookieReq);

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}
