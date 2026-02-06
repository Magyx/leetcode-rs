// LOCAL
pub struct Solution;
// LOCAL END

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        let mut nums2 = nums2;
        nums.append(&mut nums2);
        nums.sort();

        if nums.len() % 2 == 0 {
            (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) as f64 / 2f64
        } else {
            nums[nums.len() / 2] as f64
        }
    }
}
