pub fn binary_search<T>(list: Vec<T>, item: T) -> Option<usize>
where
    T: PartialEq + PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let guess = list[mid];

        if guess == item {
            return Some(mid);
        } else if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
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
        assert_ne!(binary_search(list, item), None);
    }

    #[test]
    fn test_u32() {
        let list: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let item = 8 as u32;

        let result = binary_search(list, item);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_i32() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 0;

        let result = binary_search(list, item);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_chars() {
        let list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let item = 'd';

        assert_eq!(binary_search(list, item), Some(3));
    }
}
