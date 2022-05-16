use bf_lib::BrainFuck;
use std::fs;

const HELP_MESSAGE: &'static str = r#"
Usage:
  brainfck <option> <filename>

Option:
  -h Output this message.
  -m Output the memory.
"#;

fn main() {
    let mut arg = std::env::args();
    arg.next();

    match arg.next() {
        Some(n) => match n.as_str() {
            "-h" => println!("{}", HELP_MESSAGE),
            _ => {
                let f = fs::read_to_string(n).expect("File not found");
                let mut bf = BrainFuck::new(f.trim().to_string());
                bf.exec();
                println!("{}", bf.result);
                return;
            }
        },
        None => println!("{}", HELP_MESSAGE),
    }
}
