pub struct StackState {
    stack: Vec<bool>,
}

impl StackState {
    pub fn new() -> StackState {
        StackState {
            stack: vec![]
        }
    }

    // def push(bit)
    //   bit_value = bit ? 1 : 0
    //   @stack = (@stack << 1) | bit_value
    // 
    //   bit
    // end
    pub fn push(&mut self, state: bool) {
        self.stack.push(state);
    }

    // def pop
    //   bit_value = @stack & 1
    //   @stack  >>= 1
    // 
    //   bit_value == 1
    // end
    pub fn pop(&mut self) -> bool {
        if self.stack.is_empty() {
            false
        } else {
            self.stack.pop().unwrap()
        }
    }

    // def lexpop
    //   @stack = ((@stack >> 1) | (@stack & 1))
    //   @stack[0] == 1
    // end
    pub fn lexpop(&mut self) -> bool {
        if self.stack.is_empty() { return false; }

        let last_bit = self.stack.pop().unwrap() == true;
        if last_bit == true {
            // overwrite last to true
            if self.stack.is_empty() {
                self.stack.push(true);
            } else {
                let last_bit = self.stack.last_mut().unwrap();
                *last_bit = true;
            }
        }

        if self.stack.is_empty() { return false; } else { return *self.stack.last().unwrap(); }
    }

    // def active?
    //   @stack[0] == 1
    // end
    pub fn is_active(&self) -> bool {
        !self.stack.is_empty() && ( self.stack.last().unwrap() == &true )
    }
}
