use bf_lib::BrainFuck;
use std::fs;

const HELP_MESSAGE: &str = r#"
Usage:
  brainfck <option> <filename>

Option:
  -h Output this message.
"#;

fn main() {
    let mut arg = std::env::args();
    arg.next();

    match arg.next() {
        Some(n) => match n.as_str() {
            "-h" => println!("{}", HELP_MESSAGE),
            _ => {
                let f = fs::read_to_string(n).expect("File not found");
                let f = f.trim().replace(['\n', ' '], "");
                let mut bf = BrainFuck::new(f);
                bf.exec();
                // println!("{}", bf.result);
            }
        },
        None => println!("{}", HELP_MESSAGE),
    }
}
