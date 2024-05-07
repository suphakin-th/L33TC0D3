impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut n = nums1;
        n.extend(&nums2);
        n.sort();
        let range = n.len();
        if range % 2 == 1 {
            return n[range / 2] as f64;
        } else {
            return (n[range / 2 - 1] as f64 + n[range / 2] as f64) / 2.0;
        }
    }
}