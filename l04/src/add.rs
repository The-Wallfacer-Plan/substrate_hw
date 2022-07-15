#[allow(dead_code)]

pub fn add_optional(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in nums.iter() {
        let sum_optional = sum.checked_add(num);
        match sum_optional {
            Some(s) => {
                sum = s;
            }
            None => {
                return None;
            }
        }
    }
    return Some(sum);
}


#[cfg(test)]

mod tests {
    use super::add_optional;
#[test]
    fn add_test_success() {
        let nums1 = vec![1, 2, 3, 4, 20485];
        assert_eq!(add_optional(&nums1), Some(20495));
    }

#[test]
    fn add_test_fail() {
        let nums1 = vec![1000, 999, 987, 1, 4_294_967_000u32];
        assert_eq!(add_optional(&nums1), None);
    }

}
