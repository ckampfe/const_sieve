fn main() {
    use const_sieve::*;
    let x = primes::<20000>();
    println!("here: {:?}", x);
}
