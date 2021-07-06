#![allow(clippy::many_single_char_names)]
use std::fmt::Debug;

// Just like generic types can be bounded, lifetimes can use bounds as well.
// This is specified using:
// 1) `T: 'a` All references in T must outlive lifetime 'a.
// 2) `T: Trait + 'a`: Type `T` must implement trait `Trait` and all references
//    in `T` must outlive 'a.

// Any references in `T` must outlive `'a`. And the lifetime of `Ref` may
// not exceed `'a`.
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

#[derive(Debug)]
struct OtherRef<'a> {
    x: &'a i128,
    y: &'a i128,
}

fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    // This means: Type `T` must implement trait `Debug` and all references in `T`
    // must outlive `'a`. And 'a must outlive this function.
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

pub fn lifetime_rules_bounds() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

    let a = 7i128;
    let b = 42i128;
    let other_ref = OtherRef { x: &a, y: &b };
    let new_t = Ref(&other_ref);
    print(new_t);

    let other_ref2: OtherRef;
    let new_t2: Ref<OtherRef>;
    {
        let c = 53i128;
        let d = 58i128;
        other_ref2 = OtherRef { x: &c, y: &d };
        new_t2 = Ref(&other_ref2);
        print_ref(&new_t2);
    }
    // print_ref(&new_t2);
}
