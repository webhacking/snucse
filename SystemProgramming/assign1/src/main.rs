#![feature(std_misc)]

extern crate time;

use time::PreciseTime;

fn test(stride: usize, size: usize) {
    let a = vec![b'a'; size];
    let mut b = vec![0u8; size];

    let t0 = PreciseTime::now();

    // Sequential Access
    let mut idx: usize = 0;
    while idx < size {
        b[idx] = a[idx];
        idx += 1;
    }

    let t1 = PreciseTime::now();

    // Random Access
    let mut base: usize = 0;
    while base < stride {
        let mut idx = base;
        while idx < size {
            b[idx] = a[idx];
            idx += stride;
        }
        base += 1;
    }

    let t2 = PreciseTime::now();

    let friendly = t0.to(t1).num_nanoseconds().unwrap();
    let unfriendly = t1.to(t2).num_nanoseconds().unwrap();
    let ratio = friendly as f64/unfriendly as f64;

    println!("{:>5}, {:>10}, {:>20}, {:>20}, {:>20}", stride, size, friendly, unfriendly, ratio);
}

fn main() {
    println!("{:>5}, {:>10}, {:>20}, {:>20}, {:>20}", "stride", "size", "friendly", "unfriendly", "unfriendly / friendly");
    for stride in 1usize..64 {
        for exp in 11..25 {
            for _ in 0..5 {
                test(stride, 2 << exp)
            }
        }
    }
}