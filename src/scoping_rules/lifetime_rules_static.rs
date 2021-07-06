// A variable with 'static lifetime is stored in the read-only
// memory of the binary.

// A variable with static lifetime can be made in two ways:
// 1) By declaring a constant with the static declaration
// 2) Making a string literal which has type &'static str
static NUM: i32 = 18;

#[allow(clippy::needless_lifetimes)]
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn lifetime_rules_static() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
        // When this value goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);

        let b: &i32 = coerced_static;
    }

    println!("NUM: {} stays accessible!", NUM);
}
