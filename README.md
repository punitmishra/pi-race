# 🔵 The Race to π — Algorithms Showdown

[![Race to Pi](https://github.com/YOUR_USERNAME/pi-race/actions/workflows/pi-race.yml/badge.svg)](https://github.com/YOUR_USERNAME/pi-race/actions/workflows/pi-race.yml)

A **zero-dependency Rust CLI** benchmarker for 4 legendary π algorithms, competing head-to-head on accuracy and speed.

```
  ╔══════════════════════════════════════════════════════════╗
  ║        🔵  THE RACE TO π  —  Algorithms Showdown  🔵   ║
  ╚══════════════════════════════════════════════════════════╝
```

## Algorithms

|Symbol|Algorithm      |Year|Convergence     |Formula                                              |
|------|---------------|----|----------------|-----------------------------------------------------|
|◆     |**Ramanujan**  |1914|~8 digits/term ⚡|`1/π = (2√2/9801) Σ (4k)!(1103+26390k) / (k!)⁴·396⁴ᵏ`|
|▲     |**Euler/Basel**|1735|O(1/n²)         |`π = √(6 · Σ 1/k²)`                                  |
|●     |**Wallis**     |1655|O(1/√n)         |`π/2 = Π (4k²)/(4k²-1)`                              |
|■     |**Leibniz**    |1676|O(1/n) 🐌        |`π = 4 · Σ (-1)ᵏ/(2k+1)`                             |

## Run locally

```bash
git clone https://github.com/YOUR_USERNAME/pi-race
cd pi-race
cargo run --release
```

**Requirements:** Rust stable (`rustup` or system package). No external crates.

## CI / GitHub Actions

Three jobs run on every push:

|Job                   |Trigger                        |What it does                                                                   |
|----------------------|-------------------------------|-------------------------------------------------------------------------------|
|**Algorithm Showdown**|Every push + PR                |fmt check → clippy → release build → **run the race** → upload results artifact|
|**Benchmark**         |Weekly (Mon 00:00 UTC) + manual|10-run timing table posted to job summary                                      |
|**Cross-compile**     |Every push                     |Builds for Linux x86_64, macOS arm64, Windows x86_64                           |

### Viewing results

After any push, go to **Actions → Algorithm Showdown → Summary** to see the full race output rendered in the job summary. Or download the `pi-race-results-N` artifact.

## Architecture

```
src/main.rs
│
├── Algorithm enum          (Ramanujan | Euler | Wallis | Leibniz)
│   ├── color()             ANSI terminal color per algo
│   ├── label() / formula() display metadata
│   └── compute(iters)      timed execution → (f64, Duration)
│
├── Implementations
│   ├── ramanujan(n)        log-space computation via log_gamma (Lanczos)
│   ├── euler_basel(n)      Σ 1/k², then √(6·sum)
│   ├── wallis(n)           running product
│   └── leibniz(n)          alternating series with Kahan-safe accumulation
│
├── BenchResult             iters → estimate, error, elapsed, digits_correct
│
└── Rendering
    ├── print_header()
    ├── print_convergence() 6×4 table: 10 / 100 / 1K / 10K / 100K / 1M iters
    ├── print_visual_race() block bar charts (█░░ accuracy bars)
    └── print_verdict()     ranked podium + key insight
```

## Key insight

Ramanujan’s series exhausts **f64 double precision (~15 sig figs) in just 2 iterations**. Leibniz at 1,000,000 iterations can’t match Ramanujan at k=10. To go beyond 15 digits you’d need arbitrary-precision arithmetic (e.g. the [`rug`](https://crates.io/crates/rug) crate wrapping GMP).

## Extending

Want to add more algorithms? Add a variant to the `Algorithm` enum, implement the `compute` arm, add `color()` / `label()` / `formula()` entries, and it’ll automatically appear in all three output sections.

Candidates: **Chudnovsky** (1988, ~14 digits/term, used by `y-cruncher`), **BBP formula** (1995, digit-extraction), **Machin’s formula** (1706).

## License

MIT


<!-- RACE_RESULTS_START -->

## 🏁 Latest Race Results

> 🕐 **Updated:** 2026-06-22 04:39 UTC &nbsp;|&nbsp; **Commit:** [`70033b2`](https://github.com/punitmishra/pi-race/actions/runs/27929983315) &nbsp;|&nbsp; **Winner:** **Ramanujan** with `15` correct digits &nbsp;|&nbsp; **Tier:** `extreme`

### Quick Look

| Algorithm | Best Digits | Estimate | Error |
|-----------|-------------|----------|-------|
| 🥇 **Ramanujan** | `15` | `3.141592653589793` | `0.00e+00` |
| 🥈 **Euler/Basel** | `7` | `3.141592634444062` | `1.91e-08` |
| 🥉 **Wallis** | `7` | `3.141592643066262` | `1.05e-08` |
| 4️⃣ **Leibniz** | `7` | `3.141592633590251` | `2.00e-08` |

<details>
<summary>📊 Full extreme-tier report (click to expand)</summary>

> **Tier:** `extreme`  &nbsp;|&nbsp;  **Max iterations:** `50M`  &nbsp;|&nbsp;  **True π:** `3.14159265358979323846...`

### Algorithms

| Symbol | Algorithm | Year | Convergence | Formula |
|--------|-----------|------|-------------|---------|
| ◆ | **Ramanujan** | 1914 | `~8 digits/term` | `1/π = (2√2/9801) Σ (4k)!(1103+26390k) / (k!)⁴·396⁴ᵏ` |
| ▲ | **Euler/Basel** | 1735 | `O(1/n²)` | `π   = √(6 · Σ 1/k²)  [Basel problem]` |
| ● | **Wallis** | 1655 | `O(1/√n)` | `π/2 = Π (4k²)/(4k²-1)  [Wallis product]` |
| ■ | **Leibniz** | 1676 | `O(1/n)` | `π   = 4 · Σ (-1)ᵏ/(2k+1)  [Leibniz series]` |

### Convergence Table

Correct decimal digits of π at each iteration count:

| Iterations | Ramanujan digits | Euler/Basel digits | Wallis digits | Leibniz digits |
|------------|-----------|-----------|-----------|-----------|
| `10` | 15 | 1 | 1 | 1 |
| `100` | 15 | 2 | 2 | 2 |
| `1K` | 15 | 3 | 3 | 3 |
| `10K` | 15 | 4 | 4 | 4 |
| `100K` | 15 | 5 | 5 | 4 |
| `1M` | 15 | 6 | 6 | 5 |
| `5M` | 15 | 6 | 6 | 6 |
| `10M` | 15 | 7 | 7 | 6 |
| `50M` | 15 | 7 | 7 | 7 |

### Timing Benchmarks

| Iterations | Ramanujan | Euler/Basel | Wallis | Leibniz |
|------------|--------|--------|--------|--------|
| `10` | `10.1µs` | `0.1µs` | `0.1µs` | `0.1µs` |
| `100` | `23.2µs` | `0.2µs` | `0.2µs` | `0.2µs` |
| `1K` | `20.4µs` | `1.4µs` | `1.5µs` | `1.8µs` |
| `10K` | `20.5µs` | `13.9µs` | `14.4µs` | `16.6µs` |
| `100K` | `20.1µs` | `169.4µs` | `160.8µs` | `165.7µs` |
| `1M` | `20.0µs` | `1.40ms` | `1.49ms` | `1.75ms` |
| `5M` | `20.0µs` | `7.07ms` | `7.04ms` | `7.08ms` |
| `10M` | `20.1µs` | `14.07ms` | `18.76ms` | `14.20ms` |
| `50M` | `20.0µs` | `70.25ms` | `96.19ms` | `70.83ms` |

### Accuracy Race @ `50M`

| Rank | Algorithm | Digits | Accuracy | Estimate | Error | Time |
|------|-----------|--------|----------|----------|-------|------|
| 🥇 | **Ramanujan** ◆ | `15` | `████████████████████ 100%` | `3.141592653589793` | `0.00e0` | `20.0µs` |
| 🥈 | **Euler/Basel** ▲ | `7` | `█████████░░░░░░░░░░░ 46%` | `3.141592634444062` | `1.91e-8` | `70.25ms` |
| 🥉 | **Leibniz** ■ | `7` | `█████████░░░░░░░░░░░ 46%` | `3.141592633590251` | `2.00e-8` | `70.83ms` |
| 4th | **Wallis** ● | `7` | `█████████░░░░░░░░░░░ 46%` | `3.141592643066262` | `1.05e-8` | `96.19ms` |

### Key Insight

**Ramanujan** exhausts IEEE 754 double precision (~15 sig figs) in just **2 iterations**.
Leibniz at `1M` iterations cannot match Ramanujan at `k=10`.

| | Ramanujan `k=10` | Leibniz `k=1,000,000` | True π |
|---|---|---|---|
| Estimate | `3.141592653589793` | `3.141591653589774` | `3.141592653589793` |
| Error | `0.00e0` | `1.00e-6` | — |

</details>

<!-- RACE_RESULTS_END -->













