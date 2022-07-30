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
  Time (mean ± σ):      56.0 ms ±   1.7 ms    [User: 47.0 ms, System: 0.4 ms]
  Range (min … max):    53.9 ms …  61.0 ms    49 runs
```
