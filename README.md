# shortest_path

Investigating the 'travelling sales-person' problem.

Start from [plot repo](https://github.com/jinjagit/plot) and build to solve first by brute force.

## Brute-force solution

1. Generate all unique paths for given set of coords (that visits each point once && creates a loop that ends at start coord)
2. Calculate length of each path generated
3. Return shortest path as vec of given coords + the total length of this shortest path

Step 1 could generate the following permutations, given a set of coords `[a, b, c, d]`:
```
[a, b, c, d]
[a, b, d, c]
[a, c, b, d]
[a, c, d, b]
[a, d, b, c]
[a, d, c, b]
```

Which to me looks like all possible permuations of the subset at indices 1..n-1, with the element at index 0 remining in place in all resulting permutations.

And creating the full path would involve adding the start point to the end of each list.
For example `[a, d, b, c]` becomes `[a, d, b, c, a]`

But, actually, there are equivalent pairs of generated paths here (when we connect the last coord to the first). For example:
```
[a, b, c, d, a]
[a, d, c, b, a]
```
... are equivalent paths (just traversed in opposite directions):
```
a----------b
|          |
|          |
d----------c
```
I _think_ we just want half of the set of permutations. [Confirmed there are pairs of equivalent paths, but using `.permutations(indices.len()).unique()` does not always neatly divide these into two 'halves' of the list of permutations]

DONE: Benchmark pristine non-optimized brute-force solution, for comparison with attempted optimized versions.

DONE: Filter out the equivalent paths to leave only unique paths. [EDIT: Turns out this is a _terrible_ idea. Does not scale well. So slow it's not even worth trying to benchmark. Takes orders of magitude longer to find solution, at n = 10, when this filtering is included. Need to figure out mathematically how to only generate single set of unique paths, which means reliquishing the handy `permutations...unique()` helper.]

TODO: Do not create (or worse, delete) permutations where first number (representing an index) is larger than last number (also representing an index.) This looks like it may well work, especially if I can find a way to prevent the creation of any such permutation.
See: https://stackoverflow.com/questions/960557/how-to-generate-permutations-of-a-list-without-reverse-duplicates-in-python-us

TODO: Cache distances calculated between 2 points, as same 2 points will be considered more than once in any reasonably large collection of points (and many times for large collections)

This gives us (n - 1)! / 2 permutations for n points.

Steps 2 & 3 involve iterating over each generated list, calculating and summing the distances between each pair of points taken in sequence, and returning the set that gave the shortest total distance (and that distance)

## Benchmarks:

Using [Criterion crate](https://bheisler.github.io/criterion.rs/book/getting_started.html).
Also installed [cargo-criterion plugin](https://github.com/bheisler/cargo-criterion).

To run: `$ cargo criterion`