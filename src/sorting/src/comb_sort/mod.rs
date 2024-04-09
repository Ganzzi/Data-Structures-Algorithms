const SHRINK_FACTOR: f32 = 1.3;

pub fn comb_sort(nums: &mut [i32]) {
    let len = nums.len();
    let mut gap = len;

    while gap > 1 {
        gap = (gap as f32 / SHRINK_FACTOR) as usize;

        for i in gap..len {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
        }
    }
}
