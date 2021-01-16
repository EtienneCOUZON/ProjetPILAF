use std::io::{self, Read};

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin");

    let mut lines: Vec<&str>= input.lines().collect();
    lines.dedup();
    for line in lines.iter(){
        println!("{}", line);
    }
}