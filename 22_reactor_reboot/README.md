# Day 22: Reactor Reboot

This day was, as usual these days, surprisingly tough. This day followed the pattern of being the same problem, but majorly scaled up for part 2. I didn't know how to create the optimized solution, so I used the naive solution of tracking every individual cube. Predictably, this proved to be not efficient enough for part 2.

For part 2, I created methods to fracture a cuboid based on its overlap with another cuboid. Then I could create a set of non-overlapping cuboids from a set of overlapping cuboids, and calculate the volume of the non-overlapping set. I was banging my head after my tests continually failed, until I realized that *instruction order matters*.

I used the `RangeBounds` trait to create cuboids from a variety of different range types. I'm not entirely happy with the cuboid constructors, and they definitely need written documentation to be understood. I would love to attempt a struct that has multiple `new` constructors using `impl` blocks with different types.