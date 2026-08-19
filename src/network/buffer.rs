pub struct NetworkBuffer {
    buffer: Vec<u8>,
}

impl NetworkBuffer {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn push_data(&mut self, chunk: &[u8]) {
        self.buffer.extend_from_slice(chunk);
    }

    pub fn pop_frame(&mut self) -> Option<Vec<u8>> {
        /// 헤더 크기 4
        if self.buffer.len() < 4 {
            return None;
        }

        let header_bytes: [u8; 4] = self.buffer[..4].try_into().unwrap();
        let content_length: usize = u32::from_be_bytes(header_bytes).try_into().unwrap();
        let total_frame_length = 4 + content_length;

        if self.buffer.len() < total_frame_length {
            return None;
        }

        let frame = self.buffer.drain(..total_frame_length).collect();
        Some(frame)
    }
}
