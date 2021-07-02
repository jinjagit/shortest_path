# shortest_path

Investigating the 'travelling sales-person' problem.

Start from [plot repo](https://github.com/jinjagit/plot) and build to solve first by brute force.

## Brute force solution

1. Generate all unique paths for given set of coords (that visits each point once && creates a loop that ends at start coord)
2. Calculate length of each path generated
3. Return shortest path as vec of given coords + the total length of this shortest path

Step 1 would generate the following permutations, given a set of coords `[a, b, c, d]`:
```
[a, b, c, d]
[a, b, d, c]
[a, c, b, d]
[a, c, d, b]
[a, d, b, c]
[a, d, c, b]
```
And creating the full path would involve adding the start point to the end of each list.
For example [a, d, b, c] becomes [a, d, b, c, a]

Steps 2 & 3 involve iterating over each generated list, calculating and summing the distances between each pair of points taken in sequence, and returning the set that gave the shortest total distance (and that distance)