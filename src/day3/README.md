# Raspberry Pi 4 (4 GB) Benchmarks

Running benches\aoc_benchmark.rs (target\release\deps\aoc_benchmark-9acc3cb79794a8e2.exe)
day3 input parser       time:   [478.89 ps 481.27 ps 484.14 ps]
                        change: [-0.1241% +0.5798% +1.5071%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

day3 part 1             time:   [611.92 µs 613.83 µs 615.75 µs]
                        change: [+14.894% +15.558% +16.223%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

day3 part 2             time:   [604.87 µs 612.19 µs 621.58 µs]
                        change: [+19.554% +21.854% +24.622%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

day3 part1_regex        time:   [542.49 µs 545.25 µs 548.18 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

day3 part2_regex        time:   [669.95 µs 674.24 µs 679.01 µs]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe