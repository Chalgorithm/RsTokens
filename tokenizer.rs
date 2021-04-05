use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn tokenize(string:&String) -> Vec<&str>{
    // to_lowercase does nothing yet
    string.to_lowercase();
    let tokens:Vec<&str> = string.split(' ').collect();
    return tokens;
}

fn main(){
    let mut input = String::new();
    println!("file to tokenize:");
    std::io::stdin().read_line(&mut input).unwrap();
    print!("{}",input);
    input.pop();
    let path = Path::new(&input);
    let display = path.display();

    let mut file = match File::open( &path ){
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    let collection:Vec<&str> = match file.read_to_string(&mut s){
        Err(why) => panic!("couldn't open {}: {}",display,why),
        Ok(_) => tokenize(&s),
    };
    for index in 0..collection.len(){
        print!("{}\n",collection[index]);
    }
}
