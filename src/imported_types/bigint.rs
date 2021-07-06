use num_bigint::BigInt;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::str::FromStr;

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Hash)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PrimeField {
    pub q: BigInt,
}

#[cfg_attr(
    feature = "serialization-serde",
    derive(Serialize, Deserialize, Serializer)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PrimeFieldElement<'a> {
    pub value: BigInt,
    pub field: &'a PrimeField,
}

pub fn bigint() {
    let q = BigInt::from_str("-201545000000000123456789123456789123456789123456789").unwrap();
    let field = PrimeField { q };

    let j: Vec<u8> = bincode::serialize(&field).unwrap();
    println!("{:?}", j);
    // let a: Trade = serde_json::from_str(&j).unwrap();
    let a: PrimeField = bincode::deserialize_from(&j[..]).unwrap();
    println!("{:?}", a);
    assert_eq!(field, a);

    let pfe = PrimeFieldElement {
        value: BigInt::from_str("2015450000000001234567891234567891234567891234567892").unwrap(),
        field: &field,
    };
    let pfe_j = bincode::serialize(&pfe).unwrap();
    println!("{:?}", pfe_j);
    let pfe_a: PrimeField = bincode::deserialize_from(&j[..]).unwrap();
    assert_eq!(field, pfe_a);

    // let j = toml::to_string(&trade).unwrap();
    // println!("{}", j);
    // let b: Trade = toml::from_str(&j).unwrap();
    // assert_eq!(trade, b);
}
