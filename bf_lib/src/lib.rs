use std::{char, collections::VecDeque, io};

const MAX_MEMORY: usize = 30000;

#[derive(Debug)]
pub struct BrainFuck {
    pub data: Vec<u32>,
    pub src: Vec<char>,
    pub index: usize,
    pub pointer: u32,
    pub check_point: VecDeque<usize>,
    pub result: String,
}

impl BrainFuck {
    pub fn new(src: String) -> Self {
        let src = src.chars().collect::<Vec<char>>();
        let deq: VecDeque<usize> = VecDeque::new();

        Self {
            data: vec![0; MAX_MEMORY],
            src,
            index: 0,
            pointer: 0,
            check_point: deq,
            result: String::new(),
        }
    }

    fn incr(&mut self) {
        // let sub = if self.pointer > 0 {
        //     self.pointer - 1
        // } else {
        //     self.pointer
        // };

        if self.pointer == self.data.len() as u32 {
            self.data.push(0);
            return;
        }

        self.data[self.pointer as usize] += 1;
    }

    fn decr(&mut self) {
        if self.data[self.pointer as usize] == 0 {
            return;
        }

        self.data[self.pointer as usize] = self.data[self.pointer as usize].wrapping_sub(1);
    }

    fn movl(&mut self) {
        if self.pointer == 0 {
            return;
        }

        // Use wrapping_sub to handle underflow
        self.pointer = self.pointer.wrapping_sub(1);
    }

    fn movr(&mut self) {
        if self.pointer == self.data.len() as u32 {
            self.data.push(0);
            return;
        }
        // Use wrapping_add to handle overflow
        self.pointer = self.pointer.wrapping_add(1);
    }

    fn opt(&mut self) {
        if self.pointer == self.data.len() as u32 {
            return;
        }
        // Use unwrap_or_else to handle conversion failure without panicking
        let ch = char::from_u32(self.data[self.pointer as usize] as u32).unwrap_or('\0');
        self.result.push(ch);
        print!("{}", ch);
    }

    fn inpt(&mut self) {
        let mut inp: String = String::new();
        io::stdin().read_line(&mut inp).expect("Unable to readline");

        // Use the value of `inp` here
        for chr in inp.chars() {
            self.data[self.pointer as usize] = chr as u32;
            self.movr();
        }
    }

    fn sloop(&mut self) {
        let i = if self.index == 0 { 0 } else { self.index - 1 };
        self.check_point.push_front(i);
    }

    fn eloop(&mut self) {
        // Check if the memory position is out of bounds
        if self.pointer == self.data.len() as u32 {
            self.data.push(0);
        }

        let cell = self.data.get(self.pointer as usize).unwrap();

        if *cell > 0 {
            // println!(
            //     "total checkpoint before popping front element: {:?}",
            //     self.check_point.len()
            // );
            // If the current memory cell is not zero, pop the top item from the queue
            let check_point = self.check_point.get(0).unwrap();
            // Set the index to the popped value
            // println!(
            //     "total checkpoint after popping front element {:?}",
            //     self.check_point.len()
            // );
            // self.check_point.remove(0);
            self.index = *check_point;
        }

        self.check_point.remove(0);
    }

    pub fn process(&mut self, token: char) {
        match token {
            '+' => self.incr(),
            '-' => self.decr(),
            '>' => self.movr(),
            '<' => self.movl(),
            '.' => self.opt(),
            ',' => self.inpt(),
            '[' => self.sloop(),
            ']' => self.eloop(),
            _ => println!("Token: {}", token),
        }
    }

    pub fn exec(&mut self) {
        while self.index < self.src.len() {
            let token: char = self.src[self.index];
            self.process(token);
            self.index += 1;
            // println!("{:?}, {:?}", self.pointer, self.data[self.pointer as usize]);
            // println!("{:?}, {:?}", self.index, self.src[self.index - 1 as usize]);
            // print!("{esc}c", esc = 27 as char);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_instance() {
        let mut bf = BrainFuck::new(">++++++++<.".to_string());

        bf.exec();
        assert_eq!(bf.data[1], 8);
        assert!(!bf.src.is_empty());
    }
}
