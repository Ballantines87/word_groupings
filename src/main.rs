use std::collections::HashMap;

fn main() {
    let words: Vec<String> = vec!["the".to_owned(), "het".to_owned(),
    "teh".to_owned(), "stupid".to_owned(),"studpi".to_owned(),
    "apple".to_owned(), "appel".to_owned(),"apepl".to_owned()];

    let groupings = word_grouping(words);
    let user_input = "teh";

    for words in groupings.into_iter() {
        if words.contains(&user_input.to_owned()) {
            println!("The input {:?} belongs to the grouping {:?}", user_input, words);
        }
    }

}



fn word_grouping(words: Vec<String>) -> Vec<Vec<String>> {
    let mut words_hash = HashMap::new();
    let mut chars_freq = vec![0;26];

    for word in words {
        for c in word.to_lowercase().chars() {
            chars_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }

        let key =  chars_freq.into_iter().map(|item| item.to_string()).collect::<String>();
        words_hash.entry(key).or_insert(Vec::new()).push(word);

        chars_freq = vec![0;26];

        for (key, value) in &words_hash {
            println!("Key # {:?}, Value # {:?}", key, value);
        }

        
    }

    words_hash.into_iter().map(|(_, v)| v).collect()
}

