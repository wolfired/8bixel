#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::alloc::{alloc, Layout};
use std::mem::{align_of, size_of};

extern "C" {
    fn mylog(v: isize);
}

pub struct Game {
    wid: isize,
    hei: isize,
    income_reader: IOHelper,
    outgo_writer: IOHelper,
}

impl Game {
    pub const fn new(wid: isize, hei: isize, io_buf_cap: usize) -> Self {


        let income_reader = IOHelper::new(income_buf_ptr, income_cursor_w_ptr, io_buf_cap);
        let outgo_writer = IOHelper::new(outgo_buf_ptr, outgo_cursor_w_ptr, io_buf_cap);

        Self {
            wid,
            hei,
            income_reader,
            outgo_writer,
        }
    }

    pub fn get_args_usize_ptr(&self) -> usize {
        &(self.args_usize) as *const [usize; PTR_ARGS_COUNT] as *const u8 as usize
    }

    fn get_canvas_buf_ptr(&self) -> usize {
        self.args_usize[0]
    }

    fn get_income_buf_ptr(&self) -> usize {
        self.args_usize[1]
    }

    fn get_outgo_buf_ptr(&self) -> usize {
        self.args_usize[2]
    }

    fn set_pixel_color(&self, x: isize, y: isize, r: u8, g: u8, b: u8, a: u8) {
        let offset = ((x + y * self.wid) * 4) as usize;

        let canvas_buf_ptr: usize = self.get_canvas_buf_ptr();

        unsafe {
            *((canvas_buf_ptr + offset + 0) as *mut u8) = r;
            *((canvas_buf_ptr + offset + 1) as *mut u8) = g;
            *((canvas_buf_ptr + offset + 2) as *mut u8) = b;
            *((canvas_buf_ptr + offset + 3) as *mut u8) = a;
        }
    }

    pub fn boot(&self) -> usize {
        self.set_pixel_color(self.x, self.y, 255, 0, 0, 255);

        0
    }

    pub fn ever(&mut self) -> usize {
        self.income_reader.read();

        self.recv();

        // logic

        self.send();

        self.render();

        0
    }

    pub fn halt(&self) -> usize {
        0
    }

    pub fn send(&self) {}

    pub fn recv(&self) {}

    pub fn render(&mut self) {
        // let bytes = include_bytes!("../art/fire.rgba");

        // for x in 0..96_usize {
        //     for y in 0..128_usize {
        //         self.set_pixel_color(
        //             x as isize,
        //             y as isize,
        //             bytes[(x + y * 96) * 4 + 0],
        //             bytes[(x + y * 96) * 4 + 1],
        //             bytes[(x + y * 96) * 4 + 2],
        //             bytes[(x + y * 96) * 4 + 3],
        //         );
        //     }
        // }
    }
}
