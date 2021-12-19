#![crate_name = "main"]
#![crate_type = "bin"]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod ring_buffer;
use ring_buffer::RingBuffer;

use std::{
    alloc::{alloc_zeroed, Layout},
    array::from_ref,
    mem::{align_of, align_of_val, discriminant, size_of, size_of_val},
    ops::{Add, Index},
    ptr::{
        addr_of, addr_of_mut, null, read_unaligned, slice_from_raw_parts, slice_from_raw_parts_mut,
    },
};

fn main() {
    let mut buf = RingBuffer::new(vec![0; 32]);
    for i in 0..4 {
        buf.write(&[0_u8; 9][..]);
        buf.read(&[0_u8; 9][..]);
    }
    buf.info();
}
