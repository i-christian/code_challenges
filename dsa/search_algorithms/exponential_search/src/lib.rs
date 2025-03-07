pub fn exponential_search(list: Vec<i32>, target: i32) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    if list[0] == target {
        return Some(0);
    }

    let mut idx: usize = 1;
    let mut jump = idx.pow(2);

    while jump < list.len() && list[jump] <= target {
        idx += 1;
        jump = idx.pow(2);
    }

    // Perform binary search in the range [2^(idx-1), min(2^idx, list.len())]
    let mut low = 2usize.pow(idx as u32 - 1);
    let mut high = usize::min(jump, list.len() - 1);

    while low <= high {
        let mid = low + (high - low) / 2;
        if list[mid] == target {
            return Some(mid);
        } else if list[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let list: Vec<i32> = Vec::new();
        let item = 12;
        assert_eq!(exponential_search(list, item), None);
    }

    #[test]
    fn test_one_item() {
        let list = vec![0];
        let item = 0;
        let result = exponential_search(list, item);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_i32() {
        let list: Vec<i32> = vec![6, 11, 19, 24, 33, 54, 67, 81, 94, 99];
        let item = 67;

        let result = exponential_search(list, item);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_i32_not_found() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 7;

        let result = exponential_search(list, item);
        assert_eq!(result, None);
    }
}
