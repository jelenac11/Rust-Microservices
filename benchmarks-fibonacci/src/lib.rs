#![feature(test)]
extern crate test;

use std::mem::replace;
use test::Bencher;
static BENCH_SIZE: usize = 20;

fn fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>())
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<u32>>())
}
