# Day 20: Trench Map

This day came with a twist. Images of infinite size will have a lot of squares like this:

```
...
...
...
```

If index 0 of the enhancement algorithm string was `#`, every square like that would need to be inverted, leading to an infinite amount of lit pixels! I falsely assumed that the input enhancer string would always keep such blocks unlit, but I was very very wrong. My data structure for solving this tracked only lit pixels at first, so I wasn't sure how to handle an infinite number of them.

The solution is to track either light or dark pixels depending on whether a step "inverted" empty (or full) pixel blocks. If unlit blocks get inverted, there are a finite amount of unlit pixels to track. If lit blocks get inverted again, there are once again a finite number of lit pixels to track. This was confusing to implement due to my variable names at times, but the methods I used to abstract the functionality in my struct really helped make this much easier than it would have otherwise been.

I had to use `move` for the first time in a closure:

```rust
    pub fn read_adj(&self) -> impl Iterator<Item=Coord> {
        let r = self.r;
        let c = self.c;
        (-1..=1).flat_map(move |dr| {
            (-1..=1).map(move |dc| {
                Coord::new(r + dr, c + dc)
            })
        })
    }
```

The above function returns an iterator, which means the closures inside the iterator aren't evaluated immediately. `r` and `c` are borrows of my `Coord` struct's fields, so they have to be moved into the closures so that the closures own the values. Furthermore, I have to assign the fields to new values (instead of using `self.r/c` directly) because otherwise I would need to move the entire `self`, which is not what I want to do. It's all about ownership and lifetimes.