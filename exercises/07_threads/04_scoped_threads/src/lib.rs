// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let break_index = v.len() / 2;
    let (v1, v2) = v.split_at(break_index);

    std::thread::scope(|scope| {
        let handle1 = scope.spawn(|| { v1.iter().sum() });
        let handle2 = scope.spawn(|| { v2.iter().sum() });

        let result1: i32 = handle1.join().unwrap();
        let result2: i32 = handle2.join().unwrap();

        result1 + result2
    })

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
