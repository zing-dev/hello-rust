use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("hello world");
    let first = env::args_os().next().unwrap();
    println!("{}",first.as_os_str().to_str().unwrap());
    let name = Path::new(first.as_os_str()).file_name().unwrap();
    println!("{}",name.to_str().unwrap());

    let result = Command::new("echo").arg("hello world").output().unwrap();
    println!("{}",String::from_utf8(result.stdout).unwrap());

    let result = Command::new("pwd").output().unwrap();
    println!("{}",String::from_utf8(result.stdout).unwrap());

    let result = Command::new("tasklist").arg("/FI").arg("IMAGENAME eq bash.exe").output().unwrap();
    let res = String::from_utf8(result.stdout).unwrap();
    if res.contains("bash.exe") {
        println!("ok")
    }

    let result = Command::new("tasklist").arg("/FI").arg(format!("IMAGENAME eq {}",name.to_str().unwrap())).output().unwrap();
    let res = String::from_utf8(result.stdout).unwrap();
    if res.contains(name.to_str().unwrap()) {
        println!("ok")
    }

    let size = res.find(name.to_str().unwrap()).unwrap();
    println!("{}",size);
    let mut count = 0;
    for line in res.lines() {
        if line.contains(name.to_str().unwrap()) {
            count+=1;
        }
    }
    println!("count {}",count)
}