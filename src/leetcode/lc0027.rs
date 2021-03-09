pub mod lc0027 {
    struct Solution {}

    impl Solution {
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            let mut current = 0;
            let mut index = 0;
            while index < nums.len() {
                if nums[index] != val {
                    nums[current] = nums[index];
                    current += 1;
                }
                index += 1;
            }
            current as i32
        }
    }

    #[test]
    fn test() {
        let data = &vec![
            vec![1, 1, 2, 2, 3, 4],
            vec![1, 2, 2, 3, 4, 4, 5],
            vec![1, 2, 2, 2, 4, 4, 4],
            vec![1, 2, 2, 3, 4, 4, 4, 5]
        ];
        let val = vec![2, 4, 4, 5];
        let res = vec![4, 5, 4, 7];
        let mut index = 0;
        while index < data.len() {
            let mut num = data[index].clone();
            assert_eq!(Solution::remove_element(&mut num, val[index]), res[index]);
            println!("{} {:?} {} {}", index + 1, num, val[index], res[index]);
            index += 1;
        }
    }
}