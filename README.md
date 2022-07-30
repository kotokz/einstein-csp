# einstein-csp

Einstein's Puzzle
<https://web.stanford.edu/~laurik/fsmbook/examples/Einstein'sPuzzle.html>

```  bash
❯ ./target/release/einstein-csp
+--------+-----------+--------+---------+-------+
| House  | Nation    | Drink  | Cigar   | Pet   |
+--------+-----------+--------+---------+-------+
| Yellow | Norwegian | Water  | Dunhill | Cat   |
+--------+-----------+--------+---------+-------+
| Blue   | Danish    | Tea    | Blend   | Horse |
+--------+-----------+--------+---------+-------+
| Red    | British   | Milk   | Pall    | Bird  |
+--------+-----------+--------+---------+-------+
| Green  | German    | Coffee | Prince  | Fish  |
+--------+-----------+--------+---------+-------+
| White  | Swedish   | Beer   | Master  | Dog   |
+--------+-----------+--------+---------+-------+

❯ hyperfine ./target/release/einstein-csp
Benchmark 1: ./target/release/einstein-csp
  Time (mean ± σ):      44.8 ms ±   0.9 ms    [User: 35.7 ms, System: 1.0 ms]
  Range (min … max):    43.6 ms …  49.2 ms    60 runs
```
