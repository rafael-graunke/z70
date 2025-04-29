mod parser;

use crate::parser::parse_file;

fn main() {
    let result = parse_file("mov A, B\n LOOP: \n dec A\n");

    println!("AAAAAAAAAAAAAAAAAAAAAA {:?}", result);
}
