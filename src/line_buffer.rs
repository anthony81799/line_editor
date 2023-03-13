use unicode_segmentation::UnicodeSegmentation;

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

    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }

    pub fn move_to_end(&mut self) -> usize {
        self.insertion_point = self.buffer.len();
        self.insertion_point
    }

    pub fn get_buffer_length(&self) -> usize {
        self.buffer.len()
    }

    fn get_grapheme_indices(&self) -> Vec<(usize, &str)> {
        UnicodeSegmentation::grapheme_indices(self.buffer.as_str(), true).collect()
    }

    pub fn inc_insertion_point(&mut self) {
        let grapheme_indices = self.get_grapheme_indices();
        for i in 0..grapheme_indices.len() {
            if grapheme_indices[i].0 == self.insertion_point && i < (grapheme_indices.len() - 1) {
                self.insertion_point = grapheme_indices[i + 1].0;
                return;
            }
        }
        self.insertion_point = self.buffer.len();
    }

    pub fn dec_insertion_point(&mut self) {
        let grapheme_indices = self.get_grapheme_indices();
        if self.insertion_point == self.buffer.len() {
            if let Some(index_pair) = grapheme_indices.last() {
                self.insertion_point = index_pair.0;
            } else {
                self.insertion_point = 0;
            }
        } else {
            for i in 0..grapheme_indices.len() {
                if grapheme_indices[i].0 == self.insertion_point && i > 1 {
                    self.insertion_point = grapheme_indices[i - 1].0;
                    return;
                }
            }
            self.insertion_point = 0;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.insertion_point = 0;
    }

    pub fn clear_to_end(&mut self) {
        self.buffer.truncate(self.insertion_point);
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.buffer.insert(idx, ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        let result = self.buffer.pop();
        self.insertion_point = self.buffer.len();
        result
    }

    pub fn remove_char(&mut self, idx: usize) -> char {
        self.buffer.remove(idx)
    }

    pub fn move_word_left(&mut self) -> usize {
        let mut words = self.buffer[..self.insertion_point - 1]
            .split_word_bound_indices()
            .rev();

        loop {
            match words.next() {
                Some((_, word)) if is_word_boundary(word) => {
                    continue;
                }
                Some((index, _)) => {
                    self.insertion_point = index;
                }
                None => {
                    self.insertion_point = 0;
                }
            }
            return self.insertion_point;
        }
    }

    pub fn move_word_right(&mut self) -> usize {
        let mut words = self.buffer[self.insertion_point..].split_word_bound_indices();
        let mut word_found = false;

        loop {
            match words.next() {
                Some((offset, word)) => {
                    if word_found {
                        self.insertion_point += offset;
                    } else {
                        word_found = !is_word_boundary(word);
                        continue;
                    }
                }
                None => {
                    self.insertion_point = self.buffer.len();
                }
            }
            return self.insertion_point;
        }
    }
}

fn is_word_boundary(s: &str) -> bool {
    !s.chars().any(char::is_alphabetic)
}
