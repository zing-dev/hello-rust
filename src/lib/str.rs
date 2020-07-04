pub mod str {
    #[test]
    fn into_boxed_bytes() {
        let s = "this is a string";
        let boxed_str = s.to_owned().into_boxed_str();
        let boxed_bytes = boxed_str.into_boxed_bytes();
        assert_eq!(*boxed_bytes, *s.as_bytes());
    }

    #[test]
    fn replace() {
        let s = "this is old";
        assert_eq!("this is new", s.replace("old", "new"));

        let s = "this is old";
        assert_eq!(s, s.replace("cookie monster", "little lamb"));
    }

    #[test]
    fn replacen() {
        let s = "foo foo 123 foo";
        assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
        assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
        assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
    }

    #[test]
    fn to_lowercase() {
        let s = "HELLO";
        assert_eq!("hello", s.to_lowercase());

        let sigma = "Σ";
        assert_eq!("σ", sigma.to_lowercase());

        // but at the end of a word, it's ς, not σ:
        let odysseus = "ὈΔΥΣΣΕΎΣ";
        assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

        let new_year = "农历新年";
        assert_eq!(new_year, new_year.to_lowercase());
    }

    #[test]
    fn into_string() {
        let string = String::from("birthday gift");
        let boxed_str = string.clone().into_boxed_str();
        assert_eq!(boxed_str.into_string(), string);
    }

    #[test]
    fn repeat() {
        println!("{}", "hello ".repeat(2))
    }
}
