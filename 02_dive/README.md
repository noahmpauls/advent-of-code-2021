# Day 2: Dive!

Today's solution might be *slightly* overengineered, but oh well. The main point of interest was implementing the Add binop on a custom type (Position) so that I could sum a series of positional changes using an iterator.

I almost created a set of custom coordinate types in `common` that can handle any primitive integer type, but that would be some insane overkill. In the process, I did find the [`num` crate][num], which looks fascinating. Something I'll have to check out in the future.

[num]: https://docs.rs/num/latest/num/