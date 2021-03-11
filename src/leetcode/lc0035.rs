pub mod lc0035 {
    struct Solution {}

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        }
        for i in 0..nums.len() - 1 {
            if nums[i] < target && target <= nums[i + 1] {
                return (i + 1) as i32;
            } else if nums[i] >= target {
                return i as i32;
            }
        }
        0
    }

    #[test]
    fn test() {
        let nums = [
            vec![1, 3, 5, 6],
            vec![1, 3, 5, 6],
            vec![1, 3, 5, 6],
            vec![1, 3, 5, 6],
            vec![1],
        ];
        let target = [5, 2, 7, 0, 0];
        let res = [2, 1, 4, 0, 0];

        for i in 0..nums.len() {
            assert_eq!(search_insert(nums[i].clone(), target[i]), res[i])
        }
    }
}

// 执行用时：0 ms, 在所有 Rust 提交中击败了100.00% 的用户
// 内存消耗：2.3 MB, 在所有 Rust 提交中击败了5.71% 的用户