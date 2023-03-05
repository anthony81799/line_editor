pub struct LineBuffer {
    buffer: String,
    insertion_point: usize,
}

impl LineBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            insertion_point: 0,
        }
    }

    pub fn get_insertion_point(&self) -> usize {
        self.insertion_point
    }

    pub fn set_insertion_point(&mut self, pos: usize) {
        self.insertion_point = pos;
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    pub fn get_buffer_length(&self) -> usize {
        self.buffer.len()
    }

    pub fn slice_buffer(&self, pos: usize) -> &str {
        &self.buffer[pos..]
    }

    pub fn inc_insertion_point(&mut self) {
        self.insertion_point += 1;
    }

    pub fn dec_insertion_point(&mut self) {
        self.insertion_point -= 1;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.buffer.insert(idx, ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.buffer.pop()
    }

    pub fn remove_char(&mut self, idx: usize) -> char {
        self.buffer.remove(idx)
    }

    pub fn move_word_left(&mut self) -> usize {
        match self
            .buffer
            .rmatch_indices(&[' ', '\t'][..])
            .find(|(index, _)| index < &(self.insertion_point - 1))
        {
            Some((index, _)) => {
                self.insertion_point = index + 1;
            }
            None => {
                self.insertion_point = 0;
            }
        }
        self.insertion_point
    }

    pub fn move_word_right(&mut self) -> usize {
        match self
            .buffer
            .match_indices(&[' ', '\t'][..])
            .find(|(index, _)| index > &(self.insertion_point))
        {
            Some((index, _)) => {
                self.insertion_point = index + 1;
            }
            None => {
                self.insertion_point = self.buffer.len();
            }
        }
        self.insertion_point
    }
}