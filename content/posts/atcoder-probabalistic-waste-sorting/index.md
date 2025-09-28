---
title: "AtCoder Heuristic Contest 051: Probabilistic Waste Sorting"
date: 2025-09-20
author: Gian Hancock
toc: true
githubDiscussionUrl: 'https://github.com/gian-hancock/gian-hancock.github.io/discussions/5'
draft: false
---

## 1. Overview

This is an editorial for my solution to AtCoder Heuristic Contest 051 - Probabilistic Waste Sorting. Read the problem statement [here](https://atcoder.jp/contests/ahc051/tasks/ahc051_a).

This was a 10-day contest, but I only had a couple of afternoons to spend on it, so I decided to go with the simplest approach I could think of.

## 2. Solution
My solution works in two phases: first, I search for a suitable graph topology, then I search for an effective way to assign sorter and processor types.

### 2.1. Phase 1: Topology

I use a backtracking approach to enumerate possible graphs. There are too many possibilities to fully enumerate, so I use the one which connects the most processors after 0.9s of searching.

#### 2.1.1. Graph Search Implementation

While enumerating graphs, I ensure that the graph meets the constraints of the problem:
- Each sorter in use has exactly 2 exits
- No cycles
- No intersections between conveyor belts

In order to increase the chances of finding a good solution, I reduce the search space by adding some additional constraints:
- Sorter and processor node in-degree is limited to 1 (no merging belts)
- The number of sorters is capped at `max(10, num_processors + 1)`. I simply discard sorters beyond this limit. Note that `num_processors` varies across the test cases.

Here's some pseudocode:

```pseudocode
function enumerate_all_graphs():
    // Repeat the search for different starting points.
    for each initial_sorter in sorters:
        graph_state = empty_graph()
        frontier = empty_queue()
        path = empty_stack()
        visited_states = empty_set()

        add_edge(inlet, initial_sorter)
        frontier.add(initial_sorter)

        build_graph(graph_state, frontier, path, visited_states)

function build_graph(graph_state, frontier, path, visited_states):
    // Base case: no more sorters need edge assignments, no more opportunity to add edges.
    if frontier.is_empty():
        track_best_graph_so_far(graph_state)
        return

    // Process next sorter requiring edge assignments
    current_sorter = frontier.pop()
    path.push(current_sorter)

    // Optimisation: don't process if we already saw this state
    signature = graph_state.to_canonical_form()
    if signature in visited_states:
        path.pop()
        return
    visited_states.add(signature)

    // Find valid destinations for current sorter (no intersections, not currently connected)
    candidates = get_valid_destinations(current_sorter, graph_state, path)

    // Early termination if insufficient destinations
    if candidates.size() < 2:
        path.pop()
        return

    // Try all pairs of valid destinations
    for each pair (dest1, dest2) in combinations(candidates, 2):
        // Add edges to graph
        graph_state.add_edge(current_sorter, dest1)
        graph_state.add_edge(current_sorter, dest2)

        // Update frontier with new sorter nodes. Processors have no outgoing edges, so they don't expand the frontier.
        frontier_backup = frontier.copy()
        if dest1 is sorter and not in path and not in frontier:
            frontier.add(dest1)
        if dest2 is sorter and not in path and not in frontier:
            frontier.add(dest2)

        // Recurse
        build_graph(graph_state, frontier, path, visited_states)

        // Backtrack: restore state
        frontier = frontier_backup
        graph_state.remove_edge(current_sorter, dest2)
        graph_state.remove_edge(current_sorter, dest1)

    path.pop()
```

#### 2.1.2. Visualisation

I created a basic visualisation of the graph topology to help with debugging. You can see a small test example [here](small-example-all-states.html). Note that:
- The inlet is the red node on the left (text gets cut off)
- P# nodes are processors
- S# nodes are sorters
- Call #2, #3, #5, #6, #10, #11, #13, #16 are terminal states because every sorter has 2 outgoing edges, it's impossible to add more edges. Of these #5 and #16 are the "best" because they connect all 3 processors. #5 will be chosen as it's found first. After these states, the search backtracks to a previous state and continues searching.
- Call #7 is a dead end because there's no way to connect it to 2 more nodes. Recall that:
    - There can be no cycles, ruling out S3
    - There can be no intersections, ruling out P0
    - In-degree of each node is limited to 1, ruling out P1
  This leaves P2, but each sorter must connect to 2 nodes.
- Call #8, #12, and #14 is a dead end for similar reasons to Call #7. After these calls, the search backtracks and continues searching.
- Even this tiny example has a surprising number of states. There would be quite a few more if we allowed merging, which is allowed by the problem rules, but I chose to disallow it to reduce the search space.

Finally, I added a visualisation mode which only outputs a visualisation each time a new best graph is found. This is useful for real-sized inputs such as [this one](real-input-partial.html). You can see that my approach doesn't manage to connect all processors. There is much room for improvement here.

### 2.2. Phase 2: Assignment Optimisation

Phase 2 takes the best graph topology from Phase 1 and searches for an effective way to assign sorter and processor types. This phase also takes too long, so the best result after 0.9s is used.

Here's some pseudocode:

```pseudocode
function optimise_node_types(graph):
    // Low score is better (less incorrectly sorted waste)
    best_score = infinity

    while time_remaining():
        // Random sorter type assignments
        sorter_assignments = assign_random_sorter_types(graph)

        // Greedy processor optimisation
        processor_assignments = optimise_processors(graph, sorter_assignments)

        // Evaluate and track best
        score = calculate_score(graph, processor_assignments, sorter_assignments)
        if score < best_score:
            best_score = score
            save_best_configuration()

function optimise_processors(graph, sorter_assignments):
    // Greedy: assign each waste type to processor with highest success probability
    for each waste_type:
        best_processor = find_processor_with_max_probability(waste_type, available_processors)
        assign(waste_type, best_processor)
        remove(best_processor, available_processors)
```

## 3. Thoughts
In the past, I wouldn't compete if I felt like I didn't have enough time to dedicate to the contest. Recently I've started participating anyway; I just set my expectations according to how much time I have available. I still feel like I'm learning a lot and simplifying my approach to fit the time/skills I have available has been a good exercise.

Phase 1 was particularly troublesome for me. My brute force search was initially turning up terrible results, not connecting many processors at all. Introducing additional constraints to the search (in-degree 1) helped get acceptable results, but I still wasn't connecting all processors in the majority of cases. Looking at other editorials, it seems like using triangulation to generate a mesh of connections without intersections, then constraining the search to only use those edges seems like a great way to cut down the search space.

I have no idea how effective my Phase 2 approach is, as I didn't put any time into analysing it.

