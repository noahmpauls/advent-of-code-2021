# Day 14: Extended Polymerization

The low iteration count in part 1 should have been a tip-off that this would be a day that punishes a naive solution for part 1 by using a ton of memory in part 2. But in the process of creating my naive part 1 solution, I discovered `flat_map` and `chain`, which are more amazing iterator functions.

I used my `Counter` struct, but unfortunately had to make a copy of it for backwards compatibility since my common `Counter` counts in `u32` values, which weren't large enough for this day. Blast it. However, I did add some interesting methods that return iterators. Returning iterators is incredibly easy, it turns out, and I should definitely return them more often.