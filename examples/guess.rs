use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

use rand::Rng;

fn main() {
    let i = rand::thread_rng().gen_range(1..1000);
    loop {
        let mut num = String::new();
        if let Err(_) = io::stdin().lock().read_line(&mut num) {
            println!("input error!");
            continue;
        }
        let num = if let Ok(num) = num.trim().parse::<i32>() {
            num
        } else {
            println!("parse error!");
            continue;
        };
        match num.cmp(&i) {
            Ordering::Less => println!("smaller..."),
            Ordering::Greater => println!("bigger..."),
            Ordering::Equal => {
                println!("success...");
                break;
            }
        }
        if num < i {
            println!("smaller...");
        } else if num > i {
            println!("bigger...");
        } else {
            println!("success...");
            break;
        }
    }
}
