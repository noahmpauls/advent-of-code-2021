# Day 3: Binary Diagnostic

Some fun problems with bit-shifting and clever iterator usage. The resulting code isn't the best, but it does the job.

- `u32::from_string_radix` is a fun function that I think all primitive integers have that allows parsing from binary, octal, hex, etc.
- You can `.collect()` an iterator of `char`s into a `String`.

`.unzip()` looks like a really fun function under the right circumstances. I though about using it to create strings for gamma and epsilon at the same time, like so (`main.rs` ln. 58):

```rust
    let (gamma, epsilon): (Vec<char>, Vec<char>) = counters.iter().map(|counter| {
        if counter.count('0') > counter.count('1') {
            ('0', '1')
        } else {
            ('1', '0')
        }
    }).unzip();

    // gives (
    //   vec!['1', '0', '1', '1', '0'], 
    //   vec!['0', '1', '0', '0', '1']
    // )
```

The trouble comes with collecting the result of the unzipping to `String`s and then to integers in a clean way. I decided that parsing only gamma and then bit-shifting to get epsilon was faster. But unzip is really, really cool, and I want to use it sometime.