fn main() {
    println!("{}", sum_of_multiples_of_3_or_5_below(1000))
}

fn sum_of_multiples_of_3_or_5_below(limit: usize) -> usize {
    (0..limit).fold(0, |acc, i| {
        if i % 3 == 0 || i % 5 == 0 {
            acc + i
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod p0001_tests {
    use super::*;

    #[test]
    fn find_answer_below_10() {
        assert_eq!(sum_of_multiples_of_3_or_5_below(10), 23)
    }
}
