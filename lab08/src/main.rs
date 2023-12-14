use std::{cmp, collections::HashMap, fs, io::Write};

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    let input = fs::read_to_string("src/input.txt");
    let mut max_len = 0;
    match input {
        Ok(input) => {
            for word in input.split(|c| c == ' ' || c == ',' || c == '.') {
                if word.is_empty() {
                    continue;
                }
                *m.entry(word.to_lowercase()).or_default() += 1;
                max_len = cmp::max(max_len, word.len() as u32);
            }
        }
        Err(e) => eprintln!("Failed to read input: {}", e),
    }

    let mut vec: Vec<(&String, &i32)> = m.iter().collect();
    vec.sort_by(|a, b| a.1.cmp(b.1));

    for (k, v) in vec.iter().rev() {
        match write!(std::io::stdout(),"{:width$} => {}\n",k,v,width = max_len as usize) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to stdout: {}", e),
        }
    }
}
