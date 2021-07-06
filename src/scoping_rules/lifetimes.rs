// Lifetimes in Rust don't just hold data, they also own resources.
// E.g. "Box<T>" owns memory on the heap. Rust enforces RAII which
// is "Resource Acquisition is Initialization". When an object goes
// out of scope its destructor is called, and its owner resources
// are freed. This protects against resource leak bugs.
//
// We cannot return a reference to something defined inside the
// function, since its lifetime is only that of the function body.
// So all return values that are references must have the same lifetime as that of one of the parameteres. The alternative is to return an owned type.
//
// When creating structs that reference values, we would usually specify that our struct cannot outlive the value that it references.
//
// The compiler statically guarantees (via its borrow checker) that reference values *always* point to valid objects.
// I.e., while references to an object exists, the object cannot be destroyed.
//
// Data can be immutably borrowed any number of times, but while immutably borrowed, the original data can't be mutably
// borrowed. Only one mutable borrow is allowed at a time. The original data can be borrowed again only after the mutable references has been used for the last time.
//
// Lifetime annotations are used to help the compiler (borrow checker), the don't actually change lifetimes.
//
// What happens inside the angle brackets `<'a>` is the *declaration of lifetimes.
//
// The lifetime of the returned value always has to be tied with the lifetime of the parameters, since any value created in the function cannot be returned as a reference as they are all invalidated by the end of the function.
//
// When implementing methods with lifetimes, the method themselves will (usually?) not need to declare lifetimes, as they will already be in scope from the "impl." struct.

// A lifetime is a construct that the borrow checker uses to ensure that all references are valid, and that none are dangling.

// A variable's lifetime begins when it is created, and ends when it is destroyed.

//

// // Lifetimes are annotated below with lines denoting the creation
// // and destruction of each variable.
// // `i` has the longest lifetime because its scope entirely encloses
// // both `borrow1` and `borrow2`. The duration of `borrow1` compared
// // to `borrow2` is irrelevant since they are disjoint.
#[derive(Debug)]
struct DropMe {}

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("DropMe instance dropped");
    }
}

pub fn lifetimes() {
    let i = 3;
    let dm = DropMe {};

    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
        println!("drop me used: {:?}", dm);
        println!("PMD0");
    }
    println!("PMD1");
    {
        println!("PMD2");
    }
    // Lifetime of `i` ends here
    // So does the lifetime of `dm`.
}
