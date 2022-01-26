# Day 23: Amphipod

After a long break, I finally sat down and completed what I feel is one of the more daunting problems so far in AoC 2021. What makes this one look challenging initially is how oddly shaped the input is. Other problems use grids, arrays, or other formats that fit well with how a computer processes info, but this problem uses a hallway with oddly placed rooms. Not a very intuitive format, especially with the movement rules stacked on top (although the rules ultimately make the problem easier).

Ultimately, my solution is as custom as the input format. I created a `Burrow` struct that uses a `Hallway` struct and four `Room` structs; all structs are immutable. `Hallway` and `Room` have their own methods for creating new versions of themselves by moving amphipods in and out of them, and they make sure to check whether movements and resulting states are possible and valid. `Burrow` manages the full interaction between the hallway and the rooms, and has the express purpose of finding the optimal path to the solved state.

Here's the basic algorithm I used to find the optimal solution:

```
find_min_energy(burrow, energy_so_far):
    if all amphipods are in their rooms:
        return energy_so_far
    
    energies = []
    for amphipod in hallway:
        for each room:
            if amphipod can move from hallway to room:
                next_burrow, added_energy = move(amphipod, hallway, room)
                energies += find_min_energy(next_burrow, energy_so_far + added_energy)
    
    for amphipod in room:
        for each hallway space:
            if amphipod can move from room to hallway:
                next_burrow, added_energy = move(amphipod, room, hallway)
                energies += find_min_energy(next_burrow, energy_so_far + added_energy)

    return min(energies)
```

Implementing this solution alone was very slow. I added two optimizations:
- Throughout the algorithm, keep track of the known miniumum score so far (`known_min`). Don't run `find_min_energy` if `known_min >= energy_so_far + added_energy`.
- Memoize each burrow state, keeping track of the minimum `energy_so_far` value when that state is reached. If the memo contains a lower energy value than the current `energy_so_far`, the algorithm has already found a more optimal path and doesn't need to continue.

Overall, I am very happy with this solution code. I wouldn't call it the most elegant, but I think it is tight.

## Learnings:

### Fixed size array initialization is annoying sometimes

A burrow has 4 rooms, and a hallway has 11 spaces. These are excellent candidates for fixed size arrays, and I did use these in places throughout my code. However, this can be a bit of a nightmare if the initialization of the array is programmatic. When I made the rooms in `Burrow`, for instance, this would have been nice:

```rust
pub fn new(rooms: &[Vec<Amphipod>]) -> Self {
    let rooms: [Rc<Room>; 4] = [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert]
        .into_iter().enumerate()
        .map(|(i, a)| Rc::new(Room::new(a, &rooms[i])))
        .collect();
}
```

But iterators can't really be made into fixed-size arrays, so I had to settle for this:

```rust
pub fn new(rooms: &[Vec<Amphipod>]) -> Self {
    let rooms: [Rc<Room>; 4] = [
        Rc::new(Room::new(Amphipod::Amber, &rooms[0])),
        Rc::new(Room::new(Amphipod::Bronze, &rooms[1])),
        Rc::new(Room::new(Amphipod::Copper, &rooms[2])),
        Rc::new(Room::new(Amphipod::Desert, &rooms[3])),
    ];
}
```

I feel sure there's a better way, but I haven't discovered it.

### Finally using smart pointers (`Rc`)

When a move gets made, only the hallway and one room are effected. The other three rooms stay exactly the same. Since every copy of a `Burrow` is immutable, it makes sense for `Burrow`s with the same rooms to share the `Room` objects. This is where smart pointers are needed, since otherwise there can only be one owner for each `Room`. A reference count (`Rc`) pointer made the most sense: we didn't want to actually mutate any of the rooms, just allow each `Burrow` the ability to look at the immutable room. It's super nice to have the increment of the reference count be built into `clone` and `drop` (although technically it's not best practice to use `clone` on an `Rc`).

### Peeking value of `Option`

...is difficult. I used `Option`s to store whether an amphipod was at a given place in a hallway or room. I often needed to compare an amphipod against a space that I knew was occupied, but that essentially meant unwrapping the value in the option, comparing, and then replacing the option back.

Except that I didn't seem to need to do the replacement after all. The behavior of `unwrap()` at first confused me, because it says it consumes `self`, but I was able to unwrap `Option<Amphipod>` and then reuse the same `Option` again. This is because `Option` can be reused after unwrapping if it fully implements `Copy`. The following example makes this clear:

```rust
let a = Some(10);
let b = a.unwrap();  // `a` is consumed
println!("{:?}", a); // `a` can be copied, so this is actually printing a copy

let c = Some(vec![]);
let d = c.unwrap();  // `c` is consumed
println!("{:?}", c); // `c` cannot be copied, so this generates an error
                     //   for a borrow of a moved value
```

With this in mind, it makes sense not to use `unwrap()` to look at an `Option`'s value if the plan is to reuse that `Option` later, as such code doesn't make clear what's actually going on.

Using `map`, `map_or`, or `map_or_else` might be a better way to handle this. Specifically, if called on a `Some`, `map_or` should return a clone of the value inside. Or you can explicitly use `clone()` on the `Option` itself. I'm honestly not sure what the best way to do this is. Maybe I need to use something different than `Option`.

### `fold`, `all`, and `any`

I originally used `map` and `fold` to test an iterator of booleans. Turns out, `all` and `any` already do that:

```rust
self.rooms.iter()
    .map(|r| r.is_complete())
    .fold(true, |a, b| a && b)

// ...is equivalent to...

self.rooms.iter()
    .all(|r| r.is_complete())


(depth + 1..self.size).into_iter()
    .map(|i| !self.is_native(i))
    .fold(false, |a, b| a || b)

// ...is equivalent to...

(depth + 1..self.size).into_iter()
    .any(|i| !self.is_native(i))
```

### Returning an `impl Iterator` using lifetimes

Can't remember whether I've done this before or not, but it felt new this time around when I created methods on structs that return `Iterator`s. Since the returned iterators make references to parts of the struct state, Rust has to guarantee that those references are valid while the iterator is in use. This is where lifetimes come in: if we specify that the lifetime of the bound iterator is at most that of `self`, Rust is happy. This is exactly what this method on `Hallway` does:

```rust
    pub fn _occupied_by(&'_ self, amphipod: Amphipod) -> impl Iterator<Item=usize> + '_ {
        self.spaces.iter().enumerate().filter_map(move |(i, a)| {
            if let Some(occupant) = a {
                if *occupant == amphipod {
                    return Some(i)
                }
            }
            None
        })
    }
```

The anonymouse lifetime (`'_`) bounds the lifetime of the returned `Iterator` to the lifetime of `self`. This use of lifetimes actually makes sense to me, which is awesome!

### The Serde Crate

[`Serde`](https://serde.rs/) is very cool. I used it to serialize the state of the `Burrow` when memoizing it. I'll have to explore it more; the coolest thing about it is that it uses its own intermediate representation between Rust structs and whatever format it's being serialized to. This means that it is incredibly easy to create a Serde serializer/deserializer for your own custom format by dealing with Serde's simple intermediate representation!

I also learned about a myriad of different formats for serialization, including Bincode and MessagePack.
