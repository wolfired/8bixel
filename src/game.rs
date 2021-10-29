#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::alloc::{alloc, Layout};
use std::mem::{align_of, size_of};

extern "C" {
    fn mylog(v: isize);
}

const PTR_ARGS_COUNT: usize = 6;
static mut N: Option<Box<usize>> = None;
pub static mut G: Option<Game> = None;

struct IOHelper {
    buf_ptr: usize,
    buf_cap: usize,

    cursor_r: usize,
    cursor_w_ptr: usize,

    skipped_count_r: usize,
    skipped_count_w: usize,

    total_count_r: usize,
    total_count_w: usize,
}

impl IOHelper {
    fn new(buf_ptr: usize, cursor_w_ptr: usize, buf_cap: usize) -> Self {
        Self {
            buf_ptr,
            buf_cap,
            cursor_r: 0,
            cursor_w_ptr,
            skipped_count_r: 0,
            skipped_count_w: 0,
            total_count_r: 0,
            total_count_w: 0,
        }
    }

    fn read(&mut self) {
        let cursor_w;

        unsafe {
            cursor_w = *(self.cursor_w_ptr as *const usize);
        }

        let mut cursor_r = self.cursor_r % self.buf_cap;
        let gap_r = self.buf_cap - self.cursor_r;

        if 8 > gap_r {
            self.cursor_r += gap_r;
            self.skipped_count_r += gap_r;
        }

        let len = cursor_w - self.cursor_r;

        if 8 > len {
            return;
        }

        cursor_r = self.cursor_r % self.buf_cap;

        unsafe {
            let buf = self.buf_ptr + cursor_r;

            let id = u32::from_le(*(buf as *const u32).add(0));
            let count = u32::from_le(*(buf as *const u32).add(1));

            if count as usize > len - 8 {
                return;
            }

            if 0 == id {
                self.skipped_count_r += count as usize;
            } else {
                if let Some(ref n) = N {
                    mylog(**n as isize);
                }

                if 10 == id {
                    let key = u32::from_le(*(buf as *const u32).add(2));
                    if let Some(ref mut g) = G {
                        match key {
                            65 => g.x -= 1,
                            68 => g.x += 1,
                            83 => g.y += 1,
                            87 => g.y -= 1,
                            _ => {}
                        }
                        g.set_pixel_color(g.x, g.y, 255, 0, 0, 255);
                    }
                }

                self.total_count_r += count as usize;
            }

            self.cursor_r += count as usize + 8;
        }
    }

    fn write(&mut self) {}
}

pub struct Game {
    wid: isize,
    hei: isize,
    x: isize,
    y: isize,
    args_usize: [usize; PTR_ARGS_COUNT], // [canvas, income, outgo, income_cursor_w, outgo_cursor_w]
    income_reader: IOHelper,
    outgo_writer: IOHelper,
}

impl Game {
    pub fn new(wid: isize, hei: isize, io_buf_cap: usize) -> Self {
        let canvas_buf_ptr;
        let income_buf_ptr;
        let outgo_buf_ptr;
        let income_cursor_w_ptr;
        let outgo_cursor_w_ptr;

        unsafe {
            let canvas_buf_layout =
                Layout::from_size_align_unchecked((wid * hei * 4) as usize, align_of::<u8>());
            canvas_buf_ptr = alloc(canvas_buf_layout) as usize;

            let income_buf_layout = Layout::from_size_align_unchecked(io_buf_cap, align_of::<u8>());
            income_buf_ptr = alloc(income_buf_layout) as usize;

            let outgo_buf_layout = Layout::from_size_align_unchecked(io_buf_cap, align_of::<u8>());
            outgo_buf_ptr = alloc(outgo_buf_layout) as usize;

            let income_cursor_w_layout =
                Layout::from_size_align_unchecked(size_of::<usize>(), align_of::<usize>());
            income_cursor_w_ptr = alloc(income_cursor_w_layout) as usize;

            let outgo_cursor_w_layout =
                Layout::from_size_align_unchecked(size_of::<usize>(), align_of::<usize>());
            outgo_cursor_w_ptr = alloc(outgo_cursor_w_layout) as usize;
        }

        let income_reader = IOHelper::new(income_buf_ptr, income_cursor_w_ptr, io_buf_cap);
        let outgo_writer = IOHelper::new(outgo_buf_ptr, outgo_cursor_w_ptr, io_buf_cap);
        let mut N_ptr = 0;
        unsafe {
            N = Some(Box::new(11235));
            if let Some(ref n) = N {
                N_ptr = &**n as *const usize as usize;
            }
        }

        Self {
            wid,
            hei,
            x: 150,
            y: 150,
            args_usize: [
                canvas_buf_ptr,
                income_buf_ptr,
                outgo_buf_ptr,
                income_cursor_w_ptr,
                outgo_cursor_w_ptr,
                N_ptr,
            ],
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
        // let bytes = include_bytes!("../fire.rgba");

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

    pub fn render(&self) {}
}
