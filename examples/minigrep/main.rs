use std::env;

mod minigrep;

use minigrep::Config;

fn main() {
    if let Err(e) = minigrep::run(Config::new(&env::args().collect::<Vec<String>>()).expect("err"))
    {
        println!("{}", e)
    }
}
