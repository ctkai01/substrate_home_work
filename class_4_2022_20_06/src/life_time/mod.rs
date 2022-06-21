
use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl fmt::Display for StrDisplayable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

pub fn print_vector() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_foo = StrDisplayable(vec);
        println!("{}",vec_foo);
}