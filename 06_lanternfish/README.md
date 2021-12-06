# Day 6: Lanternfish

This was an optimization problem, which part 1 hints at with the "exponential" growth of the lanternfish. Unfortunately, I took the bait and modeled each fish individually in part 1, which could only end in disaster once the number of fish started reaching the millions, billions, and trillions.

Luckily, all fish of the same age act the same way, so the minimum state needed is the counts of how many fish of each timer value there are. Thus, despite exponential growth of the fish school, each day can be calculated in a constant amount of time!

In part 2, I used an array to store the counts of fish at each age. Since there were only 9 possible ages, I used a fixed length array:

```rust
pub struct LanternfishSchool {
    counts: [u128; 9],
    //...
}
```

What I found interesting was that Rust treats arrays with explicit sizes and non-sized arrays differently:

```rust
impl LanternfishSchool {
    pub fn from(timers: &Vec<u8>) -> LanternfishSchool {
        let mut counts = [0; 9];
        // let mut counts = [u128]; <- will not compile
        //...

        LanternfishSchool { counts, z: 0 }
    }
    //...
}
```

I think array types (and slices in general) still elude me in Rust. I use `Vec` a lot, but I'm not sure if I could perhaps be using arrays or slices, and what the difference between the two is. After completing the solution, I converted a method that takes a `&Vec<u8>` to take a `&[u8]` instead, as that seems more generalized (similar to methods that take `String`s using `&str` as parameters instead).

One other Rust thing I'd forgotten: type-hinting doesn't have to be totally explicit, such as when collecting an iterator. The following are the same:

```rust      
let nums: Vec<u8> = "1, 2, 3, 4"
//  explicit ^^^^
    .split(',')
    .map(|num| num.parse::<u8>().unwrap())
    .collect();
```

```rust
let nums: Vec<_> = "1, 2, 3, 4"
//  implicit ^^^
    .split(',')
    .map(|num| num.parse::<u8>().unwrap())
    .collect();
```