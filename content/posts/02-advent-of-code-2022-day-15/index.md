---
# title: "Spending an Unreasonable Amount on a Single AoC Problem"
# title: "1 Year Into a Single AoC Problem: Beacon Exclusion Zone"
title: "Advent of Code 2022 - Beacon Exclusion Zone: 1 Year Later"
date: 2023-12-31
author: Gian Hancock
---

> NOTE:
> 
> This post is in draft and will change over time.
> 
> TODO:
> - Rename diagram files 
> - Label figures, and refer to labels for clarity
> - Incorporate feedback from my wife.
> - Implement readable implementations of each approach
> - Clean up source code and diagrams in repo
> - Complete all inline TODOs
> - Add proper performance numbers for all approaches, add a table at the end including manhattan distance counts'
> - Implement proper asides rather than using blockquotes

# 1. Advent of Code

[Advent of Code](https://adventofcode.com/2022/about) (abbreviated AoC) is an annual Advent calendar of small programming puzzles. There's a programming puzzle for each day in the calandar, accompanied by a story which ties the problems together.

After hearing about AoC from some friends, I decided to give it a go. About halfway through I came across a problem which I thought was particularly interesting: [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15).

# 2. The Problem
AoC problems come in two parts, with the second part building on the first. I want to focus on the aspects that I found most interesting, so I'm going to skip straight to the second part and gloss over some minor details. To get the full experience complete with [Excuse Plot](https://tvtropes.org/pmwiki/pmwiki.php/Main/ExcusePlot) and all, you'll need to look at the actual AoC problem description[^aoc-problem].

The problem takes place on a 2D grid over a 4×10<sup>6</sup> by 4×10<sup>6</sup> unit area. I'll refer to this area as the "search space". On this grid are a number of sensors, each with an `(x, y)` position and a `range`; `x`, `y`, and `range` are integers. In general, we'll only be interested in what happens at integer points throughout this problem[^lattice]. Each sensor "covers" an area of the grid within it's range; however distances are measured with [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) rather than [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance). Before discussing the actual problem, let's visualise what we know so far, we'll stick to smaller areas for now. Here's a depiction of two sensors in a 4x4 area:

{{<figure src="fig-001-sensors.svg">}}

> A note on terminology:
> An integer point is a point with integer coordinates, e.g. `(0, 0)`, `(1, 2)`, `(3, 4)`, etc.

Let's cover the conventions I'll be using in these visualisations:
- A grid with lines at 1 unit intervals is present to help visualise the space.
- The dotted border represents the search space. In this case the search space is 4x4 units. Areas at the edge of the search space are in bounds, so we have a total of 5<sup>2</sup> integer points in the search space, *not* 4<sup>2</sup> [as you might expect](https://en.wikipedia.org/wiki/Off-by-one_error#Fencepost_error).
- The shaded squares represent sensors. The sensor's position is the exact center of the shaded area, which will always be an integer point. The shaded area represents the area within the sensors range, its coverage.
- The origin `(0, 0)` is always at the bottom left of the search space.

Now let's make some observations about this specific example:
- The smaller sensor is located at `(2, 1)` and has a range of 1. The integer points covered by this sensor are: `(0, 2), (1, 1), (1, 2), (1, 3), (2, 2)`. Note that coordinates on the borders are considered to be in range.
- The larger sensor is located at `(3, 2)` and has a range of 2. It covers 13 integer points[^point-coverage-count]—I won't list them here.
- Sensors may overlap.
- Sensors aren't necessarily located completely within the search space. In this case the larger sensor is partially outside of the search space.
- Each sensor covers a square area due to the use of Manhattan distance; if we were to use Euclidean distance each sensor would be a [circle](https://en.wikipedia.org/wiki/Gauss_circle_problem)[^gauss-circle]. Another side effect of using Manhattan distance is that the distance between any two integer points is always an integer[^closed-addition].

Now that we've got that out of the way, let's get back to the problem. We're given an arrangement of sensors such that all but 1 point in the search space is covered by at least one sensor. Our task is to find the one "uncovered" point.

Here's a trivial but complete example over a 3x3 search space: 

{{<figure src="fig-002-trivial-solution.svg">}}

The solution for examples will be depicted with a green circle. In this case we have a single sensor at `(2, 2)` with a range of 3. This sensor covers all points in the 3x3 search space except for `(0, 0)`, which is our solution.

Here's a slightly more complicated example, we'll use this as a running example throughout the post:

{{<figure src="fig-003-non-trivial-solution.svg">}}

In this case we can see that the solution is `(2, 2)`.

Solving these problems by hand isn't goint to cut it for full sized problems. In this post I'm going to discuss a few different approaches to solving this problem and compare their run times. I'll present these approaches in the order that I implemented them.

# 3. Solving via Brute Force
The brute force approach is simple, pseudocode:

```rust
for x in 0..=4000000 {
  for y in 0..=4000000 {
    for sensor in sensors {
      if manhattan_distance((x, y), sensor.position) <= sensor.range {
        break; // Continue to next point.
      }
    }
    return point; // If we reach this point, we've found the solution.
  }
}
```

Easy... except it takes so long to run that I've never been able to get solutions to full sized problems with it. Our search space contains 1.6×10<sup>13</sup> integer points. On my machine I can check 5.3×10<sup>7</sup> points/s, extrapolating gives an estimated run time of 3.5 days in the worst case. Better than solving by hand, but still not good enough.

# 4. Reducing the Search Space by Column Skipping
With the brute force approach, the primary operation is checking whether a point is covered by any sensors—I'll call this operation a "coverage check". We can speed things up by performing fewer coverage checks. Our brute force code scans across each row, column by column. Instead, once we hit a sensor for the first time, we can take one large step across all columns that are within range of that sensor.

Let's walk through an example, but first some more diagramming conventions:
- I'll label sensors with a letter when needed for clarity.
- A black dot indicates a point where a coverage check occurs.

{{<figure src="fig-004-column-skipping.svg">}}

- We start our search at `(0, 0)`. 
- The coverage check at `(0, 0)` fails because `(0, 0)` is in `A`.
- We skip along the row to the first point outside of `A` at `(3, 0)`.
- The coverage check at `(3, 0)` fails because `(3, 0)` is in `B`.
- Repeating what we did last time, we skip along the row to the first point outside `B`: `(6, 0)`.
- `(6, 0)` is out of bounds, we skip the coverage check and move up to the next row at `(0, 1)`.
- We repeat this process, eventually landing on `(0, 4)`.
- `(0, 4)` passes the coverage check. This is our solution.

And here's some pseudocode for good measure:

```rust
for x in 0..=4000000 {
  for y in 0..=4000000 {
    for sensor in sensors {
      if manhattan_distance((x, y), sensor.position) <= sensor.range {
        // Find the x coordinate of the right border of sensor for the current y
        let sensor_border_x = sensor.x + sensor.range - abs(y - sensor.y);
        // Advance to the next position outside of the sensor
        x = sensor_border_x + 1;
      }
    // If we reach this point, we've found the solution.
    return point;
  }
}
```

> The exact workings of `get_next_x()` is left as an excerise for the reader. The important thing is that it's a constant time operation based on the sensors position, range, and the current row.
> TODO: Link to implementation

I found this approach surprisingly effective—at least it was surprising to me. The runtime has gone from approximately 3.5 days to 227ms on my machine, a factor of 1.3×10<sup>6</sup> speedup. 

Thinking about this critically for a moment: with the brute force method we perform 4×10<sup>6</sup> coverage checks for each row, now we only need around 2 (based on our worked example). Roughly speaking we'd expect a factor of 2×10<sup>6</sup> speedup, so in hindsight I shouldn't have been surprised afterall.

I think 227ms is "good enough", but can we do better?

# 5. A Different Approach: Range Exclusion
Let's take a step back and look at the actual problem data I got from AoC:

```text
Sensor at x=1112863, y=496787: closest beacon is at x=1020600, y=2000000
Sensor at x=2980210, y=1712427: closest beacon is at x=2946825, y=1712605
Sensor at x=2799204, y=1425283: closest beacon is at x=2946825, y=1712605
<37 more lines>
```

It turns out we've only got 40 sensors to deal with. This is tiny compared to the 4×10<sup>6</sup> rows in our search space.

> Ignore the beacon stuff, that's a detail from the AoC problem which I've simplified away for this post. In fact, you can ignore it all, the important thing is that we've only got 40 sensors to deal with.

Perhaps we can find an algorithm which operates primarily on sensors. Even a bad algorithm operating on 40 sensors is likely to be faster than a good algorithm operating on 4×10<sup>6</sup> points.

After some thought (and cheating[^inspiration]) I managed to find a way to solve the problem by iteratively ruling out sections of the search space. It's a bit tough to explain, and there are some fiddly details to work out, so let's start with a simplified version of the problem.

## 5.1 A Simplified Problem
We can modify the problem slightly to make it much simpler. Let's forget about Manhattan distances for now; instead, each sensor will cover a rectangular area of the 2D grid. As we'll see soon, this simplifies the problem because it makes everything axis aligned.

Here's an example of our simplified version of the problem:

{{<figure src="fig-005-aabb.svg" caption="An example of our simplified problem with the solution at `(2, 2)`.">}}

Going back to our goal—to solve the problem by operating primarily on sensors—we can make a couple of observations:

- Focusing on just the rows for now. We saw that the solution is `(2, 2)`, therefore the row `y=2` must *not* be fully covered by sensors.
- Conversely, we know there is a single solution, so every other row must be *fully* covered. In this case rows `y in {0, 1, 3, 4}` are all fully covered.
- Rows may be covered by a single sensor, or a combination of sensors. In this case the rows `y in {0, 1}` are covered by a single sensor `D` the remaining fully covered rows `y in {3, 4}` are covered by the combination of sensors `{A, B, C}`.
- We can work the other way around too: for any combination of sensors, we can determine which rows they cover. For example:
  -  `{D}` covers rows `y in {0, 1}`
  -  `{A, B, C}` covers `y in {3, 4}`
  -  `{A}` covers no rows
  -  `{A, B}` covers no rows
  -  ... and many more combinations which cover no rows.  
-  The rows covered by a set of sensors will always be contiguous.

Based on these observations, we can find *all* rows which are covered by considering *all* combinations of sensors. Doing so will leave us with a single remaining row which isn't covered. The `y` point of this row is the `y` coordinate of the solution.

Let's step through this using the example above:

- let `possible_y_values` = `{0, 1, 2, 3, 4}`.
- all possible combinations of sensors are: `{{A}, {B}, {C}, {D}, {A, B}, {A, C}, {A, D}, and so on...}`
- Iterate over all combinations of sensors, removing covered rows from `possible_y_values`.
   - `{A}` covers no rows, no need to update `possible_y_values`.
   -  `{B}` covers no rows, no need to update `possible_y_values`. Okay, from now on I'm going to skip sets of sensors which don't cover any rows.
   -  {A, B, C} covers `{3, 4}`, update `possible_y_values` from `{0, 1, 2, 3, 4}` to `{0, 1, 2}`
   -  `{A, D}` covers rows `{0, 1}`. Update `possible_y_values` from `{0, 1, 2}` to `{2}`
   -  {D} covers rows `{0, 1}`. `possible_y_values` remains `{2}`
- After iterating through all sensor sets, there is only one remaining possible y value: 2. This is our result for the y coordinate.

We can get the x coordinate by repeating this process for columns instead of rows, in this case we would also get the value of `2` for x. This leaves us with the final solution of `(2, 2)`.

Okay, so we've found another way to arrive at our solution, but our goal was to do so while avoiding any sort of iteration over points or rows, did we achieve this? Well, it depends... we're certainly not performing coverage checks on a per-point or per-row basis any more, we're not performing coverage checks at all. However we *are* storing rows in `possible_y_values`. To make it worse, `possible_y_values` contains *every* row at the beginning of the process. Storing all 4×10<sup>6</sup> rows seems contradictory to our goal, but all is not lost thanks to an observation we made earlier:

> The rows covered by a set of sensors will always be contiguous.

We can take advantage of this by storing ranges of rows. We can store a beginning and end coordinate to represent a range of values. For example the rows {0, 1, 2, 3, 4, 5} can be stored as an inclusive range from 0 through 5 which I will denote as `[0, 5]`. Similarly the rows {0, 1, 2, 3, 4, 7} can be stored as a set of ranges `{[0, 4], [7, 7]}`.

With this in mind, let's step though another more complicated example. I'll explicitly represent `possible_y_values` as a set of ranges. I'll also skip over sensor sets which don't span a whole row:

<TODO: discuss storing ranges in a search tree, https://stackoverflow.com/questions/18948351/data-structure-to-store-integer-range-query-the-ranges-and-modify-the-ranges>

{{<figure src="fig-006-aabb-complex.svg">}}

- let `possible_y_values` = `{[0, 5]}`.
- Iterate over all combinations of sensors.
   - `{A, B}` covers the range `[2, 3]`. Update `possible_y_values` from `[0, 5]` to `{[0, 1], [4, 5]}`
      - {D, E, B} covers the range `[5, 5]`. Update `possible_y_values` from `{[0, 1] [4, 5]}` to `{[0, 1], [4, 4]}`
   - `{A, C}` coverse the range `[0, 2]`. Update `possible_y_values` from `{[0, 1], [4, 4]}` to `{[4, 4]}`
- There is only one possible solution: `4`. This is our result for the y coordinate.

Storing `possible_y_values` as a set of ranges is a bit more complicated, and in these small examples it doesn't seem to be worth it. Remember though, we're covering an area 4×10<sup>6</sup> units wide with only 40 sensors. The actual ranges we'll be dealing with are huge. By storing `possible_y_values` this way, the total number of values we need to store is a function of the number of sensors, *not* the size of the search space.

Finally, before we move on the the full problem, here is some pseudocode for getting the y coordinate of the solution:

```rust
// Initialise possible_y_values as a set of ranges containing a single range 
// covering the full search space.
let possible_y_values = {[0, 4000000]};
// Iterate each possible sensor set. Implementation of get_sensor_sets() 
// omitted.
for sensor_set in get_sensor_sets() {
  // Get a set of ranges covered by the sensor set. There may be 0, 1 or 2 
  // ranges in the returned set of ranges. Implementation of get_covered_rows() 
  // is omitted.
  let covered_ranges = get_covered_ranges(sensor_set)
  // Subtract the covered ranges from possible_y_values. Implementation of 
  // subtract_range_set() is omitted.
  let possible_y_values = subtract_ranges(possible_y_values, covered_ranges)
}
// possible_y_values should only contain a single range at this point. The range
// should only contain a single y coordinate. This is the solution for y.
return possible_y_values.first().start
```

> The finer details of how to implement `get_sensor_sets()`, `get_covered_rows()`, and `subtract_range()` are left as an exercise for the reader. This is admittedly a cop out, becuase they aren't trivial to implement, however I don't want to distract from the main point, which is the higher level aspects of the algorithm. 
> TODO: Include link to source code.
> TODO: Comma check
> TODO: Mention a huge number of spurious sets which cover no rows and culling.

### 3.2. Tackling the Full Problem
Now we've got that simpler case out of the way, how is it affected when we move onto the full problem? Well, we can use the same approach, however the sensor coverage is no longer axis aligned:

{{<figure src="fig-007-diamond-range-exclusion.svg" caption="The ranges of rows `[0, 1]` and `[3, 4]` (shaded green) are covered by sensors, leaving 2 as the solution's y coordinate.">}}

Figuring out which rows are covered visually is a bit more difficult with the unaligned sensor coverage; it's a lot more difficult to do algorithmically. Instead of trying to tackle this problem, we can shift our perspective to a more convenient coordinate system.


#### 3.3. Introducing "Diagonal Space"

Let's start with a single sensor with `range=2` `position=(4, 2)`:

{{<figure src="fig-008-diagonal-space.svg">}}

We can represent this exact situation in a coordinate system that's been scaled and rotated clockwise by 45°, as shown by the axes in the above image. For lack of a better name, I'll call this "diagonal space", I'll denote the diagonal space axes as `x'` and `y'`.
  
In this coordinate system, the sensor is located at `(4, 3)`. Also, if you tilt your head, you'll see that the sensor coverage is an axis aligned square! Here, I'll do it for you:

{{<figure src="fig-009-diagonal-space-screen-aligned.svg" caption="The same diagram as above, except x' and y' have been drawn in alignment with the screen. Note that the search space is no longer axis-aligned">}}

In this coordinate system we can mostly reuse the solution from the "simple" version. Unfortunately it's not all roses, there are still some complications we need to deal with. When we changed coordinate system, we gained axis aligned sensors at the cost of an unaligned search space. This is unfortunate, but i think it's worthwhile as it allows us to easily operate on sets of sensors. Before tackling the complications, let's have a look at our go to example:

{{<figure src="fig-010-diagonal-space-example-1.svg" caption="Our go to example in `(x', y')` space. Tilt your head 45° counterclockwise (or your screen clockwise) and it should look familiar.">}}

We can see that every row in the solution space is completely covered except for the row `y'=4`. I've annotated the set of sensors which cover each row on the left of the diagram. We see that the approach we took for the simplified problem applies here too.

In the pseudocode for the "simple" version I skimmed over the `get_covered_rows()` function. In this case, it's a bit more complicated, so I'll go into a bit more detail. We are taking advantage of the fact that "rows covered by a set of sensors are contiguous" to optimise our storage of `potential_y_values`. We'll keep doing this, however we no longer get a *single* contiguous set of rows covered by a sensor set. Let's look at another example to illustrate why:

{{<figure src="fig-011-diagonal-space-example-2.svg" caption="Rows coverage annotated on the left. Solution is on the row `y=3`">}}

In this case we see that the sensor A actually covers two contiguous regions, `[0, 2]` and `[6, 8]`:

{{<figure src="fig-012-diagonal-space-example-3.svg">}}

The implementation of `get_covered_rows()` will be more complicated to account for the unaligned shape of the search area and the possibility of two contiguous sets of covered rows. There will never be more than 2 regions of covered rows, so this is doable—although admittedly quite fiddly.

Finally, here's our pseudocode for getting the y coordinate of the solution, it's very similar to the previous version:

```rust
// Convert sensor positions into diagonal space
for sensor in sensors {
  sensor.position = to_diagonal_space(sensor.position)
}

// Initialise possible_y_values as a single range from coving the full search 
// space.
let possible_y_values = {
  [to_diagonal_space(0, 0).y, to_diagonal_space(0, 4000000).y]
};
// Iterate each possible sensor set. Implementation of get_sensor_sets() 
// omitted.
for sensor_set in get_sensor_sets() {
  // Get a set of ranges covered by the sensor set. There may be 0, 1 or 2 
  // ranges in the returned set of ranges. Implementation of 
  // get_covered_ranges() is omitted.
  let covered_ranges = get_covered_ranges(sensor_set);
  // Subtract the covered ranges from possible_y_values. Implementation of 
  // subtract_range_set() is omitted.
  let possible_y_values = subtract_ranges(possible_y_values, covered_ranges);
}
// possible_y_values should only contain a single range at this point. The 
// range should only contain a single y coordinate. This is the solution for y.
return possible_y_values.first().start;

// repeat the same for x...

let diagonal_space_solution = (x, y);
return from_diagonal_space(diagonal_space_solution);
```

We do a bit of converting back and forth from standard to diagonal space and we handle multiple ranges for each for each sensor set. Otherwise it's the same as the previous version.

## 4. Yet Another Approach: Line Intersection

We've found two very different approaches to solving this problem, but we're not done yet! There's another approach we can tackle which is based on taking advantage of the properties of the sensors near the solution[^inspiration-2].

We know our solution is unique, so all integer points surrounding the solution must be covered by a sensor (or outside of the search space if the solution is on a border). Let's explore this by looking at some example cases, we'll focus on only the sensors required to cover the 8 coordinates around the solution.

{{<figure src="fig-013b-constraints.svg" caption="The 8 points around the solution (marked with x) which we will focus on" width="500px">}}

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="intersection-case-1.svg" caption="Case 1: \"Box\"" width="500px">}}
  {{<figure src="intersection-case-2.svg" caption="Case 2: \"Hall\"" width="500px">}}
  {{<figure src="intersection-case-3.svg" caption="Case 3: \"Flower\"" width="500px">}}
  {{<figure src="intersection-case-4.svg" caption="Case 4: \"Person\"?" width="500px">}}
</div>

This isn't an exhaustive list of cases, but it's enough to gain some intuition for the problem. We see that in all cases the solution is always exactly 1 unit away from the border of a sensor. This makes sense, as we've already established that the points adjacent to the solution must be covered.

If we expand each of our sensors' range by one, then our solution will always be *on* the border of a sensor. Let's redraw these examples with enlarged sensors:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="intersection-case-1-expanded.svg" caption="\"Box\" enlarged" width="500px">}}
  {{<figure src="intersection-case-2-expanded.svg" caption="\"Hall\" enlarged" width="500px">}}
  {{<figure src="intersection-case-3-expanded.svg" caption="\"Flower\" enlarged" width="500px">}}
  {{<figure src="intersection-case-4-expanded.svg" caption="\"Person\" enlarged" width="500px">}}
</div>

We see that the solution is indeed always on the border of the sensor. For this analysis, we really only care about the borders of each sensor nearest the solution. Let's focus on those:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="local-01-expanded-borders.svg" caption="\"Box\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-02-expanded-borders.svg" caption="\"Hall\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-03-expanded-borders.svg" caption="\"Flower\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-04-expanded-borders.svg" caption="\"Person\" enlarged, nearest borders" width="500px">}}
</div>

When the borders of sensors intersect at a T or X junction, the intersection is either exactly on an integer point, or it's exactly halfway between them. We'll call these intersections "aligned" or "misaligned" respectively.

In the "Box", "Flower", and "Person" cases, the solution lies on an aligned intersection. In the "Hallway" case, the intersection is adjacent to a misaligned intersecton. The solution is always either on an aligned intersection or adjacent to a misaligned one.

Based on this, we can find solutions by following these steps:
- Enlarge the sensors' ranges by 1.
- Find all intersections between the borders of the enlarged sensors.
- At each intersection aligned intersection, perform a coverage check.
- For each misaligned intersection, perform a coverage check on each of the 4 adjacent integer points.

 The number of intersections is a function of the positon and number of sensors—but not the size of the search space—so this will scale well as the search space grows.

### 4.1. A Complete Example

Let's work through our running sample problem:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="fig-003-non-trivial-solution.svg" caption="Case 1" width="500px" caption="Our running example.">}}
  {{<figure src="running-example-line-intersection.svg" caption="Case 1" width="500px" caption="Expanded borders and black marks indicating coverage checks.">}}
</div>

We see that the solution at `(2, 2)` is on an aligned intersection and this solution will be picked up by a coverage check.

### 4.2. Edge Cases

TODO: Simplify edge case 2

Now we have some literal edge cases to take care of, let's look at some examples where the solution lies on the edge of the search space:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="edge-case-1.svg" caption="Edge case 1" width="500px">}}
  {{<figure src="edge-case-2.svg" caption="Edge case 2" width="500px">}}
</div>

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="edge-case-1-expanded.svg" caption="Edge case 1 solution" width="500px">}}
  {{<figure src="edge-case-2-expanded.svg" caption="Edge case 2 solution" width="500px">}}
</div>

Fortunately, our assumptions still hold for edge cases. How about corner cases?

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="corner-case.svg" caption="Case 1" width="500px">}}
  {{<figure src="corner-case-expanded.svg" caption="Case 1" width="500px">}}
</div>

Nope! Fortunately there are only 4 corners, so we can simply hardcode a coverage check for each corner and we're done!

Here's the final pseudocode:

```rust
// Initialise points_to_check with the 4 corners of the search space.
let points_to_check = [(0, 0), (0, 4000000), (4000000, 0), (4000000, 4000000)]

for intersection_point in get_intersection_points() {
  if is_aligned(intersection_point) {
    points_to_check.add(intersection_point);
  } else {
    x = intersection_point.x;
    y = intersection_point.y;
    points_to_check.add((x - 1, y));
    points_to_check.add((x + 1, y));
    points_to_check.add((x, y - 1));
    points_to_check.add((x, y + 1));
  }
}

for point in points_to_check {
  for sensor in sensors {
    if manhattan_distance(point, sensor.position) <= sensor.range {
      break; // Continue to next point.
    }
  }
  return point // If we reach this point, we've found the solution.
}
```

> In this case `get_intersection_points` and `is_aligned_intersection` is left as an excercise to the reader.

# Closing Thoughts
At the time of writing I've only completed AoC 2022 days 1 through 15, but this is definitely my favourite problem so far. I had a ton of fun exploring the different approaches and measuring their performance. I think the size of the search space combined with the nature of the problem itself is what does it for me. The most basic approach—brute force—is just too slow, but any optimisation at all allows us to solve the problem in reasonable time, opening the door to many different approaches.

[^aoc-problem]: You'll also need to solve part 1 to get the part 2 description. Although I'm sure you can find it online somewhere.

[^gauss-circle]: In my opinion, the choice to use manhattan distance is what really makes this problem shine. It makes the problem tractible, while at the same time enabling many interesting approaches, as you'll hopefully see.

[^point-coverage-count]: A sensor with range R covers (R+1)<sup>2</sup> + R<sup>2</sup> coordinates. e.g. a sensor with `range=0` covers 1 coordinate, a sensor with `range=3` covers 25. This formula is a little more complicated than you might expect because the square area is not axis aligned. For some intuition behind this formula, have a look at this sensor with `range=2`. It can be broken down into a 3x3 grid (red) and a 2x2 grid (blue). {{<figure src="point-count.svg">}} Tangentially related: [Pick's Theorem](https://artofproblemsolving.com/wiki/index.php/Pick%27s_Theorem) relates the area of a polygon with the count of its internal and border [lattice points](#fn:2). TODO: Check this link

[^lattice]: In writing this post, I've learnt that about [point lattices](https://mathworld.wolfram.com/PointLattice.html). We're essentially dealing with a point lattice here. The integer points we'll be talking about throughout are also known as [lattice points](https://mathworld.wolfram.com/LatticePoint.html).

[^inspiration]: Credit where credits due: I couldn't think of anything until I skimmed the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). I was trying to skim for inspiration without spoiling for myself. Luckily I saw a mention of rotating the coordinate space (you'll see why this is relevant later) which got me unstuck.

[^inspiration-2]: Once again, credit to the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). Skimming this thread I saw a couple of references to line intersections which got me started on this approach.

[^closed-addition]: This is thanks to the fact that Manhattan distances are calculated using only additon, and lattices are [closed under addition](https://en.wikipedia.org/wiki/Closure_(mathematics)).

[^inspiration]: Credit where credits due: I couldn't think of anything until I skimmed the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). I was trying to skim for inspiration without spoiling for myself. Luckily I saw a mention of rotating the coordinate space (you'll see why this is relevant later) which got me unstuck.