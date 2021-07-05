struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn aliasing() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );

    // `point` cannot be mutably borrowed because it is currently immutably borrowed.
    // let mutable_borrow = &mut point;

    // The borrowed values are used again here
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Here, the immutable borrows are garbage collected, since they are not used later in the program.

    // The immutable references are no longer used for the rest of the code, so it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // point cannot be borrowed as immutable because it's currently borrowed as mutable
    // let y = &point.y;

    // point cannot be printed, since the `println!` macro takes an immutable borrow.
    // println!("Point Z coordinate is {}", point.z);

    // Mutable references can, however, be passed as immutable to `println!`. So it *is* possible to print even though a mutable reference exists.
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
