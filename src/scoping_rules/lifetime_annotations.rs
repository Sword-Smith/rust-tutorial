// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
#[allow(clippy::needless_lifetimes)]
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
// What does this lifetime parameter mean?
#[allow(clippy::extra_unused_lifetimes)]
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    // but `_x` *does* live as long as the function
    // body, but
    let _y: &i32 = &_x; // Works

    // The below line does not work since ???
    // let y: &'a i32 = &_x; // Does not work
    // A short lifetime cannot be coerced into a longer one.

    println!("_x = {}", _x);
}

pub fn lifetime_annotations() {
    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);
    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.
    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
