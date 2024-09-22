use std::fmt::{Display, Formatter};

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Method {
    GET
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", match self {
            Method::GET => "GET"
        })
    }
}