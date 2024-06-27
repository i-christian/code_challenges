fn binary_search(list: &Vec<i32>, item: i32) -> Option<usize> {
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

fn main() {
    let list = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    let item = 20;
    let result = binary_search(&list, item);

    if let Some(num) = result {
        println!("The value is at position: {}", num);
    } else {
        println!("Not found in the list");
    }
}
