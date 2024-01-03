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
> # Draft 2:
> - [x] Incorporate feedback from my wife.
> - [x] Consider changing "search space" to "search area"
> - [x] Discuss datastructures for storing ranges.
> - [x] Image sizing
> - [x] Review titles and title numbering
> - [x] Make sure pseudocode doesn't wrap at full width
> - [x] Styling increase column width
> - [x] Label figures, and refer to labels for clarity
> - [x] Find a home for these tidbits:
>   - [x] if we were to use Euclidean distance each sensor would be a [circle](https://en.wikipedia.org/wiki/Gauss_circle_problem)[manhattan-distance]
>   - [x] Another side effect of using Manhattan distance is that the distance between any two integer points is always an integer[closed-addition].
> - [x] Add visualisation of final solution
> - [x] Rename diagram files 
> # Pre Publish
> - [ ] Make sure website RSS feed works
> - [ ] gianhancock.com domain
> - [ ] Complete all inline TODOs
> - [ ] Add proper performance numbers for all approaches, add a table at the end including manhattan distance counts
> - [ ] Clean up source code and diagrams in repo
> - [ ] Fix implementation coordinate spaces with swapped `x'` `y'` axes and off by one.
> - [ ] Double check title numbering
> - [ ] https://giscus.app/
> - [ ] Make sure website appears nicely on Feedly 
> # Later
> - [ ] Implement proper asides rather than using blockquotes
> - [ ] Implement readable implementations of each approach
> - [ ] Consider switching from Hugo to Zola
>   - [ ] Table of contents

# 1. Advent of Code
[Advent of Code](https://adventofcode.com/2022/about) (abbreviated AoC) is an annual Advent calendar of small programming puzzles. There's a programming puzzle for each day in the calandar, accompanied by a story which ties the problems together.

After hearing about AoC from some friends, I decided to give it a go. About halfway through I came across a problem which I thought was particularly interesting: [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15).

# 2. The Problem
AoC problems come in two parts, with the second part building on the first. I want to focus on the aspects that I found most interesting, so I'm going to skip straight to the second part and gloss over some minor details. To get the full experience complete with [Excuse Plot](https://tvtropes.org/pmwiki/pmwiki.php/Main/ExcusePlot) and all, you'll need to look at the actual AoC problem description[^aoc-problem].

The general idea is that there are a bunch of "sensors" which are used to detect "beacons". Sensors have a position and range, and are arranged such that all but one point is covered by at least one sensor. Our task is to find the single point which isn't covered by any sensors.

I'll start with a diagram, and I'll attempt to explain how to interpret it from there:

{{<figure src="trivial-example.svg" width="400px" caption="fig. 1: A trivial example">}}

The problem takes place on a 2D grid, I've placed gridlines at 1 unit intervals in the diagram to help visualise the scale and position of things. In general, all the action takes place at "integer points", which are points with integer coordinates e.g. `(0, 0)`, `(1, 2)`, `(3, 4)`, etc[^lattice]. Each integer point occurs at the intersection of two gridlines in the diagram.

The single uncovered point we're looking for is somewhere in a 3x3 area which is represented by a dotted square. I'll call this area the "search area", in fig. 1 the search area is 3x3 units, but in the actual problem it's *much* larger: 4×10<sup>6</sup> by 4×10<sup>6</sup> units. The origin `(0, 0)` is always located at the bottom left corner of the search area. We'll show some area around the search area to give context, but remember that we're looking for uncovered points *inside* the search area.

In fig. 1 there is a single sensor `A` positioned at `(2, 2)` with a range of `2`. The area covered by `A` is diamond shaped rather than circular. This is because distance is measured by [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry). Note that some of the sensors coverage is outside of the search area, this is okay.

Finally, there's a green circle at the point `(0, 0)`. This represents the solution to our toy version of the problem. In this case it's easy enough to see visually that this is the only uncovered point. Technically there's a small triangular area which isn't covered containing an infinite number of points, but remember we're only really interested in what happens at integer points[^point-coverage]. Also note that points on the border of a sensor are considered covered.

To drive this all home, let's look at a slightly more complicated example:

# 2.1. A Running Example
I'll use this running example throught this post:

{{<figure src="running-example.svg" width="550px" caption="fig. 2: A running example">}}

- We have 4 sensors: 
  - `A: position = (0, 4); range = 3`
  - `B: position = (4, 3); range = 2`
  - `C: position = (1, 0); range = 2`
  - `D: position = (4, 1); range = 2`
- Sensors are allowed to overlap, as `B` and `D` do.
- The search area is 4x4 units. Areas at the edge of the search area are in bounds, so we have a total of 5<sup>2</sup> integer points in our search area, not 4<sup>2</sup> [as you might expect](https://en.wikipedia.org/wiki/Off-by-one_error#Fencepost_error).
- This time the solution is at `(2, 2)`, we can see visually that this is the only integer point in the search area which is uncovered.

Now, the input data I get from AoC has 40 sensors (yours may vary), and we're expected to find a solution in a 4×10<sup>6</sup> by 4×10<sup>6</sup> search area. Solving real problems by hand isn't going to cut it. In this post I'm going to discuss a few different approaches to solving this problem and compare their run times. I'll present these approaches in the order that I implemented them.

# 3. Solving via Brute Force
The brute force approach is simple, here's some [Rust](https://www.rust-lang.org/)y [pseudocode](https://en.wikipedia.org/wiki/Pseudocode):

> I'll use pseudocode throughout this post to summarise the algorithms I discuss. The pseudocode is not really all that faithful to my actual implementations, it's just a useful tool to concisely explain the high level process.

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

Easy... except it takes so long to run that I've never been able to get solutions to full sized problems with it. Our search area contains 1.6×10<sup>13</sup> integer points. On my machine I can check 5.3×10<sup>7</sup> points/s, extrapolating gives an estimated run time of 3.5 days in the worst case. Better than solving by hand, but still not good enough.

# 4. Solving With Column Skipping
With the brute force approach, the primary operation is checking whether a point is covered by any sensors—I'll call this operation a "coverage check". We can speed things up by performing fewer coverage checks. Our brute force code scans across each row, column by column. Instead, once we hit a sensor for the first time, we can take one large step across all columns that are within range of that sensor.

Let's walk through an example, but first some more diagramming conventions:
- I'll label sensors with a letter when needed for clarity.
- A black dot indicates a point where a coverage check occurs.

{{<figure src="column-skipping-example.svg" width="550px" caption="fig. 3: Solving using column skipping">}}

- We start our search at `(0, 0)`. 
- The coverage check at `(0, 0)` fails because `(0, 0)` is in `A`.
- We skip along the row to the first point outside of `A` at `(3, 0)`.
- The coverage check at `(3, 0)` fails because `(3, 0)` is in `B`.
- Repeating what we did last time, we skip along the row to the first point outside `B`: `(6, 0)`.
- `(6, 0)` is out of bounds, we skip the coverage check and move up to the next row at `(0, 1)`.
- We repeat this process, eventually landing on `(0, 4)`.
- `(0, 4)` passes the coverage check. This is our solution.

## 4.1. Running Example: Column Skipping

{{<figure src="running-example-column-skippping.svg" width="550px" caption="fig. 4: A total of 6 coverage checks are performed (black dots). We stop once we find the solution.">}}

## 4.2. Pseudocode: Column Skipping
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

## 4.3. Summary and Performance: Column Skipping

I found this approach surprisingly effective—at least it was surprising to me. The runtime has gone from approximately 3.5 days to 227ms on my machine, a factor of 1.3×10<sup>6</sup> improvement. 

Thinking about this critically for a moment: with the brute force method we perform 4×10<sup>6</sup> coverage checks for each row, now we only need around 2 (based on our worked example). Roughly speaking we'd expect a factor of 2×10<sup>6</sup> speedup, so in hindsight I shouldn't have been surprised afterall.

I think 227ms is "good enough", but can we do better?

# 5. A Different Approach: "Range Exclusion"
Now, remember that the input data I got from AoC only has 40 sensors. That's miniscule compared to the 4×10<sup>6</sup> rows in our search area.

Perhaps we can find an algorithm which operates primarily on sensors. Even a bad algorithm operating on 40 sensors is likely to be faster than a good algorithm operating on 4×10<sup>6</sup> points.

After some thought (and cheating[^inspiration]) I managed to find a way to solve the problem by iteratively ruling out sections of the search area. It's a bit tough to explain, and there are some fiddly details to work out, so let's start with a simplified version of the problem.

## 5.1 A Simplified Problem
We can modify the problem slightly to make it much simpler. Let's forget about Manhattan distances for now; instead, each sensor will cover a rectangular area of the 2D grid. As we'll see soon, this simplifies the problem because it makes everything axis aligned.

Here's an example of our simplified version of the problem:

{{<figure src="simplified-example-1.svg" width="600px" caption="fig. 5: Example 1 of our simplified problem. Solution at `(2, 2)`.">}}

Going back to our goal—to solve the problem by operating primarily on sensors—we can make a couple of observations:

- Focusing on just the rows for now. We saw that the solution is `(2, 2)`, therefore the row `y=2` must *not* be fully covered by sensors.
- Conversely, we know there is a single solution, so every other row must be *fully* covered. In this case rows `y in {0, 1, 3, 4}` are all fully covered.
- Rows may be covered by a single sensor, or a combination of sensors. In this case the rows `y in {0, 1}` are covered by a single sensor `D`, the remaining fully covered rows `y in {3, 4}` are covered by the combination of sensors `{A, B, C}`.
- We can work the other way around too: for any combination of sensors, we can determine which rows they cover. For example:
  -  `{D}` covers rows `y in {0, 1}`
  -  `{A, D}` covers rows `y in {0, 1}`. `A` is redundant here, as `D` alone has this effect, nonetheless this fact still holds.
  -  `{A, B, C}` covers `y in {3, 4}`
  -  `{A}` covers no rows
  -  `{A, B}` covers no rows
  -  ... and many more combinations that I'll skip for brevity cover no rows.
-  The rows covered by a set of sensors will always be contiguous.

Based on these observations, we can find *all* rows which are covered by considering *all* combinations of sensors. Doing so will leave us with a single remaining row which isn't covered. The `y` point of this row is the `y` coordinate of the solution.

Let's step through this using the example above:

- let `possible_y_values` = `{0, 1, 2, 3, 4}`. This is all y values in the search area.
- all possible combinations of sensors are: `{{A}, {B}, {C}, {D}, {A, B}, {A, C}, {A, D}, and so on...}` (I've omitted some for brevity)
- Iterate over all sets of sensors, removing covered rows from `possible_y_values`.
   - `{A}` covers no rows, no need to update `possible_y_values`.
   -  `{B}` covers no rows, no need to update `possible_y_values`. Okay, from now on I'm going to skip sets of sensors which don't cover any rows.
   -  {A, B, C} covers `{3, 4}`, update `possible_y_values` from `{0, 1, 2, 3, 4}` to `{0, 1, 2}`
   -  `{A, D}` covers rows `{0, 1}`. Update `possible_y_values` from `{0, 1, 2}` to `{2}`
   -  {D} covers rows `{0, 1}`. `possible_y_values` remains `{2}`
- After iterating through all sensor sets, there is only one remaining possible y value: `2`. This is our result for the y coordinate.

We can get the x coordinate by repeating this process for columns instead of rows[^range-exclusion-optimisation], in this case we would also get the value of `2` for x. This leaves us with the final solution of `(2, 2)`.

Okay, so we've found another way to arrive at our solution, but our goal was to do so while avoiding any sort of iteration over points or rows, did we achieve this? Well, it depends... we're certainly not performing coverage checks on a per-point or per-row basis any more, we're not performing coverage checks at all. However we *are* storing rows in `possible_y_values`. To make it worse, `possible_y_values` contains *every* row at the beginning of the process. Storing all 4×10<sup>6</sup> rows seems contradictory to our goal, but all is not lost thanks to an observation we made earlier:

> The rows covered by a set of sensors will always be contiguous.

We can take advantage of this by storing ranges of rows. We can store a beginning and end coordinate to represent a range of values. For example the rows `{0, 1, 2, 3, 4, 5}` can be stored as an inclusive range from 0 through 5 which I will denote as `[0, 5]`. Similarly the rows `{0, 1, 2, 3, 4, 7}` can be stored as a set of ranges `{[0, 4], [7, 7]}`[^range-data-structure].

{{<figure src="simplified-example-2.svg" width="500px" caption="fig. 6: Example 2 of our simplified problem. Solution at `(2, 4)`">}}

- let `possible_y_values` = `{[0, 5]}`.
- Iterate over all sets of sensors (excluding some for brevity).
   - `{A, B}` covers the range `[2, 3]`. Update `possible_y_values` from `[0, 5]` to `{[0, 1], [4, 5]}`
      - {D, E, B} covers the range `[5, 5]`. Update `possible_y_values` from `{[0, 1] [4, 5]}` to `{[0, 1], [4, 4]}`
   - `{A, C}` covers the range `[0, 2]`. Update `possible_y_values` from `{[0, 1], [4, 4]}` to `{[4, 4]}`
- There is only one possible solution: `4`. This is our result for the y coordinate.

Storing `possible_y_values` as a set of ranges is a bit more complicated, and in these small examples it doesn't seem to be worth it. Remember though, we're covering an area 4×10<sup>6</sup> units wide with only 40 sensors. The actual ranges we'll be dealing with are huge. By storing `possible_y_values` this way, the total number of values we need to store is a function of the number of sensors, *not* the size of the search area.

Finally, before we move on to the full problem, here is some pseudocode for getting the y coordinate of the solution:

```rust
// Initialise the set of ranges `possible_y_values` as a single range covering 
// the full search area.
let possible_y_values = {[0, 4000000]};
// Iterate each possible sensor set. Implementation of get_sensor_sets() 
// omitted.
for sensor_set in get_sensor_sets() {
  // Get the range of rows completely covered by sensor set. This may be an 
  // empty range. Implementation of get_covered_y_range() is omitted.
  let covered_y_range = get_covered_y_range(sensor_set)
  // Subtract the covered ranges from possible_y_values. Implementation of 
  // subtract_range_set() is omitted.
  let possible_y_values = subtract_ranges(possible_y_values, covered_y_range)
}
// possible_y_values should only contain a single range at this point. The range
// should only contain one value. This is the solution for y.
return possible_y_values.first().start
```

> The finer details of how to implement `get_sensor_sets()`, `get_covered_y_range()`, and `subtract_range()` are omitted for brevity. This is admittedly a cop out because they aren't completely trivial to implement, however I don't want to distract from the main point, which is the higher level aspects of the algorithm. 

> TODO: Include link to source code.

## 5.2 Tackling the Full Problem
Now we've got that simpler case out of the way, how is it affected when we move onto the full problem? Well, we can use the same approach, however the sensor coverage is no longer axis aligned:

{{<figure src="diamond-range-exclusion.svg" width="600px" caption="fig. 7: The ranges of rows `[0, 1]` and `[3, 4]` (shaded green) are covered by sensors, leaving 2 as the solution's y coordinate.">}}

Figuring out which rows are covered visually is a bit more difficult with the unaligned sensor coverage; it's a lot more difficult to do algorithmically. Instead of trying to tackle this problem, we can shift our perspective to a more convenient coordinate system.


### 5.2.1. Introducing "Diagonal Space"

Let's start with a single sensor with `range=2` `position=(4, 2)`:

{{<figure src="diagonal-space-example-standard.svg" width="400px" caption="fig. 8: A lone sensor at (4, 2)">}}

We can represent this exact situation in a coordinate system that's been scaled, translated, and rotated clockwise by 45°, as shown by the axes in fig. 9. For lack of a better name, I'll call this "diagonal space", I'll denote the diagonal space axes as `x'` and `y'`.

{{<figure src="diagonal-space-example-with-axes.svg" width="550px" caption="fig. 9: The sensor from fig. 8 with diagonal space axes `x'` and `y'`">}}
  
In this coordinate system, the sensor is located at `(8, 6)`. Also, if you tilt your head, you'll see that the sensor coverage is now axis aligned! Here, I'll do it for you:

{{<figure src="diagonal-space-example-screen-aligned.svg" width="750px" caption="fig. 10: fig. 9 but with `x'` and `y'` aligned with the screen. Note that the search area is no longer axis-aligned.">}}

In this coordinate system we can mostly reuse the solution from the "simple" version; once we find the solution in diagonal space, we'll need to convert the solution back into the original coordinate system[^coordinate-system]. Unfortunately it's not all roses, there are still some complications we need to deal with. When we changed coordinate system, we gained axis aligned sensors at the cost of an unaligned search area. This is unfortunate, but I think it's worthwhile as it allows us to easily operate on sets of sensors. Before tackling the complications, let's have a look at our go to example.

### 5.2.2. Running Example in Diagonal Space
{{<figure src="running-example-diagonal-space.svg" width="750px" caption="fig. 11: Our go to example in diagonal space. Tilt your head 45° counterclockwise (or your screen clockwise) and it should look familiar.">}}

We can see that every row in the solution space is completely covered except for the row `y'=4`. I've annotated the set of sensors which cover each row on the left of the diagram. We see that the approach we took for the simplified problem applies here too.

### 5.2.3. Complications in Diagonal Space
In the "simple" version, we relied on the observation that "rows covered by a set of sensors are contiguous" to optimise our storage of `possible_y_values`. We'll keep storing `possible_y_values` as a set of ranges, however we no longer get a *single* contiguous set of rows covered by each sensor set. Let's look at another example to illustrate why:

{{<figure src="diagonal-space-complication-example.svg" width="650px" caption="fig. 12: Another example with row coverage annotated on the left. Solution at `(7, 3)` (diagonal space)">}}

In this case we see that the sensor A actually covers two contiguous regions, `[0, 2]` and `[6, 8]`:

{{<figure src="diagonal-space-complication-example-highlighted.svg" width="650px" caption="fig. 13: Rows covered by `A` highlighted">}}

We need to account for the possibility of 2 contiguous regions of covered rows. This is a bit more complicated, but it doesn't change the overall approach.

## 5.2. Pseudocode: Range Exclusion
Finally, here's our pseudocode for getting the y coordinate of the solution, it's very similar to the previous version:

```rust
// Convert sensor positions into diagonal space
for sensor in sensors {
  sensor.position = rectangular_to_diagonal_space(sensor.position)
}

// Initialise the set of ranges `possible_y_values` as a single range covering
// the full search area.
let possible_y_values = {
  [
    rectangular_to_diagonal_space(0, 0).y, 
    rectangular_to_diagonal_space(0, 4000000).y
  ]
};
// Iterate each possible sensor set. Implementation of get_sensor_sets() 
// omitted.
for sensor_set in get_sensor_sets() {
  // Get a set of ranges covered by the sensor set. There may be 0, 1 or 2 
  // returned ranges. Implementation of get_covered_y_ranges() is omitted.
  let covered_y_ranges = get_covered_y_ranges(sensor_set);
  // Subtract the covered ranges from possible_y_values. Implementation of 
  // subtract_ranges() is omitted.
  let possible_y_values = subtract_ranges(possible_y_values, covered_y_ranges);
}

// Omitted: repeat the same for x...

// possible_y_values and possible_x_values should only contain a single range at 
// this point. The ranges should only contain one value. These are the solution 
// coordinates in diagonal space.
let diagonal_space_solution = (
  possible_y_values.first().start, 
  possible_x_values.first().start);

// Convert the solution back to rectangular space.
return diagonal_to_rectangular_space(diagonal_space_solution);
```
> TODO: Link to implementation

We do a bit of converting back and forth from standard to diagonal space and we handle multiple ranges for each for each sensor set. Otherwise it's the same as the previous version.

## 5.3. Summary and Performance: Range Exclusion

> TODO: I haven't done this yet.

# 6. Yet Another Approach: "Line Intersection"

We've found two very different approaches to solving this problem, but we're not done yet! There's another approach we can tackle which is based on taking advantage of the properties of the sensors near the solution[^inspiration-2].

## 6.1. Exploring Patterns Around the Solution

We know our solution is unique, so all integer points surrounding the solution must be covered by a sensor (or outside of the search area if the solution is on a border). Let's explore this by looking at some example cases, we'll focus on only the sensors required to cover the 8 coordinates around the solution.

{{<figure src="focus-points.svg" width="300px" caption="fig. 14: The 8 points around the solution which we will focus on (marked with x)">}}

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="intersection-case-1.svg" width="500px" caption="fig. 15: Case 1 \"Box\"">}}
  {{<figure src="intersection-case-2.svg" width="500px" caption="fig. 16: Case 2 \"Hall\"">}}
  {{<figure src="intersection-case-3.svg" width="500px" caption="fig. 17: Case 3 \"Flower\"">}}
  {{<figure src="intersection-case-4.svg" width="500px" caption="fig. 18: Case 4 \"Splat\"?">}}
</div>

This isn't an exhaustive list of cases, but it's enough to gain some intuition for the problem. We see that in all cases the solution is always exactly 1 unit away from the border of a sensor. This makes sense, as we've already established that the points adjacent to the solution must be covered.

If we expand each of our sensors' range by one, then our solution will always be *on* the border of a sensor. Let's redraw these examples with enlarged sensors:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="intersection-case-1-expanded.svg" caption="fig. 19: \"Box\" enlarged" width="500px">}}
  {{<figure src="intersection-case-2-expanded.svg" caption="fig. 20: \"Hall\" enlarged" width="500px">}}
  {{<figure src="intersection-case-3-expanded.svg" caption="fig. 21: \"Flower\" enlarged" width="500px">}}
  {{<figure src="intersection-case-4-expanded.svg" caption="fig. 22: \"Splat\" enlarged" width="500px">}}
</div>

We see that the solution is indeed always on the border of the sensor. For this analysis, we really only care about the borders of each sensor nearest the solution. Let's focus on those:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="local-01-expanded-borders.svg" caption="fig. 23: \"Box\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-02-expanded-borders.svg" caption="fig. 24: \"Hall\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-03-expanded-borders.svg" caption="fig. 25: \"Flower\" enlarged, nearest borders" width="500px">}}
  {{<figure src="local-04-expanded-borders.svg" caption="fig. 26: \"Splat\" enlarged, nearest borders" width="500px">}}
</div>

When the borders of sensors intersect at a T or X junction, the intersection is either exactly on an integer point, or it's exactly halfway between them. We'll call these intersections "aligned" or "misaligned" respectively.

In the "Box", "Flower", and "Splat" cases, the solution lies on an aligned intersection. In the "Hallway" case, the intersection is adjacent to a misaligned intersecton. The solution is always either on an aligned intersection or adjacent to a misaligned one.

Based on this, we can find solutions by following these steps:
- Enlarge the sensors' ranges by 1.
- Find all intersections between the borders of the enlarged sensors.
- At each intersection aligned intersection, perform a coverage check.
- For each misaligned intersection, perform a coverage check on each of the 4 adjacent integer points.

 The number of intersections is a function of the positon and number of sensors—but not the size of the search area—so this will scale well as the search area grows.

## 6.2. Running Example: Line Intersection

Let's work through our running sample problem:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="running-example.svg" width="500px" caption="fig. 3 (repeated): Our running example.">}}
  {{<figure src="running-example-line-intersection.svg" caption="Case 1" width="500px" caption="fig. 27: Expanded borders and black dots indicating coverage checks.">}}
</div>

We see that the solution at `(2, 2)` is on an aligned intersection and will be picked up by a coverage check.

## 6.3. Corner and Edge Cases

Now we have some literal edge cases to take care of, let's look at some examples where the solution lies on the edge of the search area:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="edge-case-1.svg" width="300px" caption="fig. 28: Edge case 1">}}
  {{<figure src="edge-case-2.svg" width="300px" caption="fig. 29: Edge case 2">}}
</div>

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="edge-case-1-expanded.svg" width="300px" caption="fig. 30: Edge case 1">}}
  {{<figure src="edge-case-2-expanded.svg" width="300px" caption="fig. 31: Edge case 2 expanded">}}
</div>

Fortunately, our assumptions still hold for edge cases. How about corner cases?

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="corner-case.svg" width="350px" caption="fig. 32: Corner case 1">}}
  {{<figure src="corner-case-expanded.svg" width="350px" caption="fig. 33: Corner case 1 expanded">}}
</div>

Nope! Fortunately there are only 4 corners, so we can simply hardcode a coverage check for each corner and we're done!

## 6.4. Pseudocode: Line Intersection

Here's the final pseudocode:

```rust
// Initialise points_to_check with the 4 corners of the search area.
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

## 6.5. Summary and Performance: Line Intersection

> TODO: I haven't done this section yet.

> In this case `get_intersection_points` and `is_aligned_intersection` is left as an excercise to the reader.

# 7. Closing Thoughts
At the time of writing I've only completed AoC 2022 days 1 through 15, but this is definitely my favourite problem so far. I had a ton of fun exploring the different approaches and measuring their performance. I think the size of the search area combined with the use of Manhattan distance[^manhattan-distance][^chebyshev_distance] makes the problem shine. The most basic approach—brute force—is only just too slow[^brute-force], but any optimisation at all allows us to solve the problem in reasonable time, opening the door to many different approaches.

## 7.1. Final Visualisations

And finally, here's a visualisation of the actual input I got from AoC[^actual-solution-type]:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
{{<figure src="final-solution-1.svg" width="500px" caption="fig. 34: 6,000,000 x 6,000,000 view">}}
{{<figure src="final-solution-2.svg" width="500px" caption="fig. 35: 1,600,000 x 1,600,000 view">}}
{{<figure src="final-solution-3.svg" width="500px" caption="fig. 36: 800,000 x 800,000 view">}}
{{<figure src="final-solution-4.svg" width="500px" caption="fig. 37: 125 x 125 view">}}
</div>

{{<figure src="final-solution-5.svg" width="800px" caption="Only sensors neighbouring the solution">}}

# 7.2. Performance Comparison

> TODO: I haven't done this section yet.

# Footnotes

[^aoc-problem]: 
    You'll also need to solve part 1 to get the part 2 description. Although I'm sure you can find it online somewhere.

[^manhattan-distance]: 
    If [Euclidian distance](https://en.wikipedia.org/wiki/Euclidean_distance) was used instead, we'd need to deal [circular sensor coverage](https://en.wikipedia.org/wiki/Gauss_circle_problem).

    Another effect of using Manhattan distance is that distances between integer points are always an integers, keeping the solutions free from floating point complications. This is thanks to the fact that the lattice is [closed under addition](https://en.wikipedia.org/wiki/Closure_(mathematics)) and Manhattan distances are calculated using addition.

[^chebyshev_distance]:
    Just for fun, here's another interesting distance metric: [Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance).

[^point-coverage]:
    Irrelevant, but interesting nonetheless: A sensor with range R covers (R+1)<sup>2</sup> + R<sup>2</sup> integer points. e.g. a sensor with `range=0` covers 1 coordinate, a sensor with `range=3` covers 25. This formula is a little more complicated than you might expect because the square area is not axis aligned. For some intuition behind this formula, have a look at this sensor with `range=2`. It can be broken down into a 3x3 grid (red) and a 2x2 grid (blue). {{<figure src="point-count.svg" width="300px">}} Tangentially related: [Pick's Theorem](https://artofproblemsolving.com/wiki/index.php/Pick%27s_Theorem) relates the area of a polygon with the count of its internal and border [lattice points](#fn:2).

[^lattice]: 
    Because everything takes place on integer points, you could say the problem takes place in a [point lattice](https://mathworld.wolfram.com/PointLattice.html). The integer points we'll be talking about throughout are also known as [lattice points](https://mathworld.wolfram.com/LatticePoint.html).

[^inspiration]: 
    Credit where credits due: I couldn't think of anything until I skimmed the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). I was trying to skim for inspiration without spoiling for myself. Luckily I saw a mention of rotating the coordinate space (you'll see why this is relevant later) which got me unstuck.

[^inspiration-2]:
    Once again, credit to the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). Skimming this thread I saw a couple of references to line intersections which got me started on this approach.

[^closed-addition]: 
    Another affect of using Manhattan distance is that distances between integer points is always an integer. This is thanks to the fact that Manhattan distances are calculated using only additon, and lattices are [closed under addition](https://en.wikipedia.org/wiki/Closure_(mathematics)).

[^inspiration]: 
    Credit where credits due: I couldn't think of anything until I skimmed the [Reddit solution thread](https://old.reddit.com/r/adventofcode/comments/zmcn64). I was trying to skim for inspiration without spoiling for myself. Luckily I saw a mention of rotating the coordinate space (you'll see why this is relevant later) which got me unstuck.

[^empty-sensor-sets]: 
    In my implementation, I actually discard sensor sets where the intersection of the sensor's of y values is empty. These sensor sets can't cover any rows that aren't covered by some subset of the sensors. Doing this dramatically cuts down the number of sensor sets to check. I gloss over this detail for the sake of focusing on the high level aspects, but it's actually critical because there are 2<sup>40</sup> - 1 possible combinations of sensors, the vast majority of which are redundant. If the set of all sensors is `S`, than the sensor sets I discuss in this post are the elements of the  [power set](https://en.wikipedia.org/wiki/Power_set) of `S` (excluding the empty set).

[^coordinate-system]: 
    In fig. 9. I've set up the origin of the "diagonal space" coordinate system so that `y'` and `x'` touch the top left and bottom left corners of the solution space respectively. There's nothing special about this convention, I chose it to make the visualisations neater. Nonetheless, here's some Rust code for converting between the systems using this convention:
    
    ```rust
    // `dimension` is the width of the search area.

    fn rectangular_to_diagonal(vec: &Vec2, dimension: i32) -> Vec2 {
        Vec2 {
            x: vec.x - vec.y + dimension,
            y: vec.x + vec.y,
        }
    }

    fn diagonal_to_rectangular(vec: &Vec2, dimension: i32) -> Vec2 {
        Vec2 {
            x: (-dimension + vec.x + vec.y) / 2,
            y: (dimension - vec.x + vec.y) / 2,
        }
    }
    ```

    A simpler convention would leave the origin at the bottom left corner of the search area in both coordinate systems. This would make the conversion functions simpler and remove the dependency on `dimension` for conversions.

[^range-exclusion-optimisation]:
    After finding the solution for y, we can shortcut the x coordinate. Now that we know the exact y value for the solution, it's sufficient to consider sensors one at a time instead of sets of sensors.

[^range-data-structure]:
    In my implementation I store a set of ranges as a [dynamic array](https://en.wikipedia.org/wiki/Dynamic_array) ([Rust Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)) of integer [2-tuples](https://en.wikipedia.org/wiki/Ordered_pair). I'm sure I could use a more efficient data structure. Perhaps an [interval tree](https://en.wikipedia.org/wiki/Interval_tree), I'm not sure?

[^actual-solution-type]:
    In my case it's a "Box" type solution (see fig. 15). I'm not sure if this is the case for everyone.

[^brute-force]:
    Actually, some people did [brute force](https://www.reddit.com/r/adventofcode/comments/zmcn64/comment/j0rjkbk/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) it on a GPU, which is pretty cool.