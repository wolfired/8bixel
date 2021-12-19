use std::{convert::TryInto, mem::size_of};

const HEADER_LEN: usize = 2 * size_of::<usize>();

pub struct RingBuffer {
    stream: Vec<u8>,

    absolute_cursor_r: usize,
    absolute_cursor_w: usize,

    bytes_skipped_r: usize,
    bytes_skipped_w: usize,

    bytes_read: usize,
    bytes_wrote: usize,
}

impl RingBuffer {
    pub fn new(stream: Vec<u8>) -> Self {
        Self {
            stream,
            absolute_cursor_r: 0,
            absolute_cursor_w: 0,
            bytes_skipped_r: 0,
            bytes_skipped_w: 0,
            bytes_read: 0,
            bytes_wrote: 0,
        }
    }

    pub fn info(&self) {
        println!(
            r#"
absolute_cursor_r: {}
  bytes_skipped_r: {}
       bytes_read: {}
absolute_cursor_w: {}
  bytes_skipped_w: {}
      bytes_wrote: {}
            "#,
            self.absolute_cursor_r,
            self.bytes_skipped_r,
            self.bytes_read,
            self.absolute_cursor_w,
            self.bytes_skipped_w,
            self.bytes_wrote,
        );
    }

    /// Payload: [ Len | ID | Content ]
    pub fn read(&mut self, payload: &[u8]) -> usize {
        let mut relative_cursor_r = self.absolute_cursor_r % self.stream.len();
        let gap_len = self.stream.len() - relative_cursor_r;

        if size_of::<usize>() >= gap_len {
            self.absolute_cursor_r += gap_len;
            self.bytes_skipped_r += gap_len;
        }

        if self.absolute_cursor_r >= self.absolute_cursor_w {
            return 0;
        }

        let readable_len = self.absolute_cursor_w - self.absolute_cursor_r;

        if size_of::<usize>() > readable_len {
            return 0;
        }

        relative_cursor_r = self.absolute_cursor_r % self.stream.len();

        let len_of_rawbytes = usize::from_le_bytes(
            self.stream[relative_cursor_r..{
                relative_cursor_r += size_of::<usize>();
                relative_cursor_r
            }]
                .try_into()
                .unwrap(),
        );

        let payload_len = size_of::<usize>() + len_of_rawbytes;

        if payload_len > readable_len {
            return 0;
        }

        self.absolute_cursor_r += payload_len;
        self.bytes_read += len_of_rawbytes;
        len_of_rawbytes
    }

    /// Payload: [ Len | ID | Content ]
    pub fn write(&mut self, payload: &[u8]) {
        let mut relative_cursor_w = self.absolute_cursor_w % self.stream.len();
        let gap_len = self.stream.len() - relative_cursor_w;

        if size_of::<usize>() >= gap_len {
            self.absolute_cursor_w += gap_len;
            self.bytes_skipped_w += gap_len;
        } else if payload.len() > gap_len - size_of::<usize>() {
            for (i, v) in (gap_len - size_of::<usize>())
                .to_le_bytes()
                .into_iter()
                .enumerate()
            {
                self.stream[relative_cursor_w + i] = v
            }

            self.absolute_cursor_w += gap_len;
            self.bytes_skipped_w += gap_len;
        }

        relative_cursor_w = self.absolute_cursor_w % self.stream.len();

        for (i, v) in payload.len().to_le_bytes().iter().enumerate() {
            self.stream[relative_cursor_w + i] = *v
        }

        relative_cursor_w += size_of::<usize>();

        for (i, v) in payload.iter().enumerate() {
            self.stream[relative_cursor_w + i] = *v;
        }

        self.absolute_cursor_w += size_of::<usize>() + payload.len();
        self.bytes_wrote += payload.len();
    }
}
