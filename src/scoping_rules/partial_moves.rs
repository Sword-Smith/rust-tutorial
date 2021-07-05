pub fn partial_moves() {
    // When destructuring a single variable, both
    // by-move, and by-reference, pattern bindings can
    // be used at the same time. Doing the will result in a partial move of the variable

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name is moved out of person, but `age` is referenced, meaning that it is borrowed without transferring any ownership
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person`
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
