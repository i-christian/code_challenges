pub fn linear_search<T: PartialEq>(list: Vec<T>, item: &T) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    let mut guess = 0;
    while guess < list.len() {
        if list[guess] == *item {
            return Some(guess);
        }
        guess += 1;
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
        assert_ne!(linear_search(list, &item), None);
    }

    #[test]
    fn test_u32() {
        let list: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let item = 8 as u32;

        let result = linear_search(list, &item);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_i32() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 0;

        let result = linear_search(list, &item);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_chars() {
        let list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let item = 'd';

        assert_eq!(linear_search(list, &item), Some(3));
    }
}
