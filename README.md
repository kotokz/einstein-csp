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

## Single thread
Benchmark 1: ./target/release/einstein-csp
  Time (mean ± σ):      44.8 ms ±   0.9 ms    [User: 35.7 ms, System: 1.0 ms]
  Range (min … max):    43.6 ms …  49.2 ms    60 runs

## multi thread
❯ hyperfine ./target/release/einstein-csp
Benchmark 1: ./target/release/einstein-csp
  Time (mean ± σ):      20.4 ms ±   2.0 ms    [User: 82.0 ms, System: 32.2 ms]
  Range (min … max):    16.7 ms …  28.3 ms    114 runs

## windows
PS D:\projects\rust\einstein-csp> hyperfine .\target\release\einstein-csp.exe     
Benchmark 1: .\target\release\einstein-csp.exe
  Time (mean ± σ):      14.7 ms ±   1.4 ms    [User: 0.0 ms, System: 0.0 ms]      
  Range (min … max):    11.2 ms …  20.5 ms    157 runs

```
