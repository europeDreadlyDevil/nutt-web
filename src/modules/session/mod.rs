use crate::modules::session::cookie_session::CookieSession;

pub mod cookie_session;

#[derive(Debug, Clone)]
pub enum Session {
    Cookie(CookieSession),
}

pub enum SessionType {
    Cookie,
}
