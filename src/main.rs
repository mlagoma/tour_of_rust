#![allow(dead_code)] // this line prevents compiler warnings

mod struct_enums;
mod option;
mod basic_result;
mod result;
mod vectors;
mod ownership;
mod static_variables;
mod struct_lifetimes;
mod text;
mod objects;
mod references;

#[allow(unused_imports)]
use crate::objects::NoiseMaker;

// fn main() {
//  // struct_enums::main();
//  // option::main();
//  // basic_result::main();
//  vectors::main();
// }

fn main() -> Result<(), String> {
    // return result::main();
    // result::main()
    // vectors::main();
    // ownership::main();
    // static_variables::main();
    // struct_lifetimes::main();
    // text::main();
    // let creature = objects::main();
    // // println!("{} goes {}", creature.name, creature.noise);
    // println!("{} goes {}", creature.name, creature.get_sound());
    // print!("{} goes ", creature.name);
    // creature.make_noise();
    // // creature.make_alot_of_noise();
    references::main();
    Ok(())
}
