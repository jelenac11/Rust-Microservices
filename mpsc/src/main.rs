extern crate rand;
extern crate futures;
extern crate tokio;

const BUFFER_SIZE: usize = 10;

use std::thread;
use futures::future::{Future, ok};
use futures::sync::mpsc;
use futures::{Sink, Stream};
use rand::{Rng, thread_rng};

fn main() {
    let (tx, rx) = mpsc::channel(BUFFER_SIZE);
    
    thread::spawn(move || {
        for _ in 0..10 {
            let mut rng = thread_rng();
            let random = rng.gen_range(0..100);
            match tx.clone().send(random).wait() {
                Ok(_) => println!("Sent {}", random),
                Err(_) => println!("Error happened"),
            };
        }
    });

    let sum = rx.fold(0, |acc, val| {
        ok(acc + val)
    }).wait().unwrap();
    println!("Calculated: {}", sum);
}