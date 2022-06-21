use std::env;

fn main() {
    let mut argv = env::args();
    let operand_lim = match argv.nth(1) {
        Some(arg) => arg,
        None => String::from("y"),
    };

    // TODO: Optimize output like gnu/core-utils "yes"

    loop {
        println!("{operand_lim}");
    }
}
