
pub fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();

    if 2 > len{
        return;
    }

    for i in 1..len {
        for j in 0..len - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j,j + 1);
            }
        }
    }
}


pub fn bubble_sort2(nums: &mut [i32]) {
    let mut unordered = true;
    let mut len = nums.len() - 1;

    while len > 0 && unordered {
        unordered = false;

        for i in 0..len {
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
                unordered = true;
            }
        }

        len -= 1;
    }
}
