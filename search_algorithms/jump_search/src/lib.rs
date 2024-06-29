pub fn jump_search(sorted_list: Vec<i32>, target: i32) -> Option<usize> {
    if sorted_list.is_empty() {
        return None;
    }

    // calculate the `jump step` based on the square root of the list length
    let mut current_index = 0;
    let jump_step = (sorted_list.len() as f32).sqrt() as usize;
    let mut next_index = jump_step;

    // Jump through the list in blocks of size `jump_step`
    // If the current element is less than the target, it moves to the next block.
    while current_index < sorted_list.len() {
        if sorted_list[current_index] == target {
            return Some(current_index);
        } else if sorted_list[current_index] < target {
            current_index = next_index;
            next_index += jump_step;
            if next_index >= sorted_list.len() {
                next_index = sorted_list.len();
            }
        } else {
            // Perform a linear search in the block
            for i in (current_index - jump_step)..current_index {
                if sorted_list[i] == target {
                    return Some(i);
                }
            }
            break;
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
        assert_eq!(jump_search(list, item), None);
    }

    #[test]
    fn test_one_item() {
        let list = vec![0];
        let item = 0;
        let result = jump_search(list, item);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_i32() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 0;

        let result = jump_search(list, item);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_i32_not_found() {
        let list: Vec<i32> = vec![-10, -5, 0, 5, 10];
        let item = 7;

        let result = jump_search(list, item);
        assert_eq!(result, None);
    }
}
