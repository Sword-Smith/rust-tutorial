use anyhow::{bail, Result};

mod conversion;
mod custom_types;
mod expressions;
mod flow_of_control;
mod formatted_print;
mod functions;
mod hello_world;
mod primitives;
mod scoping_rules;
mod types;

pub fn my_library_function() -> Result<()> {
    println!("\n\n*****Running my_private_function*****");
    my_private_function(42)?;
    println!("\n\n*****Running hello_world*****");
    hello_world::main::hello_world();
    println!("\n\n*****Running formatted_print*****");
    formatted_print::main::formatted_print();
    println!("\n\n*****Running primitives*****");
    println!("\n**primitives**");
    primitives::main::primitives();
    println!("\n**literals_and_operators**");
    primitives::literals_and_operators::literals_and_operators();
    println!("\n**tuples**");
    primitives::tuples::tuples();
    println!("\n**arrays_and_slices**");
    primitives::arrays_and_slices::arrays_and_slices();
    println!("\n\n*****Running custom_types*****");
    custom_types::structures::structures();
    println!("\n**enums**");
    custom_types::enums::enums();
    println!("\n**linked_list**");
    custom_types::linked_list::linked_list();
    println!("\n**constants**");
    custom_types::constants::constants();
    println!("\n\n*****Running types*****");
    types::casting::casting();
    types::literals::literals();
    types::inference::inference();
    types::aliasing::aliasing();
    println!("\n\n*****Running conversion*****");
    conversion::from_and_into::from_and_into();
    conversion::tryfrom_and_tryinto::tryfrom_and_tryinto();
    conversion::to_and_from_strings::to_and_from_strings();
    println!("\n\n*****Running expressions*****");
    expressions::expressions();
    println!("\n\n*****Running flow_of_controls*****");
    flow_of_control::if_else::if_else();
    flow_of_control::loop_example::loop_example();
    flow_of_control::fizz_buzz::fizz_buzz();
    flow_of_control::for_and_range::for_and_range();
    flow_of_control::match_example::match_example();
    println!("\n\n*****Running functions*****");
    functions::methods::methods();
    functions::closures::closures();
    println!("\n\n*****Scoping rules*****");
    scoping_rules::raii::scoping_rules();
    scoping_rules::to_drop::to_drop();
    scoping_rules::ownership_and_moves::ownership_and_moves();
    scoping_rules::mutability::mutability();
    scoping_rules::partial_moves::partial_moves();
    scoping_rules::borrowing::borrowing();
    scoping_rules::borrowing_mutability::borrowing_mutability();
    scoping_rules::aliasing::aliasing();
    scoping_rules::the_ref_pattern::the_ref_pattern();
    scoping_rules::lifetimes::lifetimes();
    scoping_rules::lifetime_annotations::lifetime_annotations();
    scoping_rules::lifetime_rules_functions::lifetime_rules_functions();
    scoping_rules::lifetime_rules_methods::lifetime_rules_methods();
    scoping_rules::lifetime_rules_structs::lifetime_rules_structs();
    scoping_rules::lifetime_rules_traits::lifetime_rules_traits();
    scoping_rules::lifetime_rules_bounds::lifetime_rules_bounds();
    scoping_rules::lifetime_rules_coercion::lifetime_rules_coercion();
    scoping_rules::lifetime_rules_static::lifetime_rules_static();

    Ok(())
}

fn my_private_function(n: i32) -> Result<()> {
    if n > 1000 {
        bail!("`n` cannot be larger than 1000!")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert!(my_private_function(42).is_ok());
        assert!(my_private_function(9001).is_err());
    }
}
