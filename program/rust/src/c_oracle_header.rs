#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//we do not use all the variables in oracle.h, so this helps with the warnings
#![allow(unused_variables)]
#![allow(dead_code)]
//All the custom trait imports should go here
use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
//bindings.rs is generated by build.rs to include
//things defined in bindings.h
include!("../bindings.rs");