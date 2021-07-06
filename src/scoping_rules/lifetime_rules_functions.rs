#![allow(clippy::many_single_char_names)]
// Ignoring elision, the following rules apply for functions

// any reference must have an annotated lifetime
// Any reference being returned must have the same lifetime as an input or be static.

// One reference value with lifetime `a as input. This lifetime indicates that the
// value that the input borrows must live as long as this function.
#[allow(clippy::needless_lifetimes)]
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references can also have lifetimes
#[allow(clippy::needless_lifetimes)]
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes.
#[allow(clippy::needless_lifetimes)]
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Function can return references that have been passed in
// But the correct lifetime must be returned
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// This is, however, illegal as it creates a reference to data
// that is immediately dropped
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

pub fn lifetime_rules_functions() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    // The function pass_x returns a value that
    // has the shortest lifetime of the input values
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    let e: &i32;
    let a: &i32;
    let b: &i32;
    {
        let c = 42;
        let d = 43;
        a = &c;
        b = &d;
        e = pass_x(a, b);
        print_one(&e);
    }

    // &e cannot be used here since the lifetime of it is defined in
    // the function from which it is returned, and it is the shortest
    // lifetime of the pair (a, b), and they both refer to data that
    // is dead here.
    // Generally, the lifetime parameters of functions are defined
    // in terms of its input parameters.
}
