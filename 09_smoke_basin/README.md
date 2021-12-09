# Day 9: Smoke Basin

A problem that requires grid-based neighbor finding. This would be a great candidate for a common tool, as these sorts of problems appear quite often in Advent of Code.

I used a `Box` for the first time, to return a variable length array:

```rust
fn neighbors(num: usize, max: usize) -> Box<[usize]> {
    match num {
        0 => Box::new([0, 1]),
        v if v == max => Box::new([max - 1, max]),
        v if v < max => Box::new([num-1, num, num + 1]),
        _ => Box::new([]),
    }
}
```

I was surprised that I didn't have to treat the returned `Box` any differently than the value it holds; that's the power of `Deref`.

Only gotcha for this problem was in part 2: a low point cannot have equal height neighbors, but members of a basin can. The example in the problem does not show a case where a member of a basin has an equal height to a neighbor, but those points are included. I wrote a test case that demonstrates this behavior.