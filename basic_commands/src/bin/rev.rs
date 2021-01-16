use std::io::{self, BufRead};


fn main(){
    io::stdin().lock().lines()
    .map(|line|line.expect("ligne"))
    .map(reverse_string)
    .for_each(|line|println!("{}", line));
}

fn reverse_string(string: String)->String{
    let mut tab:Vec<char>= string.chars().collect();
    tab.reverse();
    return tab.iter().collect();
    
}