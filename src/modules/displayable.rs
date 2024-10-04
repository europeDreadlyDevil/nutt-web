use std::fmt::{Display, Formatter};

pub(crate) struct DisplayableVec<T>(pub Vec<T>);

impl<T> Display for DisplayableVec<T>
where T: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for i in 0..self.0.len()-1{
            out.push_str(self.0[i].to_string().replace("\r", "").replace("\n", "").as_str());
            out.push_str("; ")
        }
        out.push_str(self.0[self.0.len()-1].to_string().as_str());

        write!(f, "{out}")
    }
}