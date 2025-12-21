pub struct Stack {
    data: Vec<CTContext>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.data.push(ctx);
    }

    pub fn push_slice(&mut self, slice: &[u8]) {
        self.data
    }

    pub fn pop(&mut self) -> u8 {
        let _ = self.data.pop();
    }
}

impl Index<usize> for Stack {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.data[i]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(mut self, i: usize) -> &mut u8 {
        &mut self.data[i]
    }
}
