### Benchmarking
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

2.00
```
calculate_power_consumption small
                        time:   [48.063 ns 48.099 ns 48.159 ns]
                        change: [-76.126% -75.847% -75.634%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

calculate_power_consumption large
                        time:   [1.5907 us 1.5968 us 1.6062 us]
                        change: [-96.965% -96.915% -96.879%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

calculate_life_system small
                        time:   [199.04 ns 199.40 ns 199.83 ns]
                        change: [-6.2822% -4.5811% -3.2059%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

calculate_life_system large
                        time:   [2.9362 us 2.9398 us 2.9444 us]
                        change: [-0.7793% +0.4736% +1.5470%] (p = 0.45 > 0.05)
                        No change in performance detected.
```

const generics
```
calculate_power_consumption small
                        time:   [37.664 ns 37.733 ns 37.818 ns]
                        change: [-0.3764% +0.2208% +0.8348%] (p = 0.48 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

calculate_power_consumption large
                        time:   [4.8193 us 4.8439 us 4.8686 us]
                        change: [-1.1925% +0.5288% +2.0714%] (p = 0.55 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

calculate_life_system small
                        time:   [203.60 ns 203.77 ns 203.97 ns]
                        change: [-0.5870% -0.3080% -0.0430%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

calculate_life_system large
                        time:   [2.9271 us 2.9397 us 2.9552 us]
                        change: [-4.2523% -2.6852% -1.3358%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
```
