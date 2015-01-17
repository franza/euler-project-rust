// Largest prime factor
// Problem 3
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

use std::iter::count;

struct EratosthenesStream<'a> {
  sieve: &'a (Iterator + 'a)
}

impl<'a> EratosthenesStream<'a> {
  fn new() -> EratosthenesStream<'a> {
    EratosthenesStream { sieve: &count(2, 1) }
  }
}

impl<'a> Iterator for EratosthenesStream<'a> {
  type Item = isize;

  fn next(&mut self) -> Option<isize> {
    let prime = self.sieve.next().expect("Out of numbers");
    self.sieve = self.sieve.filter(|&x| x % prime == 0);
    Some(prime)
  }
}

fn main() {
  // let sieve = EratosthenesStream::new();
  // for i in sieve.take(5) {
  //   println!("{}", i);
  // }
}
