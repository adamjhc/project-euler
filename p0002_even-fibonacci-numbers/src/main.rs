fn main() {
    println!("{}", sum_even_valued_fibonacci_numbers());
}

fn sum_even_valued_fibonacci_numbers() -> usize {
    let mut fibonacci_number_prev = 1;
    let mut fibonacci_number = 2;
    let mut sum = 0;
    loop {
        if fibonacci_number > 4_000_000 {
            break;
        }

        if fibonacci_number % 2 == 0 {
            sum += fibonacci_number;
        }

        let tmp = fibonacci_number;
        fibonacci_number = fibonacci_number + fibonacci_number_prev;
        fibonacci_number_prev = tmp;
    }

    sum
}
