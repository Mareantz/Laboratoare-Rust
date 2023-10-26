//ex1
fn prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    let mut i: u32 = 3;
    if n % 2 == 0 {
        return false;
    }

    while i <= n / 2 {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut next: u32 = x as u32 + 1;
    let max = std::u16::MAX as u32;
    while next < max {
        if prime(next) == true {
            break;
        }
        next += 1;
    }
    if next < max {
        return Some(next as u16);
    } else {
        return None;
    }
}

fn ex1() {
    let mut nr: u32 = 65300;
    let mut y: u16;

    while nr < std::u32::MAX {
        let x = next_prime(nr as u16);

        match x {
            Some(value) => {
                print!("{value}\n");
                y = value;
            }
            None => {
                println!("None\n");
                break;
            }
        }

        nr = y as u32;
    }
}

//ex2
fn checked_add(x: u32, y: u32) -> u32 {
    let sum: u64 = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("Overflow");
    } else {
        return sum as u32;
    }
}

fn checked_multiplication(x: u32, y: u32) -> u32 {
    let sum: u64 = x as u64 * y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("Overflow");
    } else {
        return sum as u32;
    }
}

#[derive(Debug)]
enum Error {
    Overflow,
}

fn checked_add_result(x: u32, y: u32) -> Result<u32, Error> {
    let sum: u64 = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    } else {
        Ok(sum as u32)
    }
}

fn checked_multiplication_result(x: u32, y: u32) -> Result<u32, Error> {
    let sum: u64 = x as u64 * y as u64;
    if sum > std::u32::MAX as u64 {
        Err(Error::Overflow)
    } else {
        Ok(sum as u32)
    }
}
fn ex23() {
    let result1 = checked_add(3461, 20125);
    println!("{:?}", result1);
    let result2 = checked_multiplication(64, 98);
    println!("{:?}", result2);
    let result3 = checked_add_result(std::u32::MAX, 65535);
    println!("{:?}", result3);
    let result4 = checked_multiplication_result(std::u32::MAX, 2);
    println!("{:?}\n", result4);
}

#[derive(Debug)]
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if c < 'a' || c > 'z' {
        print_error(CharError::NotLetter);
        return Err(CharError::NotLetter);
    }
    Ok((c as u8 + 'A' as u8 - 'a' as u8) as char)
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if c < 'A' || c > 'Z' {
        print_error(CharError::NotLetter);
        return Err(CharError::NotLetter);
    }
    Ok((c as u8 + 'a' as u8 - 'A' as u8) as char)
}

fn print_char(c: char) -> Result<char, CharError> {
    if (c as u8) > 127 || (c as u8) < 32 {
        print_error(CharError::NotPrintable);
        return Err(CharError::NotPrintable);
    }
    Ok(c as char)
}
fn char_to_number(c: char) -> Result<u8, CharError> {
    if c as u8 > 128 {
        print_error(CharError::NotAscii);
        return Err(CharError::NotAscii);
    }
    if c < '0' || c > '9' {
        print_error(CharError::NotDigit);
        return Err(CharError::NotDigit);
    }
    Ok((c as u8 - '0' as u8) as u8)
}
fn char_to_number_hex(c: char) -> Result<u8, CharError> {
    if c as u8 > 128 {
        print_error(CharError::NotAscii);
        return Err(CharError::NotAscii);
    }
    if (c < '0' || c > '9') && (c < 'A' || c > 'F') {
        print_error(CharError::NotBase16Digit);
        return Err(CharError::NotBase16Digit);
    }
    if c >= '0' && c <= '9' {
        Ok((c as u8 - '0' as u8) as u8)
    } else {
        Ok((c as u8 - 55) as u8)
    }
}
fn print_error(error: CharError) {
    match error {
        CharError::NotAscii => println!("The character is not an ASCII character!"),
        CharError::NotDigit => println!("The character is not a digit!"),
        CharError::NotBase16Digit => println!("The character is not a base 16 digit!"),
        CharError::NotLetter => println!("The character is not a letter!"),
        CharError::NotPrintable => println!("The character is not printable!"),
    }
}

fn ex4() {
    let a: char = 'c';
    let b: char = 'F';
    let c: char = 20 as char;
    let d: char = 38 as char;
    let e: char = '5';
    let f: char = 'C';
    let result1 = to_uppercase(a);
    let result2 = to_lowercase(b);
    let result3 = print_char(c);
    let result4 = print_char(d);
    let result5 = char_to_number(e);
    let result6 = char_to_number_hex(f);
    println!(
        "{:?},{:?},{:?},{:?},{:?},{:?}\n",
        result1, result2, result3, result4, result5, result6
    );
}

fn oglindit(n: u32) -> Option<u32> {
    let mut aux: u32 = n;
    let mut ogl: u32 = 0;
    while aux != 0 {
        ogl = ogl * 10 + aux % 10;
        aux /= 10;
    }
    if ogl == n {
        return Some(n);
    } else {
        return None;
    }
}

fn ex5() {
    let r = oglindit(121);
    if r.is_some() {
        println!("Numarul {} este oglindit", r.unwrap());
    } else {
        println!("Eroare: numarul nu este oglindit");
    }
}

fn main() {
    ex1();
    ex23();
    ex4();
    ex5();
}
