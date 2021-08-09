#!/usr/bin/env rust-script
//!```cargo
//![dependencies]
//!rand = "0.8.4"
//!```

const SHARDS: u64 = 64;
const NODES: u64 = 1000;
const ITERS: u64 = 1_000_000;

fn main() {
    let mut failures = 0u64;

    'outer: for _ in 0..ITERS {
        let mut nums_chosen = Vec::new();

        for _ in 0..NODES {
            let num: u64 = rand::random();
            let num = num % SHARDS;
            
            nums_chosen.push(num);
        }

        for i in 0..SHARDS {
            if !nums_chosen.contains(&i) {
                failures += 1;
                continue 'outer
            }
        }
    }

    println!("{} failures out of {}", failures, ITERS);
    println!(
        "{}%, success rate",
        (ITERS as f32 - failures as f32) / ITERS as f32 * 100f32,
    );
}
