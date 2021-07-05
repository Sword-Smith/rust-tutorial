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
