fn main() {
    let nums1 = vec![1,2];
    let nums2 = vec![3,4];

    println!("{}", find_median_sorted_arrays(nums1, nums2));
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut sorted: Vec<i32> = [nums1, nums2].concat();
    sorted.sort();
    if sorted.len() % 2 == 1 {
        return sorted[sorted.len() / 2] as f64;
    } else {
        return ((sorted[sorted.len() / 2] + sorted[(sorted.len() / 2) - 1]) as f64) / 2.0;
    }
}
