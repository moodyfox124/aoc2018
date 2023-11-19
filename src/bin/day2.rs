use std::{collections::HashMap, fs};

const DATA_FOLDER: &str = "./data";

struct Scanner;

impl Scanner {
    fn calculate_checksum(data: &str) -> i32 {
        let mut doubles = 0;
        let mut triples = 0;
        for line in data.lines() {
            let step = 1;
            let mut letter_count = HashMap::new();
            for letter in line.chars() {
                match letter_count.get(&letter) {
                    Some(amount) => {
                        letter_count.insert(letter, amount + step);
                    }
                    None => {
                        letter_count.insert(letter, step);
                    }
                }
            }

            let mut is_double_added = false;
            let mut is_triple_added = false;
            letter_count.iter().for_each(|(_k, v)| {
                if *v == 2 && !is_double_added {
                    doubles += 1;
                    is_double_added = true;
                } else if *v == 3 && !is_triple_added {
                    triples += 1;
                    is_triple_added = true;
                }
            })
        }
        return doubles * triples;
    }

    fn find_duplicate(data: &str, letter_diffference: usize) -> Option<String> {
        let mut character_lines: Vec<Vec<char>> = Vec::new();
        let mut result: Option<String> = None;
        let mut diff_index: Option<char> = None;
        for line in data.lines() {
            let chars: Vec<char> = line.chars().collect();
            let is_any = character_lines.iter().any(|c| {
                let mut diff = letter_diffference as i32;
                for (index, v) in c.iter().enumerate() {
                    let char_at_index = chars.get(index).expect("missing char at index");
                    if *char_at_index != *v {
                        diff -= 1;
                        diff_index = Some(*v);
                    }

                    if diff < 0 {
                        return false;
                    }
                }
                let remove_char = diff_index.expect("Should have char");
                result = Some(c.into_iter().filter(|&&v| v != remove_char).collect());

                return true;
            });
            if is_any {
                return result;
            }
            character_lines.push(chars);
        }
        return result;
    }
}

fn main() {
    let data = fs::read_to_string(format!("{DATA_FOLDER}/day2.txt"))
        .expect("Unable to open file with data");
    println!("Checksum {}", Scanner::calculate_checksum(&data));

    println!("Similar string {:?}", Scanner::find_duplicate(&data, 1));
}
