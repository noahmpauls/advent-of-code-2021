# Day 7: The Treachery of Whales

Rust iterators make for some incredibly compact code. The following two code blocks are almost functionally equivalent (except the latter is actually more robust):

```rust
let mut min_fuel = i32::MAX;

for p in 0..=max_crab {
    let cost = crabs.iter().map(|v| (v - p).abs()).sum();
    if cost < min_fuel {
        min_fuel = cost;
    }
}
```

```rust
let min_fuel: i32 = (0..=max_crab).map(|p| {
    crabs.iter().map(|v| (v - p).abs()).sum()
}).min().unwrap();
```

Today was pretty easy overall! It helped to know that there's a nice equation for the sum of integers 1 through n.