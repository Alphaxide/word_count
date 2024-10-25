use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use std::path::Path;

fn count_words_in_file(file_path: &Path) -> HashMap<String, usize> {
    let contents = fs::read_to_string(file_path).expect("could not read file");
    let mut word_count = HashMap::new();

    for word in contents.split_whitespace() {
        let word = word.to_lowercase();
        *word_count.entry(word).or_insert(0) +=1;
    }

    word_count
}

fn aggregate_word_counts(global_count: Arc<Mutex<HashMap<String, usize>>>, local_count: HashMap<String, usize>) {
    let mut global = global_count.lock().expect("could not lock mutex");


    for (word, count) in local_count{
        *global.entry(word).or_insert(0) += count;
    }
}

fn main() {
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];

    let global_word_count = Arc::new(Mutex::new(HashMap::new()));

    files.par_iter().for_each(|file_path|{
        let local_word_count = count_words_in_file(Path::new(file_path));
        aggregate_word_counts(Arc::clone(&global_word_count), local_word_count);

        let final_word_count = global_word_count.lock().expect("could not lock mutex");

        println!("word counts:");
        for (word, count) in final_word_count.iter() {
            println!("{} : {}", word, count)
        }

    });

}