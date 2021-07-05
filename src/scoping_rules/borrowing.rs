// Borrowing means to pass objects by reference

// The compiler statically guarantees via its borrow checker that references always point to valid objects

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroing box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is {}", borrowed_i32);
}

pub fn borrowing() {
    // Create a boxed and a stacked i32
    let boxed_i32 = Box::new(5i32);
    let stacked_i32 = 6i32;

    // Borrow the contents of the box. Ownership is not taken, so the contents can be borrowed again.println
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // This is not legal as it would move ownership
        // of `boxed_i32` and destroy the inner value which is borrowed later in the scope.
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    // Here, boxed_i32 can be freed as it is no longer referenced from anywhere.
    eat_box_i32(boxed_i32);
}
