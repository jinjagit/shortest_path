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

### Filtering duplicates?

The super fast Rust `Itertools` `permutations(n).unique()` command gives us 2 sets of actually unique routes, since it returns reverse copies of each route.

One would imagine, therefore, that removing, or filtering out these duplicates would improve performance. Benchmarking, however, shows that this is not the case. Using either the elegant `vec.retain(|v| v[0] < v[n - 1])` (which compares the first and last elements of each 'route' vec), or a simple conditional to 'ignore' such cases when calculating the routes total distance, add about 10% to the run-time (confimed for n = 5, 10 & 12 points).

Even though this seems counter-intuitive, and theoretically there may be a scale where the perfomance hit is less than that due to increasing route distance calculations, we have no evidence this method is useful. Thus we should just calculate all route distances for the (n-1)! cases, and not for the (n-1)! / 2 subset.

### Distances matrix?

Since we know that we will calculate the distance between any 2 points multiple times, we could first calculate the distance between each unique pair of points and add the results to a matrix. We could then use this matrix as a look-up-table to calculate the total distance of each route permutation.

We should make this 'extra efficient' by only calculating, say, the distance between point 2 -> point 4, and using this as the matrix value for point 4 -> point 2 also. This reduces the calculattions needed by half.

-------------------------------------------------------------------------------------------------------------------------------------

TODO: Cache distances calculated between 2 points, as same 2 points will be considered more than once in any reasonably large collection of points (and many times for large collections)

This gives us (n - 1)! / 2 permutations for n points.

Steps 2 & 3 involve iterating over each generated list, calculating and summing the distances between each pair of points taken in sequence, and returning the set that gave the shortest total distance (and that distance)

## Benchmarks:

Using [Criterion crate](https://bheisler.github.io/criterion.rs/book/getting_started.html).
Also installed [cargo-criterion plugin](https://github.com/bheisler/cargo-criterion).

To run: `$ cargo criterion`