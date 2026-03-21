/// ═══════════════════════════════════════════════════════════════
///  THE RACE TO π  —  Algorithms Showdown
///  Ramanujan · Euler/Basel · Wallis · Leibniz
/// ═══════════════════════════════════════════════════════════════
use std::f64::consts::PI;
use std::time::{Duration, Instant};

// ─── ANSI Colors ─────────────────────────────────────────────────
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

// ─── Algorithm enum ───────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
enum Algorithm {
    Ramanujan,
    Euler,
    Wallis,
    Leibniz,
}

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
    fn formula(self) -> &'static str {
        match self {
            Algorithm::Ramanujan => "1/π = (2√2/9801) Σ (4k)!(1103+26390k) / (k!)⁴·396⁴ᵏ",
            Algorithm::Euler => "π   = √(6 · Σ 1/k²)                  [Basel problem]",
            Algorithm::Wallis => "π/2 = Π (4k²)/(4k²-1)                [Wallis product]",
            Algorithm::Leibniz => "π   = 4 · Σ (-1)ᵏ/(2k+1)             [Leibniz series]",
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

// ─── π algorithms ────────────────────────────────────────────────

/// Ramanujan 1914 — ~8 new digits per term
/// 1/π = (2√2/9801) Σ (4k)!(1103+26390k) / ((k!)^4 · 396^{4k})
fn ramanujan(iters: u64) -> f64 {
    let factor = 2.0_f64.sqrt() * 2.0 / 9801.0;
    let mut sum = 0.0_f64;
    for k in 0..iters {
        let log_4k = log_gamma(4 * k + 1);
        let log_k4 = 4.0 * log_gamma(k + 1);
        let log_pow = (k as f64) * 4.0 * 396.0_f64.ln();
        let log_term = log_4k - log_k4 - log_pow;
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

/// Euler Basel — converges as O(1/n²)
/// π = √(6 · Σ_{k=1}^{n} 1/k²)
fn euler_basel(iters: u64) -> f64 {
    let mut s = 0.0_f64;
    for k in 1..=iters {
        s += 1.0 / (k as f64 * k as f64);
    }
    (6.0 * s).sqrt()
}

/// Wallis product — converges very slowly O(1/√n)
/// π/2 = Π_{k=1}^{n} 4k²/(4k²-1)
fn wallis(iters: u64) -> f64 {
    let mut p = 1.0_f64;
    for k in 1..=iters {
        let k2 = k as f64 * k as f64;
        p *= 4.0 * k2 / (4.0 * k2 - 1.0);
    }
    2.0 * p
}

/// Leibniz/Gregory-Madhava — converges as O(1/n)
/// π = 4 · Σ_{k=0}^{n} (-1)^k/(2k+1)
fn leibniz(iters: u64) -> f64 {
    let mut s = 0.0_f64;
    for k in 0..iters {
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        s += sign / (2.0 * k as f64 + 1.0);
    }
    4.0 * s
}

/// Log-gamma via Lanczos approximation (Spouge variant)
fn log_gamma(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    // Use log-sum for large factorials
    (1..=n).map(|i| (i as f64).ln()).sum()
}

// ─── Benchmark result ─────────────────────────────────────────────
#[allow(dead_code)]
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
}

// ─── Rendering ───────────────────────────────────────────────────

fn bar(val: usize, max: usize, width: usize, color: &str) -> String {
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
    if us < 1000.0 {
        format!("{:.1}μs", us)
    } else {
        format!("{:.2}ms", us / 1000.0)
    }
}

fn print_header() {
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
        "  {}TRUE π{} = {}{}3.14159265358979323846...{}",
        GREEN, RESET, BOLD, CYAN, RESET
    );
    println!();
    println!("  {}{}ALGORITHMS:{}", BOLD, GREY, RESET);
    let algos = [
        Algorithm::Ramanujan,
        Algorithm::Euler,
        Algorithm::Wallis,
        Algorithm::Leibniz,
    ];
    for a in algos {
        println!(
            "  {}{}{} {}{}{}",
            a.color(),
            BOLD,
            a.symbol(),
            RESET,
            a.color(),
            a.label()
        );
        println!("     {}{}", DIM, a.formula());
        print!("{}", RESET);
    }
    println!();
}

fn print_convergence(results: &[Vec<BenchResult>]) {
    println!(
        "  {}{}─── CONVERGENCE TABLE ────────────────────────────────────────{}",
        BOLD, CYAN, RESET
    );
    println!();
    println!(
        "  {}{}  {:>6}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}{}",
        BOLD, WHITE, "ITERS", "RAM", "EUR", "WAL", "LBZ", "RAM_t", "EUR_t", "WAL_t", "LBZ_t", RESET
    );
    println!("  {}{}{}{}", DIM, GREY, "─".repeat(70), RESET);

    let iter_sets = [10u64, 100, 1_000, 10_000, 100_000, 1_000_000];
    for (j, &iters) in iter_sets.iter().enumerate() {
        let row: Vec<&BenchResult> = results.iter().map(|r| &r[j]).collect();
        print!("  {}{}{:>6}{}  ", GOLD, BOLD, fmt_iters(iters), RESET);
        for r in &row {
            print!("{}{}{:>3}dp{}  ", r.algo.color(), BOLD, r.digits, RESET);
        }
        for r in &row {
            print!("{}{}  ", DIM, fmt_time(r.elapsed_us()));
        }
        println!("{}", RESET);
    }
    println!();
}

fn print_visual_race(results: &[Vec<BenchResult>]) {
    println!(
        "  {}{}─── ACCURACY RACE @ 1,000,000 iterations ─────────────────────{}",
        BOLD, CYAN, RESET
    );
    println!();

    let algos = [
        Algorithm::Ramanujan,
        Algorithm::Euler,
        Algorithm::Wallis,
        Algorithm::Leibniz,
    ];
    let max_digits = 15;
    let bw = 44;

    for (i, algo) in algos.iter().enumerate() {
        let r = &results[i][5];
        let c = algo.color();
        println!(
            "  {}{}{} {:>18}{}",
            c,
            BOLD,
            algo.symbol(),
            algo.label(),
            RESET
        );
        println!(
            "  {}  {}  {}{:>2} digits correct{}",
            " ".repeat(2),
            bar(r.digits, max_digits, bw, c),
            BOLD,
            r.digits,
            RESET
        );
        println!(
            "  {}  estimate: {}{:.15}{}  ±{:.2e}{}",
            " ".repeat(2),
            c,
            r.estimate,
            RESET,
            r.error,
            RESET
        );
        println!();
    }
}

fn print_verdict(results: &[Vec<BenchResult>]) {
    println!(
        "  {}{}─── VERDICT ───────────────────────────────────────────────────{}",
        BOLD, CYAN, RESET
    );
    println!();

    let algos = [
        Algorithm::Ramanujan,
        Algorithm::Euler,
        Algorithm::Wallis,
        Algorithm::Leibniz,
    ];
    let medals = ["🥇", "🥈", "🥉", "  "];

    let mut ranked: Vec<(usize, usize, f64)> = algos
        .iter()
        .enumerate()
        .map(|(i, _)| (i, results[i][5].digits, results[i][5].elapsed_us()))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.partial_cmp(&b.2).unwrap()));

    for (rank, &(i, digits, us)) in ranked.iter().enumerate() {
        let a = algos[i];
        println!(
            "  {}  {}{}{} {:>18}{}  →  {}{:>2} correct digits{}  in {}{}{}",
            medals[rank],
            a.color(),
            BOLD,
            a.symbol(),
            RESET,
            a.label(),
            BOLD,
            digits,
            RESET,
            DIM,
            fmt_time(us),
            RESET
        );
    }

    println!();
    println!(
        "  {}{}◆ Ramanujan adds ~8 decimal digits of precision per iteration.{}",
        BOLD, RED, RESET
    );
    println!(
        "  {}  Leibniz needs 10^n terms for n digits — an exponential gap.{}",
        DIM, RESET
    );
    println!(
        "  {}  Ramanujan with k=10 already beats Leibniz at k=1,000,000.{}",
        DIM, RESET
    );
    println!();
    println!(
        "  {}{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}",
        BOLD, GREY, RESET
    );
    println!();
    println!(
        "  {}π showcase (Ramanujan k=10): {}{:.15}{}",
        DIM,
        GREEN,
        ramanujan(10),
        RESET
    );
    println!(
        "  {}π showcase (Leibniz 1M):    {}{:.15}{}",
        DIM,
        BLUE,
        leibniz(1_000_000),
        RESET
    );
    println!(
        "  {}π (true f64 constant):      {}{:.15}{}",
        DIM, CYAN, PI, RESET
    );
    println!();
}

fn main() {
    let iter_sets = [10u64, 100, 1_000, 10_000, 100_000, 1_000_000];
    let algos = [
        Algorithm::Ramanujan,
        Algorithm::Euler,
        Algorithm::Wallis,
        Algorithm::Leibniz,
    ];

    print_header();
    eprintln!(
        "  Computing {} algorithms × {} iteration counts...",
        algos.len(),
        iter_sets.len()
    );
    eprintln!();

    let results: Vec<Vec<BenchResult>> = algos
        .iter()
        .map(|&algo| {
            iter_sets
                .iter()
                .map(|&n| BenchResult::new(algo, n))
                .collect()
        })
        .collect();

    print_convergence(&results);
    print_visual_race(&results);
    print_verdict(&results);
}
