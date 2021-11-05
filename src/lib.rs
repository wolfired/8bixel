#![crate_name = "8bixel"]
#![crate_type = "cdylib"]
#![allow(dead_code)]
#![allow(unused_variables)]

mod game;
mod stream_rw;
use game;

#[no_mangle]
pub unsafe fn init(wid: isize, hei: isize, io_buf_cap: usize) -> usize {
    G = Some(Game::new(wid, hei, io_buf_cap));
    if let Some(ref g) = G {
        g.get_args_usize_ptr()
    } else {
        1
    }
}

#[no_mangle]
pub unsafe fn boot() -> usize {
    if let Some(ref g) = G {
        g.boot()
    } else {
        1
    }
}

#[no_mangle]
pub unsafe fn ever() -> usize {
    if let Some(ref mut g) = G {
        g.ever()
    } else {
        1
    }
}

#[no_mangle]
pub unsafe fn halt(ptr: usize) -> usize {
    if let Some(ref g) = G {
        g.halt()
    } else {
        1
    }
}
