# Day 17: Trick Shot

I am not very proud of today's solution; I think it's pretty clunky overall. The main flaw is that my solution is built specifically for an input located in the `+x, -y` quadrant, and can't handle any other situations. My code disallows creating any targets outside of that quadrant.

There are a lot of different properties of the probe's movement that make its trajectory predictable enough to make calculations on. Part 1 turned out to be very simple after realizing that a projectile launched upward will pass through each `y` level twice, once with `y` velocity `dy` and once with `-dy`. We can use that to calculate a bound on the largest starting `dy` value: `dy + 1` is greater than the `y` displacement between the start and the bottom of the target, the probe will overshoot at every `dy` larger than that.

Similar properties can be used to put bounds on the `dx` and `dy` values used for part 2. But again, my solution isn't generalized, and I'm not a huge fan of today's problem.

One interesting thing I used is a `Range` type (`min..=max`), which is more powerful than I realized.