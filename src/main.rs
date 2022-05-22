use compiler::compile;

mod compiler;

fn main() {
    let result = compile("function main() { println(\"lol\"); }", false).unwrap();
    println!("{}{}", result.output, result.error);
}
