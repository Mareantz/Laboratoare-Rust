//ex1
use std::{fs, io};
fn ex1() {
    let read = fs::read_to_string("src/input1.txt");
    let s = read.unwrap();
    let mut max_size: u32 = 0;
    let mut max_char: u32 = 0;
    let mut max_size_string: String = String::new();
    let mut max_char_string: String = String::new();

    for i in s.lines() {
        let mut char_count: u32 = 0;
        if i.len() as u32 > max_size {
            max_size = i.len() as u32;
            max_size_string = String::from(i);
        }
        for _ in i.chars() {
            char_count += 1;
        }
        if char_count > max_char {
            max_char = char_count;
            max_char_string = String::from(i);
        }
    }
    println!(
        "The string with the longest number of bytes is {}",
        max_size_string
    );
    println!(
        "The string with the longest number of characters is {}",
        max_char_string
    );
}

fn ex2() {
    let read = fs::read_to_string("src/input2.txt");
    let to_cipher = read.unwrap();
    let mut ciphered: String = String::new();
    println!("The text to cipher is '{}'", to_cipher.trim_end());

    for ch in to_cipher.chars() {
        if ch == ' ' || ch == '\n' {
            ciphered.push(ch);
            continue;
        }
        if (ch < 'A' || ch > 'Z')
            && (ch < 'a' || ch > 'z')
            && (ch != ' ')
            && (ch as u8 != 13)
            && (ch != '\n')
        {
            println!("{}", ciphered);
            println!("Error: the text contains non-letter characters");
            return;
        }
        if (ch >= 'a' && ch < 'n') || (ch >= 'A' && ch < 'N') {
            ciphered.push((ch as u8 + 13) as char);
        } else {
            ciphered.push((ch as u8 - 13) as char);
        }
    }
    println!("The ciphered text is '{}'", ciphered.trim_end());
}

fn ex3() {
    let read = fs::read_to_string("src/input3.txt");
    let abbreviated = read.unwrap();
    let mut s: String = String::new();
    for i in abbreviated.split(" ") {
        if i == "pt" || i == "ptr" {
            s.push_str("pentru");
        } else if i == "dl" {
            s.push_str("domnul");
        } else if i == "dna" {
            s.push_str("doamna");
        } else {
            s.push_str(i);
        }
        s.push(' ');
    }
    println!("{}\n=>\n{}", abbreviated, s);
}

fn ex4() {
    let read = fs::read_to_string("src/input4.txt");
    let hosts = read.unwrap();
    let mut modified: String = String::new();
    for i in hosts.lines() {
        if i.starts_with("#") {
            continue;
        } else {
            let aux = i.trim();
            //let mut ok: u8 = 0;
            // for str in aux.split(" ") {
            //     //println!("{}",str);
            //     modified.push_str(str);
            //     if ok == 1 {
            //         modified.push_str(" => ");
            //     }
            //     ok += 1;
            //     if ok == 2 {
            //         break;
            //     }
            // }
            // for str in aux.splitn(3, ' ') {
            //     println!("{} and ok is {}",str,ok);
            //     modified.push_str(str);
            //     if ok == 2 {
            //         modified.push_str(" => ");
            //     }
            //     ok += 1;
            //     if ok == 2 {
            //         break;
            //     }
            // }
            // other stuff i tried
            if let Some((first, second)) = aux.split_once(" ") {
                let second = second.splitn(2, ' ').next().unwrap_or(second);
                modified.push_str(second);
                modified.push_str(" => ");
                modified.push_str(first);
                modified.push_str("\n")
            }
            //idk how to work for more than one space between ip and host
        }
    }
    println!("{}", modified);
}

fn main() {
    ex1();
    println!("====================");
    ex2();
    println!("====================");
    ex3();
    println!("====================");
    ex4();
}
