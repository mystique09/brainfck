#[derive(Debug)]
pub struct BrainFuck {
    pub data: Vec<u32>,
    pub src: Vec<char>,
    pub index: usize,
    pub mem_pos: usize,
    pub check_point: usize,
    pub result: String,
}

impl BrainFuck {
    pub fn new(src: String) -> Self {
        Self {
            data: [0; 2].to_vec(),
            src: src.chars().collect::<Vec<char>>(),
            index: 0,
            mem_pos: 0,
            check_point: 0,
            result: String::new(),
        }
    }

    pub fn run(&mut self, token: &char) {
        match token {
            '+' => {
                if (self.data.len()) <= self.mem_pos {
                    self.data.push(0);
                }
                self.data[self.mem_pos] += 1;
            }
            '-' => {
                self.data[self.mem_pos] -= 1;
            }
            '>' => {
                self.mem_pos += 1;
            }
            '<' => {
                if self.mem_pos > 0 {
                    self.mem_pos -= 1;
                }
            }
            '.' => {
                let ch = char::from_u32(self.data[self.mem_pos]).unwrap_or('\n');
                self.result.push(ch);
            }
            ',' => (),
            '[' => {
                self.check_point = self.index;
            }
            ']' => {
                if self.data[self.mem_pos] > 0 {
                    self.index = self.check_point;
                }
            }
            '\n' | '\r' | '\t' => (),
            _ => panic!(
                "ERROR: Invalid character '{}' at position {}",
                self.src[self.index], self.index
            ),
        }
    }

    pub fn exec(&mut self) {
        while self.index < self.src.len() {
            let token: char = self.src[self.index];

            self.run(&token);
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_new_instance() {
        let mut bf = BrainFuck::new(">++++++++<.".to_string());

        bf.exec();
        assert_eq!(bf.data, [0, 8].to_vec());
        assert!(!bf.src.is_empty());
    }

    #[test]
    fn test_output_data() {
        let mut bf = BrainFuck::new(">++++++++<.".to_string());
        bf.exec();

        assert_eq!(bf.data, [0, 8].to_vec());
    }

    #[test]
    fn test_loop() {
        let mut bf = BrainFuck::new(">++++++++[<+++++++++>-]+.".to_string());
        bf.exec();

        assert_eq!(bf.data, [72, 1].to_vec());
    }

    #[test]
    fn test_loop_2() {
        // --output: [101, 0] | [101, 0]
        let mut bf = BrainFuck::new(">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.".to_string());
        bf.exec();

        assert_eq!(bf.data, [101, 0].to_vec());
        assert_eq!(bf.result, "He".to_string());
    }

    #[test]
    fn test_hello_world() {
        let hello_world_test_file =
            fs::read_to_string("examples/hello_world.txt").expect("test file not found");
        let mut bf = BrainFuck::new(hello_world_test_file.trim().to_string());
        bf.exec();

        assert_eq!("Hello, World!".to_string(), bf.result);
    }
}
