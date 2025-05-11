#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

pub fn main() {
    let mut random_numbers = [8, -1, 5, 22, 0, -5, 3, 4, 1, 2, 9, 6, 7];

    random_numbers.sort();

    let result = binary_search(&random_numbers, 5);

    println!("{:?}", result);
    // return index from sorted array

    match result {
        Some((found_value, found_index)) => {
            println!("Found value {found_value} at {found_index}")
        }
        None => println!("Value not found"),
    }
}

fn binary_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len().saturating_sub(1);
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;

        let mid = (up_bound + low_bound) / 2;
        let mid_value = arr[mid];

        match mid_value.cmp(&target) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Greater => up_bound = mid.checked_sub(1)?,
            Ordering::Less => low_bound = mid + 1,
        }

        println!("Iteration: {i}");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_found() {
        let mut arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];
        arr.sort(); 
        assert_eq!((5, 2), binary_search(&arr, 5).unwrap());
    }

    #[test]
    fn element_not_found() {
        let mut arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];
        arr.sort();
        let result = binary_search(&arr, 1234);
        assert!(result.is_none());
    }

    #[test]
    fn smallest_element_not_found() {
        let mut arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];
        arr.sort();
        let result = binary_search(&arr, -100);
        assert!(result.is_none());
    }
}
