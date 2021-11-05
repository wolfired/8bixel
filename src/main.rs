#![crate_name = "main"]
#![crate_type = "bin"]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    alloc::{alloc_zeroed, Layout},
    array::from_ref,
    mem::{align_of, align_of_val, discriminant, size_of, size_of_val},
    ops::{Add, Index},
    ptr::{
        addr_of, addr_of_mut, null, read_unaligned, slice_from_raw_parts, slice_from_raw_parts_mut,
    },
};

#[derive(Debug)]
struct Sun {
    arr: [u8; 2],
}

enum Son {
    A(i32),
    B(i32),
}

fn main() {
}
