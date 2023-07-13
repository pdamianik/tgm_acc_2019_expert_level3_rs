# TGM Advent Coding Contest 2019 Expert Level 3

A blazingly fast reimplementation of [pdamianik/TGM-ACC-2019 level3](https://github.com/pdamianik/TGM-ACC-2019/blob/master/levels/level3/main.py) (in Rust of course :wink:), bringing calculation times down to ~500ms for `M(2*10^9)`.

## The problem

The problem is defined as:

```
S[0]=290797 S[n+1]=S[n]^2 mod 50515093 Let A(i,j) min S[i],S[i+1],…,S[j] for i≤j. Let M(N) = ∑A(i,j) for 1≤i≤j≤N
```

or in python:

```python
def S(n: Int) -> Int:
    result = 290797
    for i in range(n):
        result = result**2 % 50515093
    return result

def A(i: Int, j: Int) -> Int:
    minS = S(i)
    for i in range(i+1, j+1):
        val = S(i)
        if val < minS:
            minS = val
    return minS

def M(n: Int) -> Int:
    sumA = 0
    for j in range(1, n+1):
        for i in range(1, n+1):
            sumA += A(i, j)
    return sumA
```

or in rust: See [src/initial.rs](src/initial.rs)

In particular we are interested in the value `M(2_000_000_000) = ?` and know that `M(10) = 432256955` and `M(10_000) = 3264567774119`.

`M(10)` can be calculated quickly with just the naive implementation above (e.g. the rust implementation takes between 50ms and 100ms to complete), but since this algorithm has a time complexity of `O(n^4)` it requires a different approach for anything beyond `n = 10^1`.

## Optimizations

### Caching

time: `O(n^3)`, space: `O(n)`

The first simple optimization can be seen in the [original repository](https://github.com/pdamianik/TGM-ACC-2019/blob/master/levels/level3/main.py#L5) where a simple cache for the `S(n)` function is added. This optimization reduces the time complexity to `O(n^3)` which still is far from usable for our target of `n = 2_000_000_000`.

> Because a cache for `S(n)` is used in later optimizations (e.g. [src/rayon.rs:7](src/rayon.rs#L7)) there is no deticated caching version in the rust implementation

### Reuse previous minima and parallelize

time: `O(n^2)` (with caching), `O(n^3)` (without caching), space: `O(n)` (with caching), `O(1)` (without caching)

> Note: the rust implementation uses caching (see: [src/rayon.rs:7](src/rayon.rs#L7) and [src/parallel.rs:11](src/parallel.rs#L11))

A different way to describe the problem is to sum the minima of all continuous ranges of `S[]`. A way to visualize the naive algorithm for `M(4)` would be:

```
S ->  |1|2|3|4|
i = 1  _        = 1
j = 1 --------- = 1
i = 1  _ _      = min(1, 2)
i = 2    _      = 2
j = 2 --------- = min(1, 2) + 2
i = 1  _ _ _    = min(1, 2, 3)
i = 2    _ _    = min(2, 3)
i = 3      _    = 3
j = 3 --------- = min(1, 2, 3) + min(2, 3) + 3
i = 1  _ _ _ _  = min(1, 2, 3, 4)
i = 2    _ _ _  = min(2, 3, 4)
i = 3      _ _  = min(3, 4)
i = 4        _  = 4
j = 4 --------- = min(1, 2, 3, 4) + min(2, 3, 4) + min(3, 4) + 4
```

where each `i`-row is the minimum of all the underlined elements and each `j`-row is the sum of all `i`-rows up to the previous `j`-row (exclusive).
The `j`-rows therefore mirror the behavior of `A(i, j)`.

The result would be the sum of all `j`-rows: `min(1) + min(1, 2) + min(2) + min(1, 2, 3) + min(2, 3) + min(3) + min(1, 2, 3, 4) + min(2, 3, 4) + min(3, 4) + min(4)`.

By reversing the order of the `i`-rows between the `j`-row we can reuse the minimum of the the previous iteration of `i` by utilizing the fact that `min(a, b, c) = min(min(a, b), c)`:

```
S ->  |1|2|3|4|
i = 1  _        = 1
j = 1 --------- = 1
i = 2    _      = 2
i = 1  _ _      = min(1, 2)
j = 2 --------- = 2 + min(1, 2)
i = 3      _    = 3
i = 2    _ _    = min(2, 3)
i = 1  _ _ _    = min(1, min(2, 3))
j = 3 --------- = min(3) + min(2, 3) + min(1, min(2, 3))
i = 4        _  = 4
i = 3      _ _  = min(3, 4)
i = 2    _ _ _  = min(2, min(3, 4))
i = 1  _ _ _ _  = min(1, min(2, min(3, 4)))
j = 4 --------- = min(4) + min(3, 4) + min(2, min(3, 4)) + min(1, min(2, min(3, min(4)))
```

Each `i`-row can now reuse the result of the previous `i`-row, which results in a time-complexity of `O(n^2)` (assuming that a lookup into `S[]` has a time complexity of `O(1)`, which can be accomplished with a cache)

Additionally every `j`-row can be calculated independently of the other `j`-rows and therefore we can use multiple threads to handle each `j`-row.

These optimizations are implemented in [src/rayon.rs](src/rayon.rs) (implemented with [rayon](https://crates.io/crates/rayon)) and [src/parallel.rs](src/parallel.rs) (implemented with rust native threads)

### Make it linear

time: `O(~n)` space: `O(~1)`

This algorithm iterates through `S[]` mostly linearly resulting in the `O(~n)` time complexity.
Because this algorithm accesses `S[]` linearly there is no need for a cache for all `S[]` values anymore. There is a local cache for some of the last `S[]` values until the last maximum which makes up the `O(~1)` space complexity as this cache doesn't grow with `n` but the specific properties of `S[]` specifically that it has a global minimum and that it is periodic.
Another difference to the previous algorithm is that every `j`-row now depends on some of the last `j`-rows (at least the previous one but in the worst case on all the ones until the last global minimum; this is the local cache).

In this algorithm we always explicitly add the current element to the sum (in [src/linear.rs:32](src/linear.rs#L32)) as every element will be the minimum at least once, when it is the only element being checked.

The first optimization is the optimization reusisng previous minima from the previous algorithm applied linearly (The minimum between this element and the last element).
If the last element is smaller than the current element we can simply add the sum from the last element to the total sum. This is done in [src/linear.rs:49-51](src/linear.rs#L49-L51).

```
...|i-3|i-2|i-1|i|...
     _             = i-3
 _   _             = min(..., i-3)
   --------------- = i-3 + min(..., i-3)
         _         = i-2
     _   _         = min(i-3, i-2)
 _   _   _         = min(..., i-3, i-2)
   --------------- = i-2 + min(i-3, i-2) + min(..., i-3, i-2)
             _     = i-1
         _   _     = min(i-2, i-1)
     _   _   _     = min(i-3, i-2, i-1)
 _   _   _   _     = min(..., i-3, i-2, i-1)
   --------------- = i-1 + min(i-2, i-1) + min(i-3, i-2, i-1) + min(..., i-3, i-2, i-1) = last_sum
// current iteration
                _  = i // added anyway
             _  _  = i-1 // Note that from here on we can just use the previous rows since they will always contian an element smaller than the current one (starting with the last element)
         _   _  _  = min(i-2, i-1)
     _   _   _  _  = min(i-3, i-2, i-1)
 _   _   _   _  _  = min(..., i-3, i-2, i-1)
   --------------- = i /* added anyway */ + last_sum = new last_sum
```

The second optimization is that if the current element is smaller than the last element, we have to backtrack trough a local cache of the last elements to find a smaller element.
While iterating through the local cache we keep track of a cache minimum from the last element onward and subtract it with each iteration from the last sum and add the current element.
This effectively replaces each of the last added minima in the last sum which are bigger than our current element in `S[]` with the current element.
The cache has to reach until the last global minimum since that element is guaranteed to be smaller than our current element (if it wasn't our current element would be the global minimum, which is a seperate case handled by the last optimization).
Lastly the updated last sum is added to the total sum and the current element is added to the last sum. This is done in [src/linear.rs:35-51](src/linear.rs#L35-L51).

```
...|g|i-3|i-2|i-1|i|...
    _                = g // global minimum
 _  _                = ... * g // always g
   ----------------- = g + ... * g
       _             = i-3 // smaller than i, in local cache
    _  _             = g
 _  _  _             = ... * g
   ----------------- = i-3 + g + ... * g
           _         = i-2 // bigger than i, in local cache
       _   _         = i-3
    _  _   _         = g
 _  _  _   _         = ... * g
   ----------------- = i-2 + i-3 + g + ... * g
               _     = i-1 // bigger than i, in local cache
           _   _     = min(i-2, i-1)
       _   _   _     = i-3
    _  _   _   _     = g
 _  _  _   _   _     = ... * g
   ----------------- = i-1 + min(i-2, i-1) + i-3 + g + ... * g = last_sum
// current iteration:
                  _  = i // added anyway
               _  _  = i
           _   _  _  = i
       _   _   _  _  = i-3
    _  _   _   _  _  = g
 _  _  _   _   _  _  = ... * g
   ----------------- = i /* added anyway */ + last_sum - (i-1) + i - min(i-2, i-1) + i /* backtracking */ = new last_sum
```

The finial optimization keeps track of the global minimum.
If our current element is smaller or equals the last global minimum, `S[i] * (i - 1)` can simply be added to the total sum and the local cache can be reset.
It is important that this case should also apply when the current element equals the last global mimimum as this keeps the local cache small.
This is done in [src/linear.rs:53-59](src/linear.rs#L53-L59).

```
// current iteration
...|i-2|i-1|g|...
            _  = g // added anyway
         _  _  = g
     _   _  _  = g
 _   _   _  _  = ... * g
   ----------- = g /* added anyway */ + g * (i - 1) /* i is the index of S which starts at 1 which is why we have to subtract one here, in praxis S gets indexed starting at 0 so this is just i in the code */ = new last_sum
```

### Abuse the cycle

time: `O(~1)`, space: `O(~1)`

The last two algorithms use the fact that the elements of `S[]` repeat and have a repeating global mimimum of 3.
The difference between [src/cycle.rs](src/cycle.rs) and [src/hardcoded.rs](src/hardcoded.rs) is that the former calculates the span of a cycle on the fly and uses that to extrapolate any `M(n)`, whereas the latter has the cycle attributes hardcoded.
Both also use [src/linear.rs](src/linear.rs) in the background to calculate values inside and before the first iteration of the cycle.

The following is a simplyfied view of the cycles of S:

```
         6               5
|-1-|---2v--|---3---|---4v--|
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|

g...Global minima
1...Prefix (all values before the first global minimum)
2...The first cycle
3...All cycles in between the target cycle and the first cycle (padding cycles)
4...The target cycle
5...The example target (n)
6...The example target position in the first cycle
a,b,d,e,f...arbitrary numbers in S[]
```

Any target before or in the first cycle will be returned early, since we have to calculate those areas anyway.

The first step in [src/cycle.rs](src/cycle.rs) is to find the first and second occurance of a global minimum in `S[]`.
The results of this step are hardcoded in [src/hardcoded.rs](src/hardcoded.rs), which is the only difference between these two algorithms.
This is done in [src/cycle.rs:11-37](src/cycle.rs#L11-L37) and hardcoded in [src/hardcoded.rs:6-9](src/hardcoded.rs#L6-L9).

When the first cycle is found we can calculate the target's equivalent position in the first cycle with `first_min_index + ((n - first_min_index) % (second_min_index - first_min_index) /* the cycle width */)`.
Additionally we can get the target cycle index (from the first cycle onwards, in the example it would be 1) with the forumla `(n - first_min_index) / (second_min_index - first_min_index) /* the cycle width */`.
This is done in [src/cycle.rs:58-61](src/cycle.rs#L58-L61) and [src/hardcoded.rs:19-21](src/hardcoded.rs#L19-L21).

```
         4               3
|-p-|---0v--|---1---|---2v--|
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|

g...Global minima
0...cycle with index 0 (the first cycle)
1...cycle with index 1
2...Cycle with index 2 (the example target cycle)
3...The example target (n)
4...The example target position in the first cycle
p...The prefix
a,b,d,e,f...arbitrary numbers in S[]
```

The `cycle_local_sum` is simply `M(target's equivalent first cycle position)`.
This is taken from cached values in [src/cycle.rs:62](src/cycle.rs#L62) and calculated based on the hardcoded state of `M(min_index)` in [src/hardcoded.rs:23-46](src/hardcoded.rs#L23-L46).

With the value of `M(first_cycle_index)` (calculated with [src/linear.rs](src/linear.rs)) we can fill in the cycles in between with the `first_cycle_sum = M(second_min_index) - M(first_min_index)` multitplied with the `cycle_index`.
`first_cycle_sum` is the sum of all the `j`-rows in the first cycle and multitplied with the `cycle_index`. This results in the sum of all the cycles in between the target position in the first cycle and the target position in the target cycle minus all the global minima in between (those grow with each cycle).
This is done in [src/cycle.rs:64-73](src/cycle.rs#L64-L73) and [src/hardcoded.rs:48-53](src/hardcoded.rs#L48-L53).

```
|----6----|
    |-------5-------|    
|-6-|-------5-------|--6--|
|0|0|3|3|3|3|3|3|3|3|3|3|3| // The global minima included in 5 and 6
|0|0|0|0|0|0|4|4|4|4|8|8|8| // The global minima missing in 5 and 6
         4               3
|-p-|---0v--|---1---|---2v--|
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|

g...Global minima
0...cycle with index 0 (the first cycle) (the sum of this cycle is the first_cycle_sum -> gets scaled with the cycle index 2)
1...cycle with index 1
2...Cycle with index 2 (the example target cycle)
3...The example target (n)
4...The example target position in the first cycle
5...The scaled first_cycle_sum
6...The cycle_local_sum
p...The prefix
a,b,d,e,f...arbitrary numbers in S[]
```

The last step is to fill in these missing global minima. Since the total sum of cycle global minima is an arithmetic progressing with each additional cycle we can use [this forumla](https://en.wikipedia.org/wiki/Arithmetic_progression#Sum) to calculate the fill with the cycle index as `n`.
This is done in [src/cycle.rs:77-88](src/cycle.rs#L77-L88) and [src/hardcoded.rs:56-64](src/hardcoded.rs#L56-L64).
This also adds global minima for a full target cycle which have to be subtracted in the end to result in the right result. See [src/cycle.rs:90-94](src/cycle.rs#L90-L94) and [src/hardcoded.rs:66-70](src/hardcoded.rs#L66-L70).

```
|----6----|
    |-------5-------|    
|-6-|-------5-------|--6--|
|0|0|3|3|3|3|3|3|3|3|3|3|3| // The global minima included in 5 and 6
|0|0|0|0|0|0|4|4|4|4|8|8|8| // The global minima missing in 5 and 6
|0|0|0|0|0|0|4|4|4|4|8|8|8|8| // The global minima filled in
|0|0|0|0|0|0|0|0|0|0|0|0|0|8| // The global minima subtracted to get to the right result
         4               3
|-p-|---0v--|---1---|---2v--|
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|

g...Global minima
0...cycle with index 0 (the first cycle) (the sum of this cycle is the first_cycle_sum -> gets scaled with the cycle index 2)
1...cycle with index 1
2...Cycle with index 2 (the example target cycle)
3...The example target (n)
4...The example target position in the first cycle
5...The scaled first_cycle_sum
6...The cycle_local_sum
p...The prefix
a,b,d,e,f...arbitrary numbers in S[]
```

All in all the sums of the cycles can be visualized as follows:

```
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
 _                           -> a
----------------------------- = a
   _                         -> b
 _ _                         -> min(a, b)
----------------------------- = b + min(a, b) // the sum until here is the prefix_sum
     _                       -> g
   _ _                       -> g
 _ _ _                       -> g
----------------------------- = 3 * g /* prefix global minima */ // the first_cycle_sum starts here
       _                     -> d
     _ _                     -> g
   _ _ _                     -> g
 _ _ _ _                     -> g
----------------------------- = d + 3 * g /* prefix global minima */
         _                   -> e
       _ _                   -> min(d, e)
     _ _ _                   -> g
   _ _ _ _                   -> g
 _ _ _ _ _                   -> g
----------------------------- = e + min(d, e) + 3 * g /* prefix global minima */ // The sum from the beginning until here is the cycle_local_sum
           _                 -> f
         _ _                 -> min(e, f)
       _ _ _                 -> min(d, e, f)
     _ _ _ _                 -> g
   _ _ _ _ _                 -> g
 _ _ _ _ _ _                 -> g
----------------------------- = f + min(e, f) + min(d, e, f) + 3 * g /* prefix global minima (are included in the first_cycle_sum) */ // the first_cycle_sum ends here
             _               -> g
           _ _               -> g
         _ _ _               -> g
       _ _ _ _               -> g
     _ _ _ _ _               -> g
   _ _ _ _ _ _               -> g
 _ _ _ _ _ _ _               -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = 4 * g /* cycle global minima (have to be filled in) */ + 3 * g /* prefix global minima */
               _             -> d
             _ _             -> g
           _ _ _             -> g
         _ _ _ _             -> g
       _ _ _ _ _             -> g
     _ _ _ _ _ _             -> g
   _ _ _ _ _ _ _             -> g
 _ _ _ _ _ _ _ _             -> g
----------------------------- = d + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                 _           -> e
               _ _           -> min(d, e)
             _ _ _           -> g
           _ _ _ _           -> g
         _ _ _ _ _           -> g
       _ _ _ _ _ _           -> g
     _ _ _ _ _ _ _           -> g
   _ _ _ _ _ _ _ _           -> g
 _ _ _ _ _ _ _ _ _           -> g
----------------------------- = e + min(d, e) + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                   _         -> f
                 _ _         -> min(e, f)
               _ _ _         -> min(d, e, f)
             _ _ _ _         -> g
           _ _ _ _ _         -> g
         _ _ _ _ _ _         -> g
       _ _ _ _ _ _ _         -> g
     _ _ _ _ _ _ _ _         -> g
   _ _ _ _ _ _ _ _ _         -> g
 _ _ _ _ _ _ _ _ _ _         -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = f + min(e, f) + min(d, e, f) + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                     _       -> g
                   _ _       -> g
                 _ _ _       -> g
               _ _ _ _       -> g
             _ _ _ _ _       -> g
           _ _ _ _ _ _       -> g
         _ _ _ _ _ _ _       -> g
       _ _ _ _ _ _ _ _       -> g
     _ _ _ _ _ _ _ _ _       -> g
   _ _ _ _ _ _ _ _ _ _       -> g
 _ _ _ _ _ _ _ _ _ _ _       -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = 4 * g /* cycle global minima */ + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                       _     -> d
                     _ _     -> g
                   _ _ _     -> g
                 _ _ _ _     -> g
               _ _ _ _ _     -> g
             _ _ _ _ _ _     -> g
           _ _ _ _ _ _ _     -> g
         _ _ _ _ _ _ _ _     -> g
       _ _ _ _ _ _ _ _ _     -> g
     _ _ _ _ _ _ _ _ _ _     -> g
   _ _ _ _ _ _ _ _ _ _ _     -> g
 _ _ _ _ _ _ _ _ _ _ _ _     -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = d + 4 * g /* cycle global minima */ + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                         _   -> e
                       _ _   -> min(d, e)
                     _ _ _   -> g
                   _ _ _ _   -> g
                 _ _ _ _ _   -> g
               _ _ _ _ _ _   -> g
             _ _ _ _ _ _ _   -> g
           _ _ _ _ _ _ _ _   -> g
         _ _ _ _ _ _ _ _ _   -> g
       _ _ _ _ _ _ _ _ _ _   -> g
     _ _ _ _ _ _ _ _ _ _ _   -> g
   _ _ _ _ _ _ _ _ _ _ _ _   -> g
 _ _ _ _ _ _ _ _ _ _ _ _ _   -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = e + min(d, e) + 4 * g /* cycle global minima */ + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */
                           _ -> f
                         _ _ -> min(e, f)
                       _ _ _ -> min(d, e, f)
                     _ _ _ _ -> g
                   _ _ _ _ _ -> g
                 _ _ _ _ _ _ -> g
               _ _ _ _ _ _ _ -> g
             _ _ _ _ _ _ _ _ -> g
           _ _ _ _ _ _ _ _ _ -> g
         _ _ _ _ _ _ _ _ _ _ -> g
       _ _ _ _ _ _ _ _ _ _ _ -> g
     _ _ _ _ _ _ _ _ _ _ _ _ -> g
   _ _ _ _ _ _ _ _ _ _ _ _ _ -> g
 _ _ _ _ _ _ _ _ _ _ _ _ _ _ -> g
|a|b|g|d|e|f|g|d|e|f|g|d|e|f|
----------------------------- = f + min(e, f) + min(d, e, f) + 4 * g /* cycle global minima */ + 4 * g /* cycle global minima */ + 3 * g /* prefix global minima */ // the cycle global minima in this row would be an overcalculation if e is the target and therefore have to be removed retroactively
```

## Some rough timings

Done with `cargo bench` wich uses [criterion.rs](https://github.com/bheisler/criterion.rs) in the background. The following are the center timing results (probably medians), except for [src/linear.rs](src/linear.rs) `M(2*10^9)` which is just a single manual run.

|   call    |[src/initial.rs](src/initial.rs)|[src/rayon.rs](src/rayon.rs)|[src/parallel.rs](src/parallel.rs)|[src/linear.rs](src/linear.rs)|[src/cycle.rs](src/cycle.rs)|[src/hardcoded.rs](src/hardcoded.rs)|
|-----------|--------------------------------|----------------------------|----------------------------------|------------------------------|----------------------------|------------------------------------|
| time complexity | `O(n^4)` | `O(n^2)` | `O(n^2)` | `O(n)` | `O(~1)` | `O(~1)` |
| space complexity | `O(1)` | `O(n)` | `O(n)` | `O(~1)` | `O(~1)` | `O(~1)` |
|  M(10^1)  |            2.6323 µs           |          19.230 µs         |             273.02 µs            |           727.77 ns          |          653.09 ns         |             750.55 ns              |
|  M(10^4)  |               -                |          852.29 ms         |             50.711 ms            |           913.78 µs          |          1.1499 ms         |             868.87 µs              |
| M(2*10^9) |               -                |             -              |                -                 |     ~4.5 min (single run)    |          1.5520 s          |             497.97 ms              |

