pub fn find_smallest(array: &[i32]) -> usize {
    let mut smallest = array[0];
    let mut sm_idx = 0;

    for (i, &item) in array.iter().enumerate() {
        if item < smallest {
            smallest = item;
            sm_idx = i;
        }
    }

    sm_idx
}

pub fn selection_sort(mut array: Vec<i32>) -> Option<Vec<i32>> {
    if array.is_empty() {
        return None;
    }
    let mut new_array = Vec::new();

    while !array.is_empty() {
        let smallest_idx = find_smallest(&array);
        new_array.push(array.remove(smallest_idx));
    }

    Some(new_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let list = vec![5, -100, 6, 2, 100, 0];
        let result = selection_sort(list);
        assert_eq!(result, Some(vec![-100, 0, 2, 5, 6, 100]));
    }

    #[test]
    fn test_empty() {
        let list = Vec::new();
        let result = selection_sort(list);
        assert_eq!(result, None);
    }
}
