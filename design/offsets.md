# Neighbor Offsets

The center cell is located at `(row, col)`. The eight surrounding cells use these row and column offsets:

| (-1, -1) | (-1, 0) | (-1, 1) |
|---|---|---|
| (0, -1) | CENTER | (0, 1) |
| (1, -1) | (1, 0) | (1, 1) |

The center cell `(0, 0)` is not counted as its own neighbor.

Negative row offsets move up, positive row offsets move down, negative column offsets move left, and positive column offsets move right.
