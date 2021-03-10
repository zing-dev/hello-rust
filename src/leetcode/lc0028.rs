pub mod lc0028{
    use std::ops::Index;

    struct Solution{}
    impl Solution {
        pub fn str_str(haystack: String, needle: String) -> i32 {
            //
            let p1 = haystack.as_bytes();
            let p2 = needle.as_bytes();
            let mut i = 0;
            let mut j = 0;
            while i < p1.len() && j < p2.len() {
                if p1[i] == p2[j] {
                    i+=1;
                    j+=1;
                }else{
                    i = i - j +1;
                    j = 0;
                }
            }
            println!("{} {}",i,j);
            if j == p2.len() {
                return (i-j) as i32
            }
            -1
        }
    }
    #[test]
    fn test(){
        assert_eq!(Solution::str_str(String::from("abcd"),String::from("bc")),1);
        assert_eq!(Solution::str_str(String::from("abcd"),String::from("0")),-1);
    }
}

// 执行用时：0 ms, 在所有 Rust 提交中击败了100.00% 的用户
// 内存消耗：2.3 MB, 在所有 Rust 提交中击败了8.62% 的用户