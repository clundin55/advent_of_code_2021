### Introduction

Advent of Code 2021 Solutions in Rust.

By Carl Lundin

#### Day Three

**Benchmarking**
Initial attempt:

```
calculate_power_consumption small
                        time:   [197.97 ns 199.31 ns 201.64 ns]
                        change: [-3.3375% -2.0943% -0.7200%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

calculate_power_consumption large
                        time:   [51.406 us 52.052 us 53.343 us]
                        change: [-0.9841% +0.0400% +1.4177%] (p = 0.96 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

calculate_life_system small
                        time:   [205.77 ns 209.06 ns 215.60 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

calculate_life_system large
                        time:   [2.9193 us 2.9407 us 2.9643 us]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

