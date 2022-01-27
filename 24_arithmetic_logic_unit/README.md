# Day 24: Arithmetic Logic Unit

Ouch. This one hurt. Badly.

Creating an ALU was actually very easy. I like the way my implementation for that turned out.

Actually solving the problem, though, was really difficult. At first, I thought I could brute-force a solution as long as I used a memo. The idea is that any program can be divided up into blocks that start with an `inp` and end just before the next `inp` (or the end of the program). Then, we can perform a DFS through these blocks like so:

```
analyze_blocks(blocks, index, start_state, memo) -> Option<[digit]>

    for inputs x in (1..10).rev():
        if memo contains { index, start_state, input }:
            continue

        create an ALU with input x and start state s
        output = run blocks[index]
        if index + 1 >= blocks.len():
            if output.z == 0:
                return Some([input])
            else 
                memo += { index, start_state, input }
        else:
            if Some(digits) = analyze_blocks(blocks, index + 1, output):
                return Some(digits + input)
            else:
                memo += { index, start_state, input }

    return None
```

Starting with the first block, we feed in every possible input (9, 8, 7, .., 1) and a start state where all ALU registers are 0. From there, we feed the output of each initial input/state into the next block. When we reach the final block, we check if `z == 0`, then cascade back the digits if so.

This was still incredibly slow. The memo wasn't doing enough. After looking at the MONAD program (and getting a tiny hint), I altered the memo to track a subset of the state. In each block, register `w` only ever held the new input, and registers `x` and `y` were always reset before use. The only material register for each block is `z`; this meant I could memoize the block index, input, and start `z` state alone. Unfortunately, this means the solution isn't generalized for all possible programs; it is specific to MONAD.

This was *still* slow, but fast enough to get me an actual result. Running part 1 in release mode took ~85 seconds. Part 2 was slightly better for me; it took only ~5 seconds (due to the placement of the solution in the range). In total, it would take ~3-4 minutes to scan the entire possible number range.

Just for fun, `threaded.rs` contains a version of part 1 where I attempted to use multiple threads with a shared memo to break up the workload. Turns out the overhead of maintaining the shared memo outweighed the sharding by multiple threads. It's an interesting idea, though; maybe if I tried sharing less state I'd have more success.