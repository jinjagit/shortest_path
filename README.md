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

This is all possible permuations of the subset at indices 1..n-1, with the element at index 0 remining in place in all resulting permutations.

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

Using the super-fast Rust `Itertools` `permutations(n).unique()` command gives us 2 subsets of effectively equivalent unique routes, since it returns reverse copies of each route.

One would imagine, therefore, that removing, or filtering out these duplicates would improve performance. Benchmarking, however, shows that this is not the case. Using either the elegant `vec.retain(|v| v[0] < v[n - 1])` (which compares the first and last elements of each 'route' vec), or a simple conditional to 'ignore' such cases when calculating the routes total distance, adds about 10% to the run-time (confimed for n = 5, 10 & 12 points).

Even though this seems counter-intuitive, and theoretically there may be a scale where the perfomance hit is less than that due to increasing route distance calculations, we have no evidence this method is useful. Thus, we should just calculate all route distances for the (n-1)! cases, and not for the (n-1)! / 2 subset.

____________________________________________________________________________

### Distances matrix

Since we know that we will calculate the distance between any 2 points multiple times, we could first calculate the distance between each unique pair of points and put the results in a matrix. We could then use this matrix as a look-up-table to calculate the total distance of each route permutation.

We make this 'extra efficient' by only calculating, say, the distance between point 2 -> point 4, and using this as the matrix value for point 4 -> point 2 also. This reduces the calculations needed by half.

This approach significantly improves run-times. 4% faster for 10 points and 12% faster for 12 points. It seems likely this improvement becomes more significant as we increase the number of points.

____________________________________________________________________________

### Single thread

We could use multiple threads, but since previous experimentation shows this gives about a 4 - 5 times perfomance increase, and this will probably only make solving for paths with one added point reasonable, on my machine, it seems unimportant to implement. What matters more is having a fair comparison between brute force and any approximation method tried.

____________________________________________________________________________

### TLDR

Using a matrix of distances between points and _not_ filtering out the single subset of duplicate route permutations gives us a reasonable brute-force method for finding the shortest path through any given set of coordinates.

In practice, solving for any more than 12 points is unsuitable for benchmarking on my machine (each solution takes about 50 seconds). 10 points is way faster (approx. 0.25 secs / solution).

____________________________________________________________________________

## Benchmarks:

Using [Criterion crate](https://bheisler.github.io/criterion.rs/book/getting_started.html).
Also installed [cargo-criterion plugin](https://github.com/bheisler/cargo-criterion).

To run: `$ cargo criterion`

____________________________________________________________________________

## Resources?
- https://github.com/LazoCoder/Ant-Colony-Optimization-for-the-Traveling-Salesman-Problem/blob/master/Ants/Ant.java
- https://www.baeldung.com/java-ant-colony-optimization