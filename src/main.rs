use std::env;
use rand::{Rng, rngs::ThreadRng};
use std::fs;

//this is a joke choose your own adventure generator. It picks random words so you can choose the ones you like as you read and create your own adventure.

fn main() {
    //this seems messy. there has to be a better way.
    let default_word_count: usize = 3000;
    let args: Vec<String> = env::args().skip(1).collect();
    let word_count: usize = match args.first() {
        Some(arg) => arg.parse().unwrap_or(default_word_count),
        None => default_word_count
    };

    let common_words: Vec<String> = read_file("/home/justin/Projects/AdventureGenerator/src/commonwords.txt");
    let all_words: Vec<String> = read_file("/home/justin/Projects/AdventureGenerator/src/allwords.txt");

    let mut buffer:Vec<String> = Vec::with_capacity(word_count);
    let mut rng: ThreadRng = rand::thread_rng();
    for _i in 0..word_count {
        if rng.gen_range(0..100) < 75 {
            buffer.push(common_words[rng.gen_range(0..common_words.len())].clone());
        }
        else {
            buffer.push(all_words[rng.gen_range(0..all_words.len())].clone());
        }
    }

    println!("{}", buffer.join(" "));
}

fn read_file(file: &str) -> Vec<String> {
  match fs::read_to_string(&file) {
    Ok(value) => value.split("\n").map(|s| s.into()).collect(),
    Err(e) =>{
        eprintln!("Error reading file: {}: \n with error: {}", file, e);
        std::process::exit(1);
        }
  }
}