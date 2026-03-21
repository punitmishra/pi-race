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
