pub mod lc0026 {
    struct Solution {}

    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() == 0 {
                return 0;
            }
            let mut slow = 0;
            let mut fast = 1;
            while fast < nums.len() {
                if nums[fast] != nums[slow] {
                    slow += 1;
                    nums[slow] = nums[fast];
                }
                fast += 1;
            }
            (slow + 1) as i32
        }
    }

    #[test]
    fn test() {
        let data = &vec![
            vec![],
            vec![0, 0, 2, 2, 2, 3, 4],
            vec![1, 1, 2, 2, 3, 4, 4],
            vec![1, 2, 2, 3, 4, 4, 5],
            vec![1, 2, 2, 2, 4, 4, 4],
            vec![1, 2, 2, 3, 4, 4, 4, 5]
        ];
        let res = vec![0, 4, 4, 5, 3, 5];
        let mut index = 0;
        while index < data.len() {
            let mut num = data[index].clone();
            assert_eq!(Solution::remove_duplicates(&mut num), res[index]);
            println!("{} {:?} {}", index + 1, num, res[index]);
            index += 1;
        }
    }
}
// 删除排序数组中的重复项

// 执行用时：4 ms, 在所有 Rust 提交中击败了43.92% 的用户
// 内存消耗：2.1 MB, 在所有 Rust 提交中击败了74.83% 的用户