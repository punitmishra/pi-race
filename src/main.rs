//! ═══════════════════════════════════════════════════════════════
//!  THE RACE TO π  —  Algorithms Showdown
//!  Ramanujan · Euler/Basel · Wallis · Leibniz
//!
//!  Usage:
//!    pi-race                          # terminal output, standard tier
//!    pi-race --format markdown        # GitHub-flavored markdown tables
//!    pi-race --format json            # machine-readable JSON
//!    pi-race --tier deep              # nano|micro|standard|deep|extreme
//!    pi-race --tier extreme --format markdown
//! ═══════════════════════════════════════════════════════════════

use std::f64::consts::PI;
use std::time::{Duration, Instant};

// ─── ANSI palette ────────────────────────────────────────────────
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[38;5;196m";
const ORANGE: &str = "\x1b[38;5;214m";
const PURPLE: &str = "\x1b[38;5;135m";
const BLUE: &str = "\x1b[38;5;75m";
const GREEN: &str = "\x1b[38;5;82m";
const CYAN: &str = "\x1b[38;5;51m";
const GOLD: &str = "\x1b[38;5;220m";
const WHITE: &str = "\x1b[38;5;255m";
const GREY: &str = "\x1b[38;5;240m";

// ─── Precision tiers ─────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
enum Tier {
    Nano,
    Micro,
    Standard,
    Deep,
    Extreme,
}

impl Tier {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "nano" => Some(Tier::Nano),
            "micro" => Some(Tier::Micro),
            "standard" => Some(Tier::Standard),
            "deep" => Some(Tier::Deep),
            "extreme" => Some(Tier::Extreme),
            _ => None,
        }
    }

    fn iter_sets(self) -> &'static [u64] {
        match self {
            Tier::Nano => &[10, 100, 1_000, 10_000],
            Tier::Micro => &[10, 100, 1_000, 10_000, 100_000],
            Tier::Standard => &[10, 100, 1_000, 10_000, 100_000, 1_000_000],
            Tier::Deep => &[
                10, 100, 1_000, 10_000, 100_000, 1_000_000, 5_000_000, 10_000_000,
            ],
            Tier::Extreme => &[
                10, 100, 1_000, 10_000, 100_000, 1_000_000, 5_000_000, 10_000_000, 50_000_000,
            ],
        }
    }

    fn label(self) -> &'static str {
        match self {
            Tier::Nano => "nano",
            Tier::Micro => "micro",
            Tier::Standard => "standard",
            Tier::Deep => "deep",
            Tier::Extreme => "extreme",
        }
    }
}

// ─── Output formats ───────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
enum Format {
    Terminal,
    Markdown,
    Json,
}

impl Format {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "terminal" => Some(Format::Terminal),
            "markdown" | "md" => Some(Format::Markdown),
            "json" => Some(Format::Json),
            _ => None,
        }
    }
}

// ─── Algorithm ───────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
enum Algorithm {
    Ramanujan,
    Euler,
    Wallis,
    Leibniz,
}

const ALL_ALGOS: [Algorithm; 4] = [
    Algorithm::Ramanujan,
    Algorithm::Euler,
    Algorithm::Wallis,
    Algorithm::Leibniz,
];

impl Algorithm {
    fn color(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => RED,
            Algorithm::Euler => PURPLE,
            Algorithm::Wallis => ORANGE,
            Algorithm::Leibniz => BLUE,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "Ramanujan  (1914)",
            Algorithm::Euler => "Euler/Basel (1735)",
            Algorithm::Wallis => "Wallis     (1655)",
            Algorithm::Leibniz => "Leibniz    (1676)",
        }
    }
    fn short(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "Ramanujan",
            Algorithm::Euler => "Euler/Basel",
            Algorithm::Wallis => "Wallis",
            Algorithm::Leibniz => "Leibniz",
        }
    }
    fn formula(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "1/π = (2√2/9801) Σ (4k)!(1103+26390k) / (k!)⁴·396⁴ᵏ",
            Algorithm::Euler => "π   = √(6 · Σ 1/k²)  [Basel problem]",
            Algorithm::Wallis => "π/2 = Π (4k²)/(4k²-1)  [Wallis product]",
            Algorithm::Leibniz => "π   = 4 · Σ (-1)ᵏ/(2k+1)  [Leibniz series]",
        }
    }
    fn symbol(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "◆",
            Algorithm::Euler => "▲",
            Algorithm::Wallis => "●",
            Algorithm::Leibniz => "■",
        }
    }
    fn convergence_class(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "~8 digits/term",
            Algorithm::Euler => "O(1/n²)",
            Algorithm::Wallis => "O(1/√n)",
            Algorithm::Leibniz => "O(1/n)",
        }
    }
    fn compute(self, iters: u64) -> (f64, Duration) {
        let start = Instant::now();
        let v = match self {
            Algorithm::Ramanujan => ramanujan(iters),
            Algorithm::Euler => euler_basel(iters),
            Algorithm::Wallis => wallis(iters),
            Algorithm::Leibniz => leibniz(iters),
        };
        (v, start.elapsed())
    }
}

// ─── π algorithm implementations ─────────────────────────────────

/// Ramanujan 1914: ~8 new decimal digits per term.
/// 1/π = (2√2/9801) · Σ_{k=0}^{∞} (4k)!(1103 + 26390k) / ((k!)^4 · 396^{4k})
fn ramanujan(iters: u64) -> f64 {
    let factor = 2.0_f64.sqrt() * 2.0 / 9801.0;
    let mut sum = 0.0_f64;
    for k in 0..iters {
        let log_term =
            log_factorial(4 * k) - 4.0 * log_factorial(k) - (k as f64) * 4.0 * 396.0_f64.ln();
        if log_term < -700.0 {
            break;
        }
        let term = (1103.0 + 26390.0 * k as f64) * log_term.exp();
        sum += term;
        if term.abs() < 1e-300 {
            break;
        }
    }
    1.0 / (factor * sum)
}

/// Euler/Basel 1735: π = √(6 · Σ_{k=1}^{n} 1/k²). Converges as O(1/n²).
fn euler_basel(iters: u64) -> f64 {
    let mut s = 0.0_f64;
    for k in 1..=iters {
        s += 1.0 / (k as f64 * k as f64);
    }
    (6.0 * s).sqrt()
}

/// Wallis 1655: π/2 = Π_{k=1}^{n} 4k²/(4k²-1). Converges as O(1/√n).
fn wallis(iters: u64) -> f64 {
    let mut p = 1.0_f64;
    for k in 1..=iters {
        let k2 = k as f64 * k as f64;
        p *= 4.0 * k2 / (4.0 * k2 - 1.0);
    }
    2.0 * p
}

/// Leibniz 1676: π = 4 · Σ_{k=0}^{n} (-1)^k/(2k+1). Converges as O(1/n).
fn leibniz(iters: u64) -> f64 {
    let mut s = 0.0_f64;
    for k in 0..iters {
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        s += sign / (2.0 * k as f64 + 1.0);
    }
    4.0 * s
}

/// log(n!) = Σ_{i=1}^{n} log(i)
fn log_factorial(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    (1..=n).map(|i| (i as f64).ln()).sum()
}

// ─── Benchmark result ─────────────────────────────────────────────
struct BenchResult {
    algo: Algorithm,
    iters: u64,
    estimate: f64,
    error: f64,
    elapsed: Duration,
    digits: usize,
}

impl BenchResult {
    fn new(algo: Algorithm, iters: u64) -> Self {
        let (estimate, elapsed) = algo.compute(iters);
        let error = (estimate - PI).abs();
        let digits = if error == 0.0 {
            15
        } else {
            (-error.log10()).floor().max(0.0) as usize
        };
        BenchResult {
            algo,
            iters,
            estimate,
            error,
            elapsed,
            digits,
        }
    }
    fn elapsed_us(&self) -> f64 {
        self.elapsed.as_secs_f64() * 1e6
    }
    fn elapsed_pretty(&self) -> String {
        fmt_time(self.elapsed_us())
    }
}

// ─── Shared helpers ───────────────────────────────────────────────

fn fmt_iters(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        format!("{}", n)
    }
}

fn fmt_time(us: f64) -> String {
    if us < 1_000.0 {
        format!("{:.1}µs", us)
    } else if us < 1_000_000.0 {
        format!("{:.2}ms", us / 1_000.0)
    } else {
        format!("{:.3}s", us / 1_000_000.0)
    }
}

fn ansi_bar(val: usize, max: usize, width: usize, color: &str) -> String {
    let filled = (val * width / max).min(width);
    format!(
        "{}{}{}{}{}",
        color,
        "█".repeat(filled),
        GREY,
        "░".repeat(width - filled),
        RESET
    )
}

fn md_bar(val: usize, max: usize) -> String {
    // Unicode block progress bar for Markdown (renders in GitHub)
    let width = 20usize;
    let filled = (val * width / max).min(width);
    let pct = (val * 100 / max).min(100);
    format!(
        "{}{} {}%",
        "█".repeat(filled),
        "░".repeat(width - filled),
        pct
    )
}

// ─── Terminal renderer ────────────────────────────────────────────

fn terminal_run(results: &[Vec<BenchResult>], tier: Tier) {
    // Header
    println!();
    println!(
        "  {}{}╔══════════════════════════════════════════════════════════╗{}",
        BOLD, CYAN, RESET
    );
    println!(
        "  {}{}║        🔵  THE RACE TO π  —  Algorithms Showdown  🔵   ║{}",
        BOLD, CYAN, RESET
    );
    println!(
        "  {}{}╚══════════════════════════════════════════════════════════╝{}",
        BOLD, CYAN, RESET
    );
    println!();
    println!(
        "  {}TRUE π{} = {}{}3.14159265358979323846...{}  {}tier: {}{}",
        GREEN,
        RESET,
        BOLD,
        CYAN,
        RESET,
        DIM,
        tier.label(),
        RESET
    );
    println!();
    println!("  {}{}ALGORITHMS:{}", BOLD, GREY, RESET);
    for a in ALL_ALGOS {
        println!(
            "  {}{}{} {}{}{}  {}[{}]{}",
            a.color(),
            BOLD,
            a.symbol(),
            RESET,
            a.color(),
            a.label(),
            DIM,
            a.convergence_class(),
            RESET
        );
        println!("     {}{}{}", DIM, a.formula(), RESET);
    }
    println!();

    // Convergence table
    println!(
        "  {}{}─── CONVERGENCE TABLE ({} tier) ──────────────────────────────{}",
        BOLD,
        CYAN,
        tier.label(),
        RESET
    );
    println!();
    println!(
        "  {}{}  {:>7}  {:>5}  {:>5}  {:>5}  {:>5}  {:>8}  {:>8}  {:>8}  {:>8}{}",
        BOLD, WHITE, "ITERS", "RAM", "EUR", "WAL", "LBZ", "RAM_t", "EUR_t", "WAL_t", "LBZ_t", RESET
    );
    println!("  {}{}{}{}", DIM, GREY, "─".repeat(78), RESET);
    for (j, &iters) in tier.iter_sets().iter().enumerate() {
        let row: Vec<&BenchResult> = results.iter().map(|r| &r[j]).collect();
        print!("  {}{}{:>7}{}  ", GOLD, BOLD, fmt_iters(iters), RESET);
        for r in &row {
            print!("{}{}{:>4}dp{}  ", r.algo.color(), BOLD, r.digits, RESET);
        }
        for r in &row {
            print!("{}  {:>8}{}", DIM, r.elapsed_pretty(), RESET);
        }
        println!();
    }
    println!();

    // Visual race (at max tier)
    let last_j = tier.iter_sets().len() - 1;
    let max_iters = tier.iter_sets()[last_j];
    println!(
        "  {}{}─── ACCURACY RACE @ {} iterations ─────────────────────────────{}",
        BOLD,
        CYAN,
        fmt_iters(max_iters),
        RESET
    );
    println!();
    let max_digits = 15usize;
    let bw = 40;
    for (i, algo) in ALL_ALGOS.iter().enumerate() {
        let r = &results[i][last_j];
        let c = algo.color();
        println!(
            "  {}{}{} {:>19}{}",
            c,
            BOLD,
            algo.symbol(),
            algo.label(),
            RESET
        );
        println!(
            "    {}  {}{:>2} digits correct{}",
            ansi_bar(r.digits, max_digits, bw, c),
            BOLD,
            r.digits,
            RESET
        );
        println!(
            "    {}estimate: {}{:.15}{}  ±{:.2e}  in {}{}",
            DIM,
            c,
            r.estimate,
            RESET,
            r.error,
            r.elapsed_pretty(),
            RESET
        );
        println!();
    }

    // Verdict
    println!(
        "  {}{}─── VERDICT ────────────────────────────────────────────────────{}",
        BOLD, CYAN, RESET
    );
    println!();
    let medals = ["🥇", "🥈", "🥉", "   "];
    let mut ranked: Vec<(usize, usize, f64)> = ALL_ALGOS
        .iter()
        .enumerate()
        .map(|(i, _)| {
            (
                i,
                results[i][last_j].digits,
                results[i][last_j].elapsed_us(),
            )
        })
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.partial_cmp(&b.2).unwrap()));
    for (rank, &(i, digits, us)) in ranked.iter().enumerate() {
        let a = ALL_ALGOS[i];
        println!(
            "  {}  {}{}{} {:>19}{}  {:>2} correct digits  in {}",
            medals[rank],
            a.color(),
            BOLD,
            a.symbol(),
            RESET,
            a.label(),
            digits,
            fmt_time(us)
        );
    }
    println!();
    println!(
        "  {}{}◆ Ramanujan converges at ~8 digits/term — hits f64 ceiling in 2 iterations.{}",
        BOLD, RED, RESET
    );
    println!(
        "  {}  Leibniz needs 10^n iterations for n digits. Ramanujan k=10 > Leibniz k=1M.{}",
        DIM, RESET
    );
    println!();
    println!(
        "  {}{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}",
        BOLD, GREY, RESET
    );
    println!();
    println!(
        "  {}π  Ramanujan k=10:  {}{:.15}{}",
        DIM,
        GREEN,
        ramanujan(10),
        RESET
    );
    println!(
        "  {}π  Leibniz  1M:     {}{:.15}{}",
        DIM,
        BLUE,
        leibniz(1_000_000),
        RESET
    );
    println!("  {}π  true f64:        {}{:.15}{}", DIM, CYAN, PI, RESET);
    println!();
}

// ─── Markdown renderer ────────────────────────────────────────────

fn markdown_run(results: &[Vec<BenchResult>], tier: Tier) {
    let last_j = tier.iter_sets().len() - 1;
    let max_iters = tier.iter_sets()[last_j];
    let max_digits = 15usize;

    println!("## 🔵 The Race to π — Results");
    println!();
    println!(
        "> **Tier:** `{}`  &nbsp;|&nbsp;  **Max iterations:** `{}`  &nbsp;|&nbsp;  **True π:** `3.14159265358979323846...`",
        tier.label(),
        fmt_iters(max_iters)
    );
    println!();

    // Algorithm legend
    println!("### Algorithms");
    println!();
    println!("| Symbol | Algorithm | Year | Convergence | Formula |");
    println!("|--------|-----------|------|-------------|---------|");
    for a in ALL_ALGOS {
        println!(
            "| {} | **{}** | {} | `{}` | `{}` |",
            a.symbol(),
            a.short(),
            &a.label()[a.label().len() - 5..a.label().len() - 1],
            a.convergence_class(),
            a.formula()
        );
    }
    println!();

    // Convergence table
    println!("### Convergence Table");
    println!();
    println!("Correct decimal digits of π at each iteration count:");
    println!();
    print!("| Iterations |");
    for a in ALL_ALGOS {
        print!(" {} digits |", a.short());
    }
    println!();
    print!("|------------|");
    for _ in ALL_ALGOS {
        print!("-----------|");
    }
    println!();
    for (j, &iters) in tier.iter_sets().iter().enumerate() {
        print!("| `{}` |", fmt_iters(iters));
        for i in 0..ALL_ALGOS.len() {
            let r = &results[i][j];
            print!(" {} |", r.digits);
        }
        println!();
    }
    println!();

    // Timing table
    println!("### Timing Benchmarks");
    println!();
    print!("| Iterations |");
    for a in ALL_ALGOS {
        print!(" {} |", a.short());
    }
    println!();
    print!("|------------|");
    for _ in ALL_ALGOS {
        print!("--------|");
    }
    println!();
    for (j, &iters) in tier.iter_sets().iter().enumerate() {
        print!("| `{}` |", fmt_iters(iters));
        for i in 0..ALL_ALGOS.len() {
            let r = &results[i][j];
            print!(" `{}` |", r.elapsed_pretty());
        }
        println!();
    }
    println!();

    // Accuracy race visual
    println!("### Accuracy Race @ `{}`", fmt_iters(max_iters));
    println!();
    println!("| Rank | Algorithm | Digits | Accuracy | Estimate | Error | Time |");
    println!("|------|-----------|--------|----------|----------|-------|------|");

    let mut ranked: Vec<usize> = (0..ALL_ALGOS.len()).collect();
    ranked.sort_by(|&a, &b| {
        results[b][last_j]
            .digits
            .cmp(&results[a][last_j].digits)
            .then(
                results[a][last_j]
                    .elapsed_us()
                    .partial_cmp(&results[b][last_j].elapsed_us())
                    .unwrap(),
            )
    });
    let medals = ["🥇", "🥈", "🥉", "4th"];
    for (rank, &i) in ranked.iter().enumerate() {
        let a = ALL_ALGOS[i];
        let r = &results[i][last_j];
        println!(
            "| {} | **{}** {} | `{}` | `{}` | `{:.15}` | `{:.2e}` | `{}` |",
            medals[rank],
            a.short(),
            a.symbol(),
            r.digits,
            md_bar(r.digits, max_digits),
            r.estimate,
            r.error,
            r.elapsed_pretty()
        );
    }
    println!();

    // Key insight
    println!("### Key Insight");
    println!();
    println!(
        "**Ramanujan** exhausts IEEE 754 double precision (~15 sig figs) in just **2 iterations**."
    );
    println!("Leibniz at `1M` iterations cannot match Ramanujan at `k=10`.");
    println!();
    println!("| | Ramanujan `k=10` | Leibniz `k=1,000,000` | True π |");
    println!("|---|---|---|---|");
    println!(
        "| Estimate | `{:.15}` | `{:.15}` | `{:.15}` |",
        ramanujan(10),
        leibniz(1_000_000),
        PI
    );
    println!(
        "| Error | `{:.2e}` | `{:.2e}` | — |",
        (ramanujan(10) - PI).abs(),
        (leibniz(1_000_000) - PI).abs()
    );
}

// ─── JSON renderer ────────────────────────────────────────────────

fn json_run(results: &[Vec<BenchResult>], tier: Tier) {
    let last_j = tier.iter_sets().len() - 1;
    println!("{{");
    println!("  \"tier\": \"{}\",", tier.label());
    println!("  \"true_pi\": {:.15},", PI);
    println!("  \"algorithms\": [");
    for (ai, algo) in ALL_ALGOS.iter().enumerate() {
        let comma_a = if ai < ALL_ALGOS.len() - 1 { "," } else { "" };
        println!("    {{");
        println!("      \"name\": \"{}\",", algo.short());
        println!("      \"formula\": \"{}\",", algo.formula());
        println!("      \"convergence\": \"{}\",", algo.convergence_class());
        println!("      \"runs\": [");
        for (j, &iters) in tier.iter_sets().iter().enumerate() {
            let r = &results[ai][j];
            let comma_r = if j < tier.iter_sets().len() - 1 {
                ","
            } else {
                ""
            };
            println!("        {{");
            println!("          \"iters\": {},", iters);
            println!("          \"estimate\": {:.15},", r.estimate);
            println!("          \"error\": {:.6e},", r.error);
            println!("          \"digits_correct\": {},", r.digits);
            println!("          \"elapsed_us\": {:.3}", r.elapsed_us());
            println!("        }}{}", comma_r);
        }
        println!("      ],");
        println!("      \"best_digits\": {},", results[ai][last_j].digits);
        println!(
            "      \"best_estimate\": {:.15}",
            results[ai][last_j].estimate
        );
        println!("    }}{}", comma_a);
    }
    println!("  ]");
    println!("}}");
}

// ─── CLI arg parser ───────────────────────────────────────────────

struct Config {
    format: Format,
    tier: Tier,
}

impl Config {
    fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut format = Format::Terminal;
        let mut tier = Tier::Standard;
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--format" | "-f" => {
                    if let Some(v) = args.get(i + 1) {
                        if let Some(f) = Format::from_str(v) {
                            format = f;
                        } else {
                            eprintln!("Unknown format '{}'. Use: terminal|markdown|json", v);
                            std::process::exit(1);
                        }
                        i += 1;
                    }
                }
                "--tier" | "-t" => {
                    if let Some(v) = args.get(i + 1) {
                        if let Some(t) = Tier::from_str(v) {
                            tier = t;
                        } else {
                            eprintln!(
                                "Unknown tier '{}'. Use: nano|micro|standard|deep|extreme",
                                v
                            );
                            std::process::exit(1);
                        }
                        i += 1;
                    }
                }
                "--help" | "-h" => {
                    println!("pi-race — The Race to π");
                    println!();
                    println!("  --format|-f  terminal|markdown|json  (default: terminal)");
                    println!(
                        "  --tier|-t    nano|micro|standard|deep|extreme  (default: standard)"
                    );
                    println!();
                    println!("  Tiers (max iterations):");
                    println!("    nano     → 10K");
                    println!("    micro    → 100K");
                    println!("    standard → 1M  (default)");
                    println!("    deep     → 10M");
                    println!("    extreme  → 50M");
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        Config { format, tier }
    }
}

// ─── Main ─────────────────────────────────────────────────────────

fn main() {
    let cfg = Config::from_args();

    if cfg.format == Format::Terminal {
        eprintln!(
            "  Computing 4 algorithms × {} tier ({} iter sets)...",
            cfg.tier.label(),
            cfg.tier.iter_sets().len()
        );
    }

    let results: Vec<Vec<BenchResult>> = ALL_ALGOS
        .iter()
        .map(|&algo| {
            cfg.tier
                .iter_sets()
                .iter()
                .map(|&n| BenchResult::new(algo, n))
                .collect()
        })
        .collect();

    match cfg.format {
        Format::Terminal => terminal_run(&results, cfg.tier),
        Format::Markdown => markdown_run(&results, cfg.tier),
        Format::Json => json_run(&results, cfg.tier),
    }
}
