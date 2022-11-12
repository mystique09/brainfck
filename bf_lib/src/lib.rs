pub mod token;

use std::{
    char,
    io::{self, Write},
};
use token::Token;

#[derive(Debug)]
pub struct BrainFuck {
    pub data: Vec<u32>,
    pub src: Vec<Token>,
    pub index: usize,
    pub mem_pos: usize,
    pub check_point: usize,
    pub result: String,
}

impl BrainFuck {
    pub fn new(src: String) -> Self {
        Self {
            data: [0; 3000].to_vec(),
            src: src
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(Token::from)
                .collect::<Vec<Token>>(),
            index: 0,
            mem_pos: 0,
            check_point: 0,
            result: String::new(),
        }
    }

    fn incr(&mut self) {
        if self.data.len() <= self.mem_pos {
            self.data.push(0);
        }
        self.data[self.mem_pos] += 1;
    }

    fn decr(&mut self) {
        if self.data.len() == self.mem_pos {
            self.data.push(0);
        }

        if self.data[self.mem_pos] < 1 {
            self.data[self.mem_pos] = 1;
        }

        self.data[self.mem_pos] -= 1;
    }

    fn movl(&mut self) {
        if self.mem_pos > 0 {
            self.mem_pos -= 1;
        }
    }

    fn movr(&mut self) {
        if self.data.len() == self.mem_pos {
            self.data.push(0);
        }
        self.mem_pos += 1;
    }

    fn opt(&mut self) {
        let ch = char::from_u32(self.data[self.mem_pos]).unwrap_or('\r');
        self.result.push(ch);
        print!("{}", ch);
    }

    fn inpt(&mut self) {
        let mut inp: String = String::new();
        io::stdin().read_line(&mut inp).expect("Unable to readline");

        let chrs = inp.chars().collect::<Vec<char>>();
        io::stdout().flush().unwrap();

        if self.mem_pos >= self.data.len() {
            self.data.push(0);
            return;
        }

        for chr in chrs {
            self.data[self.mem_pos] = chr as u32;
            self.movr();
        }
    }

    fn sloop(&mut self) {
        self.check_point = self.index;
    }

    fn eloop(&mut self) {
        if self.mem_pos >= self.data.len() {
            self.data.push(0);
        }
        if self.data[self.mem_pos] != 0 {
            self.index = self.check_point;
        }
    }

    pub fn parse(&mut self, token: Token) {
        match token {
            Token::INCR => self.incr(),
            Token::DECR => self.decr(),
            Token::MOVR => self.movr(),
            Token::MOVL => self.movl(),
            Token::OPT => self.opt(),
            Token::INPT => self.inpt(),
            Token::SLOOP => self.sloop(),
            Token::ELOOP => self.eloop(),
            Token::NONE => (),
        }
    }

    pub fn exec(&mut self) {
        while self.index < self.src.len() {
            let token: Token = self.src[self.index];
            self.index += 1;
            self.parse(token);
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
