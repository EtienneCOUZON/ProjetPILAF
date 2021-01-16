use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    pattern: usize
}

fn main(){
    let options = Options::from_args();
    io::stdin().lock().lines()
    .map(|line|line.expect("ligne"))
    .take(options.pattern)
    .for_each(|line|println!("{}", line));
}