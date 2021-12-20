# Day 18: Snailfish

This day was rough, and I have officially fallen behind by a day because of it.

Originally I tried to represent SnailNums using a recursive data type. Because reducing requires a lot more global knowledge of an entire snail num than just a single regular or pair can have, though, I decided to make a simpler `Vec`-based rep that models brackets with gains/decreases in bracket count per num. Using some nifty properties of that representation, I could do an explode/split at any index.

My only trouble came when I misunderstood how the rules are applied. I did not realize that you MUST explode any pair that can be exploded before attempting to split. The many bugs I ran into resulted in a massive suite of tests, so that was nice.

This day used a `check_rep`, which was a life-saver. Using it in the constructor was weird at first, but now I know this pattern:

```rust
    pub fn new(string: &str) -> Thing {
        let fields = // make fields...;

        let result = Thing { fields };
        result.check_rep();

        result
    }

    fn check_rep(&self) {
        // check fields
    }
```

I also wrote a small `Tokenizer`. It's no good yet; it isn't memory efficient because it requires the entire string being tokenized to be loaded into a `Vec<char>`. Using iterators to implement a memory-efficient tokenizer would be a very interesting problem for the future.