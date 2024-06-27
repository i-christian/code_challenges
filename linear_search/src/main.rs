fn linear_search(list: Vec<i32>, item: &i32) -> Option<usize> {
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

fn main() {
    let list = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let item = 15;

    let result = linear_search(list, &item);
    if let Some(val) = result {
        println!("The item: {} is on position: {}", item, val);
    } else {
        println!("{} not found in the list", item);
    }
}
