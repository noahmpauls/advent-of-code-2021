# Day 5: Hydrothermal Venture

The solution suggested by this problem is to keep track of a grid of integer points. This method is incapable of solving a more generalized line intersection problem, but this problem maintains some key invariants about the line segments being considered that allow this solution to be used.

The solution to this problem relies heavily on the special properties of the input:
- start and end of line segments are integer coordinates
- slope of line segments is either horizontal, vertical, or +/-1

This means that each line segment has a set of integer coordinates it passes through. Two lines that pass through the same integer coordinates intersect at those coordinates. I used an occurrence counter to check how many times a given coordinate appears in all the lines' integer coordinates and found the number of coordinates that occurred 2 times or more.

The input actually follows one other interesting property that isn't necessarily true given the two conditions on the segments: all intersections happen at integer coordinates. Consider the following input:

```
0,0 -> 1,1
0,1 -> 1,0
```

These two lines have integer coordinates defining them and have slopes of +1 and -1, respectively. But their intersection is at (0.5, 0.5)! My solution would not have considered this intersection,but still got a correct answer. These sorts of situations must be disallowed in the input, and if two diagnoal lines intersect, they must do so at integer coordinates. 