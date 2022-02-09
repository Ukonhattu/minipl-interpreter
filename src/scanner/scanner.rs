pub struct Scanner {
    program: Vec<char>,
    read_index: usize
}

impl Scanner {
    
    pub fn new(program: String) -> Self {
        Self {
            program: program.chars().collect(),
            read_index: 0
        }
    }
    pub fn scan(&self) {
        let left = 0; 
        let right = 0;
        let len = self.program.len();

        while right <= len && left <= right {
            
        }
    }

    fn get_next_character(&mut self) -> Option<&char> {
        let char = self.program.get(self.read_index);
        self.read_index += 1;
        char 

    }

    fn peek_next_character(&self) -> Option<&char> {
        self.program.get(self.read_index + 1)
    }


}