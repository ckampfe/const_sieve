pub fn primes<const N: usize>() -> Vec<usize> {
    let primes_array = sieve_of_eratosthenes::<N>();

    std::array::IntoIter::new(primes_array)
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i + 2) } else { None })
        .collect()
}

pub const fn sieve_of_eratosthenes<const N: usize>() -> [bool; N] {
    // true indicates "prime", false indicates "composite"
    let mut numbers = [true; N];

    let mut i: usize = 2;

    // for all numbers 2..=N
    while i <= N {
        // if the n at the current index is a prime
        if numbers[i - 2] {
            let mut multiplier = 0;
            let mut composite = compute_j(i, multiplier);

            while composite <= N + 1 {
                numbers[composite - 2] = false;

                multiplier += 1;
                composite = compute_j(i, multiplier);
            }
        }

        i += 1;
    }

    // special case handling in case the very last number is somehow prime
    if numbers[N - 1] {
        numbers[N - 1] = false;
    }

    return numbers;
}

const fn compute_j(i: usize, multiplier: usize) -> usize {
    i.pow(2) + (i * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(primes::<5>(), vec![2, 3, 5]);
    }

    #[test]
    fn it_works2() {
        assert_eq!(primes::<30>(), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn it_works3() {
        assert_eq!(primes::<31>(), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    }

    #[test]
    fn it_works4() {
        assert_eq!(primes::<1>(), vec![]);
    }

    #[test]
    fn it_works5() {
        assert_eq!(primes::<2>(), vec![2]);
    }
}
