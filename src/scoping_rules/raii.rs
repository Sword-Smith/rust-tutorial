fn create_box() {
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

pub fn scoping_rules() {
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Many boxes can be created. There is no need to
    // manually free memory.
    for _ in 0..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
