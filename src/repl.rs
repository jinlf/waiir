use super::lexer::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

const PROMPT: &str = ">> ";
pub fn start(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut fmt = BufWriter::new(output);
    loop {
        fmt.write_fmt(format_args!("{}", PROMPT)).unwrap();
        fmt.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut l = Lexer::new(&mut line);
        loop {
            let tok = l.next_token();
            if tok.tk_type == TokenType::EOF {
                break;
            }
            fmt.write_fmt(format_args!("{:?}\n", tok)).unwrap();
        }
    }
}
