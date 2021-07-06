#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// This is an example of how the static lifetime
// parameter can be used.
#[derive(Debug)]
struct BorrowedStaticLifetime {
    x: &'static i32,
}

impl<'a> Default for Borrowed<'a> {
    // I guess the lifetime of the value 10 is static?
    fn default() -> Self {
        Self { x: &10 }
    }
}

impl Default for BorrowedStaticLifetime {
    // I guess the lifetime of the value 10 is static?
    fn default() -> Self {
        Self { x: &15 }
    }
}

pub fn lifetime_rules_traits() {
    let a: Borrowed = Default::default();
    println!("lifetime_rules_traits: a is {:?}", a);

    let b: BorrowedStaticLifetime = BorrowedStaticLifetime::default();
    println!("lifetime_rules_traits: b is {:?}", b);

    // Will not work since `i` does not have static lifetime
    // let i = 14;
    // let bsl1 = BorrowedStaticLifetime { x: &i };
    // println!("lifetime_rules_traits: bsl1 is {:?}", bsl1);

    // Will work since the referenced x value *does* have static lifetime
    let bsl2 = BorrowedStaticLifetime { x: &16 };
    println!("lifetime_rules_traits: bsl2 is {:?}", bsl2);
}
