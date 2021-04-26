use std::fmt;

#[path = "./polynomial.rs"]
mod polynomial;

// Public key should maybe also have a `size` field
#[derive(Debug)]
pub struct PublicKey<'a> {
    pub a: polynomial::Polynomial<'a>,
    pub b: polynomial::Polynomial<'a>,
}

impl fmt::Display for PublicKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(b={}, a={})", self.b, self.a)
    }
}
