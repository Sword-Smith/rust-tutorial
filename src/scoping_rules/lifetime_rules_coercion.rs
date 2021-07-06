// Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, b>` reads: lifetime 'a is at least as long as `b.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn choose_first_no_coercion<'a, 'b>(first: &'a i32, _: &'b i32) -> &'a i32 {
    first
}

fn choose_first_one_lifetime<'a>(first: &'a i32, _: &'a i32) -> &'a i32 {
    first
}

pub fn lifetime_rules_coercion() {
    let first = 2;

    let chosen1: &i32;
    let chosen2: &i32;
    let chosen3: &i32;
    {
        let second = 3;

        println!("The product is {}", multiply(&first, &second));
        chosen1 = choose_first(&first, &second);
        println!("{} is the first,", chosen1);
        chosen2 = choose_first_no_coercion(&first, &second);
        println!("{} is the first,", chosen2);
        chosen3 = choose_first_one_lifetime(&first, &second);
        println!("{} is the first,", chosen3);
    }
    // println!("{} is the first,", chosen1); // Will not work since the chosen1 has the lifetime of `second`.
    // println!("{} is the first,", chosen3); // Disallowed for the same reason
    println!("{} is the first,", chosen2);
}

// With coercion a longer lifetime can be coerced into a shorter one,
// so that it works inside a scope it normally wouldn't work in.
