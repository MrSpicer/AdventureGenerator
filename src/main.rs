use std::env;
use rand::{Rng, rngs::ThreadRng};
//use std::fs;

//this is a joke choose your own adventure generator. It picks random words so you can choose the ones you like as you read and create your own adventure.


// take an input for number of words to output: word_count
// read word lists in
// loop word_count number of times
    //do a random 0-1
    // if <0.75 pick a random word from the common list
    //else pick a word from the crossword list
    //write word to string builder or buffer or whatever
//output string

fn main() {
    //this seems messy. there has to be a better way.
    let default_word_count = 3000;
    let args: Vec<String> = env::args().skip(1).collect();
    let word_count: usize = match args.first() {
        Some(arg) => arg.parse().unwrap_or(default_word_count),
        None => default_word_count
    };

    //replace with read from files
    let common_words: Vec<String> = vec!["the".to_string(),"be".to_string(),"and".to_string(),"a".to_string()];
    let all_words: Vec<String> = vec!["aardvark".to_string(),"zebra".to_string(),"test".to_string(),"walnut".to_string(),"ear".to_string(),"pants".to_string()];

    let mut buffer:Vec<String> = Vec::with_capacity(word_count);

    let mut rng: ThreadRng = rand::thread_rng();

    for _i in 1..word_count {
        let rnum: u32 = rng.gen_range(0..100);
        println!("the num: {}", rnum);

        if rnum < 75 {
            let word: String = common_words[rng.gen_range(0..common_words.len())].clone();
            buffer.push(word);
        }
        else {
            let word: String = all_words[rng.gen_range(0..all_words.len())].clone();
            buffer.push(word);

        }
    }
    println!("word count: {}", word_count);

    println!("test: {}", buffer.join(" "));
}