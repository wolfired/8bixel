#![crate_name = "main"]
#![crate_type = "bin"]
#![allow(unused_imports)]
#![feature(array_map)]
#![feature(array_zip)]
#![feature(array_methods)]

use std::{
    alloc::{alloc_zeroed, Layout},
    array::from_ref,
    mem::{align_of, align_of_val, size_of, size_of_val},
    ops::Add,
    ptr::{
        addr_of, addr_of_mut, null, read_unaligned, slice_from_raw_parts, slice_from_raw_parts_mut,
    },
};

fn main() {
}
