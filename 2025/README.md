# Advent of Code 2025

The folder contains my personal solutions for Advent of Code 2025.

This year, I wrote my solutions in Rust. I wanted to practice it as I haven't used it in a while.

- Install Rust Stable 1.91 or higher
- Run `cargo run --bin dayX -- path/to/input.txt`

## Notes

I used hints on these problems:

- Day 9 Part 2
- Day 10 Part 2
- Day 12 Part 1

Everything else was solved without looking at any hints / sample answers.

### Day 1

Got confused by the wrapping addition/subtraction LOL. But otherwise it was OK.

### Day 2

Definitely picked a strange way to solve it... think I should've done it by chunk size instead.

### Day 3

No issues with part 1 or part 2!

### Day 4

Should've used loops to access the surrounding cells...

I ran into ownership issues in this problem. I wanted to modify the grid's adjacency counts as I was iterating over it, but Rust didn't like how I was retrieving some cells immutably and another cell mutably. Changing the data structure helped, though I'm sure there's a better approach.

### Day 5

My guess for part 2 was correct, so I wrote the merging logic ahead of time.

... until I found out that my merging logic was wrong LOL. I didn't handle cases like `100-110` and `103-107` correctly, since I only checked if range 1's start/end were contained by range 2. So I kept getting answers that were too large as there were extraneous ranges.

### Day 6

Problem was easy to understand, though it took me a while to figure out how to implement part 2.

### Day 7

Part 1 was fine. The index-based iteration I used here may have helped with Day 4's ownership issues.

Part 2 took me a while to figure out - had trouble correlating the total path count with the possible paths generated in Part 1.

### Day 8

Got confused by part 1 - the example used the 10 shortest edges (i.e. box count / 2) while the main input used the 1000 shortest edges (i.e. box count). But after realising this, it was quite straightforward.

My merging function isn't very efficient though... every time I remove the 1-2 matching circuits, it has to shuffle the elements to form a contiguous list. So it could be improved by changing the data structures used.

### Day 9

Part 1 is fine - brute force is enough to solve it.

But I have no idea how to solve part 2... not very good at this type of problem. Ended up brute forcing again using the [`geo`](https://crates.io/crates/geo) crate to check if the rectangle was inside the input polygon.

### Day 10

Feels like Day 9 all over again...

This is the first day where I found manual parsing to be too tedious, so I added [`regex`](https://crates.io/crates/regex) to help me out. Other languages come with regex in their standard library, so I'd say this is perfectly fine.

Part 1 was solvable with breadth-first search. Part 2 can technically be solved with the same approach, but it's too slow and eats up several gigabytes of memory. Out of curiosity, I tried solving each machine input in parallel (16 cores / 32 threads) and it went through my PC's 32 GB of RAM within seconds haha.

I eventually solved part 2 using linear programming, though I passed it to the [`microlp`](https://crates.io/crates/microlp) crate for the actual solution. Apparently, I've never used linear programming before? Or at least I can't remember anything about it from university...

### Day 11

I initially solved part 1 with basic BFS, but as expected, it's too memory intensive for part 2. Then I adjusted the path counting algorithm so it works in a memoized manner, making it much more efficient to run.

### Day 12

I was concerned when I saw it was a packing problem with irregular shapes and overlaps were allowed... But after looking at hints, the expected solution was kinda anti-climactic given how complicated the example was. Shame this was the last problem for this year.
