# Day 21: Dirac Dice

This one gave me a lot of trouble, so much so that I had to get a hint to make sure I was on the right track for part 2. Part 1 was simple, which I knew was setting up for a very difficult second part. Most of the trouble came from serializing the game state and memoizing it. Once I got that down, I then made a bunch of errors reading the instructions (the score to reach changed?!?). Overall, I spent way too much time on this day.

I one again had to use my counter from `common`, but had to remake it with `u128` counts. I finally implemented `IntoIterator` and `FromIterator` for the first time, and even did two different implementations of `FromIterator`! That, at the very least, redeemed my performance on this day. Otherwise, this one was surprisingly rough.