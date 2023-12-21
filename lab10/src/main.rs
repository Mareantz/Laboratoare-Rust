use std::io;
use std::cell::RefCell;

struct Cache {
    data: RefCell<Vec<(u32, bool)>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            data: RefCell::new(Vec::with_capacity(10)),
        }
    }

    fn get(&self, num: u32) -> Option<bool> {
        let data = self.data.borrow();
        data.iter().find_map(|&(n, prim)| if n == num { Some(prim) } else { None })
    }

    fn insert(&self, num: u32, prim: bool) {
        let mut data = self.data.borrow_mut();
        if data.len() == 10 {
            data.remove(0);
        }
        data.push((num, prim));
    }
}

fn prim(nr: u32) -> bool {
    if nr == 1 || nr == 0 {
        return false;
    }
    if nr % 2 == 0 && nr != 2 {
        return false;
    }
    let mut i: u32 = 3;
    while i * i <= nr {
        if nr % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn main() {
    let cache = Cache::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: u32 = input.trim().parse().unwrap();

        if let Some(prim) = cache.get(num) {
            println!("{} is prime: {}. (Retrieved from cache)", num, prim);
        } else {
            let prim = prim(num);
            println!("{} is prime: {}. (Calculated)", num, prim);
            cache.insert(num, prim);
        }
    }
}