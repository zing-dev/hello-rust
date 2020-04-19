#[cfg(test)]
pub mod types_test {
    #[test]
    pub fn cast() {
        let v1 = 111111.1111;
        println!("{}", v1);

        let v2: i8 = v1 as i8;
        println!("{}", v2);
    }
}