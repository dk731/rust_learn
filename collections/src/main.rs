use std::{collections::HashMap, str::pattern::Pattern};

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar".to_string();

    s1 += &s2;

    s1.push_str(&s2);
    println!("s1: {}, s2 is {}", s1, s2);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);

    //////////////////////////////////

    let mut arr = vec![6, 1, 2, 7, 9, 0, 2, 34, 23, 4];
    arr.sort();

    println!("Sorted array: {:?}", arr);
    println!("Array median is: {}", arr[arr.len() / 2]);

    //////////////////////////////////

    let test_str = "This is test string for apple first end last";
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result_str = String::new();

    for (i, word_slice) in test_str.split_ascii_whitespace().enumerate() {
        let mut transformed_word = match word_slice.chars().next() {
            None => continue,
            Some(char) => {
                if vowels.contains(&char) {
                    format!("{}{}-hay", char, word_slice)
                } else {
                    format!("{}-{}ay", word_slice, char)
                }
            }
        };

        if i != 0 {
            result_str += " ";
        }

        result_str += &transformed_word;
    }

    println!("Result string: '{}'", result_str);
}
