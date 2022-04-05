fn main() {
    println!("{}", sum_squares_difference(100));
}

fn sum_squares_difference(limit: usize) -> usize {
    let sum_squares = (1..=limit).fold(0, |acc, i| acc + i * i);
    let sum = (1..=limit).sum::<usize>();
    let square_sums = sum * sum;

    square_sums - sum_squares
}

#[cfg(test)]
mod p0003_tests {
    use super::*;

    #[test]
    fn find_answer_below_10() {
        assert_eq!(sum_squares_difference(10), 2640)
    }
}
