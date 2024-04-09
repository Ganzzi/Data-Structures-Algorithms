pub fn cocktail_sort(nums: &mut [i32]) {
    let len = nums.len();

    let mut c = 0;
    let mut bubble = true;

    while c < (len >> 1) && bubble {
        bubble = false;

        // bubble L to R
        for j in c..(len - c - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                bubble = true;
            }
        }

        // bubble R to L
        for j in (c + 1..=(len - c - 1)).rev() {
            if nums[j] < nums[j - 1] {
                nums.swap(j - 1, j);
                bubble = true
            }
        }

        c += 1;
    }
}
