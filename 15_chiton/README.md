# Day 15: Chiton

Today's problem was just an implementation of [Dijkstra's Algorithm][dij] for finding the shortest path through a graph of weighted edges. In this case, nodes are coordinates in the grid and edges are the risk level taken on when moving to a new space on the grid. Runtime of the algorithm is optimized by using a priority queue; Rust doesn't provide a useful priority queue natively (`Binary Heap` is woefully lacking in functionality), so I used the amazing [`priority_queue` crate][pq].

I wrote some absolutely awful code for part 2, where the grid had to be tiled and transformed. As usual, I could do it better with a better knowledge of iterators. But my solution works!


[dij]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
[pq]: https://docs.rs/priority-queue/latest/priority_queue/