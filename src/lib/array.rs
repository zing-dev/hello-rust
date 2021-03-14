pub mod array {
    #[test]
    fn test() {
        let x = [1, 2, 3, 4, 5];
        println!("{:?}", x);
        println!("{:?}", x.len());
        println!("{:?} {:?}", x.get(0), x.get(0).unwrap());
        println!("{:?},{:?}", x.binary_search(&2), x.binary_search(&3));
        println!("{:?},{:?}", x.iter(), x.iter().next());
        let x1 = x.as_ptr();
        println!("{:?}", x1)
    }
}
