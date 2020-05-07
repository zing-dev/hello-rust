use std::io;

#[test]
fn stdin() {
    let mut string = String::new();
    match io::stdin().read_line(&mut string) {
        Ok(_) => {}
        Err(_) => {}
    }
    println!("{}", string);
}
