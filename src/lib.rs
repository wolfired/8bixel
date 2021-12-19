#![crate_name = "8bixel"]
#![crate_type = "cdylib"]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[no_mangle]
pub unsafe fn init() -> usize {
    1
}

#[no_mangle]
pub unsafe fn boot() -> usize {
    0
}

#[no_mangle]
pub unsafe fn ever() -> usize {
    0
}

#[no_mangle]
pub unsafe fn halt() -> usize {
    0
}
