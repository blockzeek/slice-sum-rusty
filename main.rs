fn sum(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;

    for num in nums.iter() {
        match sum.checked_add(*num) {
            Some(_) => {
                sum = sum + num;
            }
            None => return None,
        }
    }

    Some(sum)
}

fn main() {
    assert_eq!(sum(&[1u32, 2u32]), Some(3u32), "not overflow");
    assert_eq!(sum(&[1u32, u32::MAX]), None, "None when overflow");
}
