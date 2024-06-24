// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // let leek_v = Box::leak(v.into_boxed_slice());
    let leek_v = v.leak();

    let break_index = leek_v.len() / 2;
    let (v1, v2) = leek_v.split_at(break_index);

    let handle1 = thread::spawn(move || { v1.iter().sum() });
    let handle2 = thread::spawn(move || { v2.iter().sum() });

    let result1: i32 = handle1.join().unwrap();
    let result2: i32 = handle2.join().unwrap();

    result1 + result2

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
