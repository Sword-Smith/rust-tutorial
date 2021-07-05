// When doing pattern matching or destructuring via the let binding, the ref keyword can be used to take references to the fields of a struct/tuple. The example below shows a few instances where this can be useful

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

pub fn the_ref_pattern() {
    let c: char = 'Q';

    // A ref borrow on the left side of an assignment is equivalent to an `&` borrow on the right side
    #[allow(clippy::toplevel_ref_arg)]
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 == ref_c2 evaluates to {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        // This shows the destructuring of a struct. I am used to doing it fow tuples,
        // but it is possible to do for structs too.
        // By adding a `ref` infront of the variable name, an immutable reference
        // to the value is created.
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // Return a copy of the `x` field of point
        // The value is copied since *ref_to_x has type i32 which is a
        // atomic type.
        *ref_to_x
    };

    // Make a mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // Mutate the `y` field of `mutable_point` through a mutable reference
        *mut_ref_to_y = 1;
    }

    println!("point is ({},{})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // A mutable tuple that includes a pointer
    let mut mutable_tuple: (Box<u32>, u32) = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    // {
    //     // Destructure `mutable_tuple` to change the value of `last`.
    //     let (_, ref mut last) = mutable_tuple;
    //     *last = 2u32;
    // }
    // println!("tuple is {:?}", mutable_tuple);
    println!("tuple is {:?}", mutable_tuple);
}
