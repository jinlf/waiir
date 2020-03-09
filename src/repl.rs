use super::ast::*;
use super::lexer::*;
use super::parser::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

const PROMPT: &str = ">> ";
const MONKEY_FACE: &str = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;
pub fn start(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut fmt = BufWriter::new(output);
    loop {
        fmt.write_fmt(format_args!("{}", PROMPT)).unwrap();
        fmt.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut l = Lexer::new(&mut line);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        if p.get_errors().len() != 0 {
            print_parser_errors(&mut fmt, p.get_errors());
            continue;
        }

        fmt.write_fmt(format_args!("{}\n", program.unwrap().string()))
            .unwrap();
    }
}

fn print_parser_errors(fmt: &mut BufWriter<&mut dyn Write>, errors: &Vec<Box<String>>) {
    fmt.write_fmt(format_args!("{}", MONKEY_FACE)).unwrap();
    fmt.write_fmt(format_args!(
        "{}",
        "Woops! We ran into some monkey business here!\n"
    ))
    .unwrap();
    fmt.write_fmt(format_args!("{}", " parser errors:\n"))
        .unwrap();
    for msg in errors {
        fmt.write_fmt(format_args!("\t{}\n", msg)).unwrap();
    }
    fmt.write_fmt(format_args!("\n")).unwrap();
}
