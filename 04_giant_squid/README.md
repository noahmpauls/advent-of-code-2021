# Day 4: Giant Squid

Beyond some annoying parsing, this problem was fairly straighforward. One interesting Rust quirk came when implementing part 2. I wanted to iterate through the list of boards to mutate them, and then I wanted to store an immutable reference to the last board that had a bingo:

```rust
    let mut boards: Vec<BingoBoard> = //...

    let mut last_bingo_board = None;

    for mark in to_mark {
        for board in boards.iter_mut() {  // <- iterate through mutable refs
            if !board.bingo() && MarkResult::Bingo == board.mark(mark) {
                last_bingo_board = Some(&*board);  // <- store immutable ref
            }
        }
    }

    // use last_bingo_board
```

But this doesn't compile:

```
error[E0499]: cannot borrow `boards` as mutable more than once at a time
   --> 04_giant_squid\src\main.rs:101:22
    |
101 |         for board in boards.iter_mut() {
    |                      ^^^^^^ `boards` was mutably borrowed here in the previous iteration of the loop

For more information about this error, try `rustc --explain E0499`.
error: could not compile `giant_squid` due to previous error
```

I can't keep an immutable reference at the same time as a mutable reference. Due to my program logic, I know for a fact that the board will not be mutated after a bingo, but the borrow checker doesn't know that. This would most likely require using `Rc` or `RefCell` to fix, but I found a simpler solution that stores only the unmarked cells that I need.