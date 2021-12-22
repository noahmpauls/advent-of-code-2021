# Day 19: Beacon Scanner

This day is basically Day 20 of AoC 2020. This time, though, I finally figured out that this is a matrix problem. Points in three-dimensional space can be rotated around the X, Y, and Z axes using [rotation matrices][rot]. I used the [`ndarray`][nd] crate for this, a crate that seems very powerful but is not one I want to mess with too much. I'm not a huge matrix person.

This was a time-consuming day, but I'm glad it worked practically first try.

I read [a great piece about iterators][iter] that helped me understand `iter()` vs `into_iter()`, and help clear up some confusion with nested `for` loops. Speaking of loops, the Rust compiler is incredibly smart:

```rust
'outer: for (i, scanner) in remaining.iter().enumerate() { // remaining borrowed immutably
    for perm in scanner.perms() {
        for (placed, position) in placements.iter() {
            if let Some(delta) = placed.overlaps(&perm, overlap_min) {
                placement = Some((perm, position.translate(&delta)));
                remaining.remove(i); // remaining borrowed mutable
                break 'outer;
            }
        }
    }
}
```

The borrow checker knows all. If the `break` directive is removed, the `remaining.remove()` throws a compiler error because the `remaining` array is still being borrows as immutable in future loop iterations. But wouldn't you know it, Rust can tell that the immutable borrow doesn't need to last after the `for` loop is broken! That is actually amazing.

[rot]: https://en.wikipedia.org/wiki/Rotation_matrix#In_three_dimensions
[nd]: https://crates.io/crates/ndarray
[iter]: https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html