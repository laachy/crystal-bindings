#![no_std]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

mod types;

/* This is the root of all generated bindings */
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));