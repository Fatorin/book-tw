// ANCHOR: here
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);
    // ANCHOR_END: here
    assert_eq!(result, 100);
    // ANCHOR: here

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);
    // ANCHOR_END: here
    assert_eq!(result, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
