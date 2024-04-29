# Benchmark Result

## N=100

```bash
hyhyperfine --warmup 5 --runs 10   \
  "python simple/rs_bleuscore.py 100" \
  "python simple/local_hf_bleu.py 100" \
  "python simple/sacre_bleu.py 100"   \
  "python simple/hf_evaluate.py 100"

Benchmark 1: python simple/rs_bleuscore.py 100
  Time (mean ± σ):      19.0 ms ±   2.6 ms    [User: 17.8 ms, System: 5.3 ms]
  Range (min … max):    14.8 ms …  23.2 ms    10 runs

Benchmark 2: python simple/local_hf_bleu.py 100
  Time (mean ± σ):      21.5 ms ±   2.2 ms    [User: 19.0 ms, System: 2.5 ms]
  Range (min … max):    16.8 ms …  24.1 ms    10 runs

Benchmark 3: python simple/sacre_bleu.py 100
  Time (mean ± σ):      45.9 ms ±   2.2 ms    [User: 38.7 ms, System: 7.1 ms]
  Range (min … max):    43.5 ms …  50.9 ms    10 runs

Benchmark 4: python simple/hf_evaluate.py 100
  Time (mean ± σ):      4.504 s ±  0.429 s    [User: 0.762 s, System: 0.823 s]
  Range (min … max):    4.163 s …  5.446 s    10 runs

Summary
  python simple/rs_bleuscore.py 100 ran
    1.13 ± 0.20 times faster than python simple/local_hf_bleu.py 100
    2.42 ± 0.35 times faster than python simple/sacre_bleu.py 100
  237.68 ± 39.88 times faster than python simple/hf_evaluate.py 100
```

## N = 1K ~ 1M

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `python simple/rs_bleuscore.py 1000` | 20.3 ± 1.3 | 18.2 | 21.4 | 1.00 |
| `python simple/local_hf_bleu.py 1000` | 45.8 ± 1.2 | 44.2 | 47.5 | 2.26 ± 0.16 |
| `python simple/rs_bleuscore.py 10000` | 37.8 ± 1.5 | 35.9 | 39.5 | 1.87 ± 0.14 |
| `python simple/local_hf_bleu.py 10000` | 295.0 ± 5.9 | 288.6 | 304.2 | 14.55 ± 0.98 |
| `python simple/rs_bleuscore.py 100000` | 219.6 ± 3.3 | 215.3 | 224.0 | 10.83 ± 0.72 |
| `python simple/local_hf_bleu.py 100000` | 2781.4 ± 42.2 | 2723.1 | 2833.0 | 137.13 ± 9.10 |
| `python simple/rs_bleuscore.py 1000000` | 2048.8 ± 31.4 | 2013.2 | 2090.3 | 101.01 ± 6.71 |
| `python simple/local_hf_bleu.py 1000000` | 28285.3 ± 100.9 | 28182.1 | 28396.1 | 1394.51 ± 90.21 |




### N=1000
```bash
hyhyperfine --warmup 5 --runs 10   \
  "python simple/rs_bleuscore.py 1000" \
  "python simple/local_hf_bleu.py 1000" \

Benchmark 1: python simple/rs_bleuscore.py 1000
  Time (mean ± σ):      20.8 ms ±   2.3 ms    [User: 31.5 ms, System: 11.3 ms]
  Range (min … max):    17.4 ms …  24.1 ms    10 runs

Benchmark 2: python simple/local_hf_bleu.py 1000
  Time (mean ± σ):      44.8 ms ±   2.1 ms    [User: 41.3 ms, System: 3.4 ms]
  Range (min … max):    41.2 ms …  48.0 ms    10 runs

Summary
  python simple/rs_bleuscore.py 1000 ran
    2.15 ± 0.26 times faster than python simple/local_hf_bleu.py 1000
```

### N=10,000
```bash
hyhyperfine --warmup 5 --runs 10   \
  "python simple/rs_bleuscore.py 10000" \
  "python simple/local_hf_bleu.py 10000" \

Benchmark 1: python simple/rs_bleuscore.py 10000
  Time (mean ± σ):      38.3 ms ±   1.5 ms    [User: 132.4 ms, System: 102.2 ms]
  Range (min … max):    36.3 ms …  40.3 ms    10 runs

Benchmark 2: python simple/local_hf_bleu.py 10000
  Time (mean ± σ):     298.9 ms ±   6.2 ms    [User: 295.5 ms, System: 3.1 ms]
  Range (min … max):   289.6 ms … 312.2 ms    10 runs

Summary
  python simple/rs_bleuscore.py 10000 ran
    7.79 ± 0.35 times faster than python simple/local_hf_bleu.py 10000
```

### N=100,000
```bash
hyhyperfine --warmup 5 --runs 10   \
  "python simple/rs_bleuscore.py 100000" \
  "python simple/local_hf_bleu.py 100000" \

Benchmark 1: python simple/rs_bleuscore.py 100000
  Time (mean ± σ):     222.2 ms ±   3.5 ms    [User: 962.7 ms, System: 1107.9 ms]
  Range (min … max):   216.0 ms … 228.7 ms    10 runs

Benchmark 2: python simple/local_hf_bleu.py 100000
  Time (mean ± σ):      2.784 s ±  0.048 s    [User: 2.773 s, System: 0.010 s]
  Range (min … max):    2.738 s …  2.896 s    10 runs

Summary
  python simple/rs_bleuscore.py 100000 ran
   12.53 ± 0.29 times faster than python simple/local_hf_bleu.py 100000
```

### N=1,000,000
```bash
hyhyperfine --warmup 1 --runs 3   \
  "python simple/rs_bleuscore.py 1000000" \
  "python simple/local_hf_bleu.py 1000000"
Benchmark 1: python simple/rs_bleuscore.py 1000000
  Time (mean ± σ):      2.018 s ±  0.042 s    [User: 10.077 s, System: 10.327 s]
  Range (min … max):    1.988 s …  2.066 s    3 runs

Benchmark 2: python simple/local_hf_bleu.py 1000000
  Time (mean ± σ):     27.760 s ±  0.373 s    [User: 27.687 s, System: 0.065 s]
  Range (min … max):   27.335 s … 28.036 s    3 runs

Summary
  python simple/rs_bleuscore.py 1000000 ran
   13.76 ± 0.34 times faster than python simple/local_hf_bleu.py 1000000
```
