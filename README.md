# Benchmarks

Ran on a Mac Pro M2:

```
test test::ahash::bench_10          ... bench:         650 ns/iter (+/- 27)
test test::ahash::bench_100         ... bench:       6,044 ns/iter (+/- 247)
test test::ahash::bench_10_000      ... bench:     636,383 ns/iter (+/- 45,962)
test test::ahash::bench_bad_100     ... bench:      68,421 ns/iter (+/- 1,773)
test test::ahash::bench_bad_10_000  ... bench:   7,263,262 ns/iter (+/- 239,556)
test test::naive::bench_10          ... bench:         624 ns/iter (+/- 9)
test test::naive::bench_100         ... bench:       7,016 ns/iter (+/- 359)
test test::naive::bench_10_000      ... bench:  15,477,025 ns/iter (+/- 153,631)
test test::naive::bench_bad_100     ... bench:      65,838 ns/iter (+/- 3,308)
test test::naive::bench_bad_10_000  ... bench:  23,860,137 ns/iter (+/- 1,324,424)
test test::sorted::bench_10         ... bench:         652 ns/iter (+/- 10)
test test::sorted::bench_100        ... bench:       5,178 ns/iter (+/- 310)
test test::sorted::bench_10_000     ... bench:     483,654 ns/iter (+/- 11,294)
test test::sorted::bench_bad_100    ... bench:      65,100 ns/iter (+/- 1,156)
test test::sorted::bench_bad_10_000 ... bench:   6,624,381 ns/iter (+/- 252,992)
```
