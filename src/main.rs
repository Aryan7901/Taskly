use std::env::args;
use std::iter::Iterator;
use std::process;
use taskly::Conditions;
fn main() {
    let _ = enable_ansi_support::enable_ansi_support();
    let args: Vec<String> = args().skip(1).collect();
    run(args);
}
fn run(args: Vec<String>) {
    let condition = match args.len() {
        0 => Conditions::default(),
        1 => Conditions::from(Some(&args[0]), None, None),
        2 => Conditions::from(Some(&args[0]), Some(&args[1]), None),
        _ => Conditions::from(Some(&args[0]), Some(&args[1]), Some(&args[2])),
    };
    if let Err(err) = condition.exec() {
        println!("Error: {}", err);
        process::exit(1);
    }
}
