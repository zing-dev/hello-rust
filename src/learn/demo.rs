#[cfg(test)]
pub mod demo {
    use std::io;
    use std::io::BufRead;

    #[test]
    pub fn guess_number() {
        let r = rand::random::<u8>();
        println!("please input number");
        loop {
            let mut num = String::new();
            io::stdin().read_line(&mut num).expect("read error");
            let num = num.parse::<u8>().expect("parse error");
            if num > r {
                println!("bigger");
                continue;
            } else if num < r {
                println!("smaller");
                continue;
            } else {
                println!("success");
            }
        }
    }
}
