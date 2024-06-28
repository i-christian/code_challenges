pub fn interpolation(list: Vec<i32>, target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    if list.is_empty() {
        return None;
    }

    while low <= high && list[low] <= target && target <= list[high] {
        if low == high {
            if list[low] == target {
                return Some(low);
            } else {
                return None;
            }
        } else {
            // Interpolation search algorithm core formula
            let numerator = (target - list[low]) as usize * (high - low);
            let denominator = (list[high] - list[low]) as usize;
            let pos = low + numerator / denominator;

            if pos < low || pos > high {
                return None;
            }
            if list[pos] == target {
                return Some(pos);
            } else if list[pos] < target {
                low = pos + 1;
            } else {
                high = pos - 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty() {
        let list: Vec<i32> = Vec::new();
        let item = 12;
        assert_eq!(interpolation(list, item), None);
    }

    #[test]
    fn test_i32() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 0;

        let result = interpolation(list, item);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_i32_not_found() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 7;

        let result = interpolation(list, item);
        assert_eq!(result, None);
    }
}
