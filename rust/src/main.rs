use enterprise::parser::to_ast;
use std::fs;

fn main() {
    let fizz = fs::read_to_string("src/samples/fdcFizzBuzzDelegator.E™").expect("Cannot read fizz");
    let ast = to_ast(&fizz);
    println!("AST: {:?}", ast);
}
