pub fn quicksort(mut array: Vec<i32>) -> Vec<i32> {
    // Base case covers [] and [1]
    if array.len() < 2 {
        return array;
    } else {
        let pivot = array.remove(0);
        let mut less = Vec::new();
        let mut greater = Vec::new();

        // populate arrays with values on the left and right side of the pivot
        for i in array {
            if i <= pivot {
                less.push(i);
            } else {
                greater.push(i);
            }
        }

        // reassemble the array
        let sorted = vec![quicksort(less), vec![pivot], quicksort(greater)].concat();
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let list = vec![5, -100, 6, 2, 100, 0];
        let result = quicksort(list);
        assert_eq!(result, vec![-100, 0, 2, 5, 6, 100]);
    }

    #[test]
    fn test_empty() {
        let list = Vec::new();
        let result = quicksort(list);
        assert!(result.is_empty());
    }
}
