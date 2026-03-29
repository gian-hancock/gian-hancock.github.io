---
title: "CodinGame Winter Challenge 2026: SnakeByte"
date: 2026-03-29
author: Gian Hancock
toc: true
draft: false
githubDiscussionUrl: 'https://github.com/gian-hancock/gian-hancock.github.io/discussions/6'
---

- [CodinGame Winter Challenge 2026: SnakeByte](https://www.codingame.com/multiplayer/bot-programming/winter-challenge-2026-snakebyte)
- `snakebyte-death-&-coop.mp4`: [replay link](https://www.codingame.com/replay/881567325)
- `food-hesitancy.mp4`: [replay link](https://www.codingame.com/share-replay/881527710)

This is an editorial for my entry to the CodinGame Winter 2026 competition. I finished 149th of 2382 with a bot written in Rust.

<!--IMAGE-->

## SnakeByte

SnakeByte is a multiplayer programming game based on Snake, with some variations to make it more interesting and suitable for a bot programming competition. Each match is played between two bot-controlled teams of four snakes on a 2D grid viewed from the side. Snakes grow by eating power sources, and the winner is the player with more total body segments remaining when the game ends.

<!--VIDEO-->

One new mechanic is gravity. A snake needs one of its body segments to be supported by something solid, otherwise the whole snake falls. Platforms, other snakes, and uneaten power sources all count as support. Gravity makes it difficult for shorter snakes to climb up high to reach power sources at the top of the map, however since snakes can be supported by other snakes, it's possible to cooperate with allied snakes, or even climb on opponents to get higher than otherwise possible.

Each competitor submits source code to the platform. On every turn, the bot reads the full game state from standard input and writes commands for each surviving snake to standard output. The game advances one turn at a time, with every snake moving one tile up, down, left, or right each turn. Both bots have 50 ms to submit their commands; if they fail to do so, their snakes continue moving in the same direction by default. After that, the platform simulates the outcome of both players' commands and advances to the next turn. The game continues until all food has been eaten or 200 turns have elapsed.

CodinGame runs matches between the submitted bots, using the results to place each bot on a leaderboard.

<!--RECENT MATCHES-->
<!--LEADERBOARD-->

You can read the full rules here: https://www.codingame.com/ide/puzzle/winter-challenge-2026-snakebyte

## Approach

My final entry uses [beam search](https://en.wikipedia.org/wiki/Beam_search) to choose moves for each snake. I went through several iterations before settling on the final implementation.

### Initial Implementation

My very first implementation divided the 50 ms turn time equally among all surviving snakes. Each snake performed a BFS that stopped as soon as it found reachable food, then moved along the discovered path. If no path to food was found before the time budget expired, the snake fell back to a simple heuristic that tried to avoid damage by avoiding moves likely to crash into a platform or another snake.

The BFS simulated each valid move from the current position, including damage and gravity. Each snake had only three valid moves per turn, since moving backwards would always cause it to crash into itself, so the search had a branching factor of 3. Allied and opponent snakes were both treated as static obstacles.

The competition is split into leagues: bronze, silver, gold, and platinum. This approach was enough to reach gold league. Its biggest weaknesses were:

1. The BFS only helped when it actually found food. If it did not, the snake fell back to a very simple heuristic and often made poor moves. This happened both when food was only reachable by relying on other snakes to move into useful positions later, and when the search simply could not go deep enough to find a path because the food was too far away.
2. Each snake operated individually and they could not cooperate to reach food.

### Beam Search

I wanted my bot to make reasonable decisions decisions even when it couldn't find food immediately so I replaced the per-snake BFS with a per-snake beam search. Planning was still sequential: each controlled snake searched against a projected working state so later allies could avoid cells already claimed by earlier ones, but there was still no true coordinated planning beyond the first move, and enemy snakes were still treated as static obstacles.

The search expanded states layer by layer for the allotted time, kept only the top scoring states at each depth, and then chose the root move corresponding to the best branch it had found. Each search node tracked the simulated body, the first root move, the search depth, and the cumulative length delta.

The evaluation function was:

`score = average_length_delta + distance_bonus`

`average_length_delta = cumulative_length_delta / depth`

Here `cumulative_length_delta` is the running total formed by adding the snake's current length delta at every simulated turn along the branch. That means an early gain or loss is counted again on each later turn, so earlier events have more weight than later ones. Dividing by depth then normalizes for branch length without removing that bias toward earlier, more reliable gains and losses. The `distance_bonus` was a bonus based on Manhattan distance to the nearest remaining food; the bonus decays linearly based on distance to the nearest food source.

I arrived at the cumulative length delta approach after noticing that snakes would sometimes ignore nearby food in favour of a longer, more efficient route to collect multiple pieces, only for an enemy snake to take the food first.

### Coordinated Beam Search

After analysing the per-snake beam search, I noticed that deeper searches were not especially beneficial. Presumably this was because the farther ahead the search looked, the less accurate the static-snake assumption became. That made me think it was better to spend simulation budget on coordination rather than on more depth, so I tried simulating all of my snakes together as a single entity.

I kept the beam search structure, but now each node represented a combined move for the whole team, for example `{snake1: up, snake2: down, snake3: up}`. In the worst case this increased the branching factor to `3^4 = 81`, three legal moves for four snakes.

Initially this performed much worse. The larger branching factor forced me to give up either beam width or search depth. I made some optimisations to increase the number of simulations I could perform per turn, tweaked the evaluation function, and introduced adaptive beam width. The adaptive beam width helped maintain a more consistent search depth as the effective branch factor changed during the game, for example when some snakes had already died.

Even with those improvements, I could never get this version of the bot to perform as well as the previous one.

### Adaptive Beam Search

The improvement that got me into platinum was backing away from full-team search and only searching nearby allies together. I partitioned my controlled snakes into groups of size 1 or 2 by greedily pairing allied snakes whose closest endpoints, either head or tail, were within Manhattan distance four. Any snake that was not paired became a singleton group. Each group was then planned together, while opponents and out-of-group allies were treated as static occupancy.

This change recovered a lot of the lost search depth while still allowing coordinated play. It was quite satisfying to watch pairs of snakes cooperate to climb toward food in situations where the earlier bots would fail. Since groups were capped at size 2, the worst-case branching factor dropped from `3^4 = 81` in the full-team search to `3^2 + 3^2 = 18` when the turn decomposed into two paired searches. If three snakes were close together, the search still decomposed them as 2 + 1 rather than trying to solve a larger combined problem.

<!--VIDEO of COORDINATION-->

Most of the remaining work was tuning the various bot parameters: the weights for the evaluation function, and the beam search width. I eventually settled on the same general adaptive beam-width concept as before, but tracked it separately for groups of size 1 and 2 so each could converge toward a more appropriate search depth.

The biggest weaknesses of the final bot were all consequences of treating enemy snakes as static obstacles. There was no attempt to simulate enemy moves, which meant the bot would often go into tunnels and die because it did not account for an opponent moving in to block the exit. It also made poor decisions about which food to contest, because there was no notion of who could realistically arrive first.

<!--VIDEO of contesting hopeless food-->
<!--Video of death in tunnel-->

I also think there was something slightly wrong with the evaluation function. Sometimes snakes would circle around food instead of just taking it. I tried to tune the scoring so that actually eating food would always be preferable, but there were still situations where the bot seemed too willing to hover near food for proximity score instead of converting it immediately.

<!--video of food hesitancy-->

## Workflow

The biggest workflow improvement compared to previous competitions was local testing using `cgarena`. This let me run various versions of my bots against each other. This was useful for parameter tuning.

I used CG-Bundler to keep the Rust code split across multiple files while still producing a single-file submission for CodinGame.

## Conclusion

This was my first CodinGame contest using a simulation-based approach, and I was surprised by how effective even my very simple BFS-based approach was. In future competitions I would like to experiment with MCTS and related techniques so my bots can factor in potential opponent moves.
