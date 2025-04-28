mod parser;

use crate::parser::types::Statement;
use crate::parser::parse_label;

fn main() {
    let (_, teste) = parse_label("TASDASD        :      \n").unwrap();

    match teste {
        Statement::Label(name) => { println!("{}<<<", name) },
        Statement::Operation(_,_,_ ) => { print!("boom"); }
    }

}
