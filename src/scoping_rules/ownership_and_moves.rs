fn destroy_box(x: Box<i32>) {
    println!("Destroying a box that contains {}", x);

    // `c` is destroyed and its memory is freed
}

pub fn ownership_and_moves() {
    let x = 5u32;

    // *copy* `x` into `y` -- no resources are moved
    // x is an atomic type and thus implements
    // the `Copy` trait
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a *heap* allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // *Move* `a into `b`
    let b = a;

    // The pointer address of `a ` is copied into `b`.
    // Both are now pointers to the same heap allocated
    // data, but `b` now owns it.

    // `a` can no longer access the heap allocated
    // data, because it no longer owns it.

    // println!("a contains {}", a);

    destroy_box(b);

    // println!("b contains {}", b);
    // println!("");
}
