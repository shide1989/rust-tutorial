#![allow(dead_code, unused_variables, unused_imports)]
// The allow is used to avoid warnings on this tutorial project

// use crate::modules::functions::*;
// use crate::modules::ranges::*;
// use crate::modules::ownership::*;
// use crate::modules::strutures::*;
// use crate::modules::struct_example::*;
// use crate::modules::methods::*;
// use crate::modules::enums::*;
// use crate::modules::matches::*;
// use crate::modules::control_flow::*;
// use crate::modules::ranges::test;
// use crate::modules::collections;
// use crate::modules::strings;
// use crate::modules::functions;
// use crate::modules::ownership;
// use crate::modules::format;
use crate::modules::errors;
use crate::modules::generics;
use crate::modules::hashmaps;
use crate::modules::traits;

mod modules;

fn main() {
    println!("-- Hello tutorials ! --");
    traits::test_traits();
}
