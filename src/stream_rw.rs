use std::{convert::TryInto, mem::size_of};

const HEADER_LEN: usize = 2 * size_of::<usize>();

pub struct StreamRW {
    stream: Vec<u8>,

    cursor_r: usize,
    cursor_w: usize,

    bytes_skipped_r: usize,
    bytes_skipped_w: usize,

    bytes_read: usize,
    bytes_wrote: usize,
}

impl StreamRW {
    pub fn new(stream: Vec<u8>) -> Self {
        Self {
            stream,
            cursor_r: 0,
            cursor_w: 0,
            bytes_skipped_r: 0,
            bytes_skipped_w: 0,
            bytes_read: 0,
            bytes_wrote: 0,
        }
    }

    pub fn info(&self) {
        println!(
            "cursor_r: {}\ncursor_w: {}\nbytes_skipped_r: {}\nbytes_skipped_w: {}\nbytes_read: {}\nbytes_wrote: {}",
            self.cursor_r,
            self.cursor_w,
            self.bytes_skipped_r,
            self.bytes_skipped_w,
            self.bytes_read,
            self.bytes_wrote,
        );
    }

    /// payload: [ ID | Content Len | Content ]
    pub fn read(&mut self, payload: &[u8]) -> usize {
        let mut cursor_r = self.cursor_r % self.stream.len();
        let gap_len = self.stream.len() - cursor_r;

        if HEADER_LEN >= gap_len {
            self.cursor_r += gap_len;
            self.bytes_skipped_r += gap_len;
        }

        if self.cursor_r >= self.cursor_w {
            return 0;
        }

        let remain_len = self.cursor_w - self.cursor_r;

        if HEADER_LEN > remain_len {
            return 0;
        }

        cursor_r = self.cursor_r % self.stream.len();

        let id = usize::from_le_bytes(
            self.stream[cursor_r..{
                cursor_r += size_of::<usize>();
                cursor_r
            }]
                .try_into()
                .unwrap(),
        );
        let content_len = usize::from_le_bytes(
            self.stream[cursor_r..{
                cursor_r += size_of::<usize>();
                cursor_r
            }]
                .try_into()
                .unwrap(),
        );

        let payload_len = HEADER_LEN + content_len;

        if payload_len > remain_len {
            return 0;
        }

        if 0 == id {
            self.bytes_skipped_r += payload_len;
            self.cursor_r += payload_len;
            self.read(payload)
        } else {
            // TODO
            self.bytes_read += payload_len;
            self.cursor_r += payload_len;
            payload_len
        }
    }

    /// payload: [ ID | Content Len | Content ]
    pub fn write(&mut self, payload: &[u8]) {
        let mut cursor_w = self.cursor_w % self.stream.len();
        let gap_len = self.stream.len() - cursor_w;

        if HEADER_LEN > gap_len {
            self.cursor_w += gap_len;
            self.bytes_skipped_w += gap_len;
        } else if payload.len() > gap_len {
            for (i, v) in [0_usize.to_le_bytes(), (gap_len - HEADER_LEN).to_le_bytes()]
                .concat()
                .as_slice()
                .iter()
                .enumerate()
            {
                self.stream[cursor_w + i] = *v
            }

            self.cursor_w += gap_len;
            self.bytes_skipped_w += gap_len;
        }

        cursor_w = self.cursor_w % self.stream.len();

        for (i, v) in payload.iter().enumerate() {
            self.stream[cursor_w + i] = *v;
        }

        self.bytes_wrote += payload.len();
        self.cursor_w += payload.len();
    }
}
