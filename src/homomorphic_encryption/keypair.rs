#[path = "./public_key.rs"]
mod public_key;
use self::public_key::PublicKey;

#[path = "./polynomial.rs"]
mod polynomial;
use self::polynomial::Polynomial;

// KeyPair should maybe also have a `size` field
#[derive(Debug)]
struct KeyPair<'a> {
    pk: PublicKey<'a>,
    sk: Polynomial<'a>,
}

impl fmt::Display for KeyPair<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pk: {}, sk: {}", self.pk, self.sk)
    }
}

impl KeyPair {
    pub fn keygen(pqr: &'a PolynomialQuotientRing) -> Self {
        let sk: Polynomial = Polynomial::gen_binary_poly(pqr);
        let a: Polynomial = Polynomial::gen_uniform_poly(pqr);
        let e: Polynomial = Polynomial::gen_normal_poly(pqr);
        let zero: Polynomial = Polynomial::additive_identity(pqr);
        let b: Polynomial = zero.sub(&a).mul(&sk).sub(&e).modulus();
        let pk = PublicKey { a, b };
        KeyPair { pk, sk }
    }
}
