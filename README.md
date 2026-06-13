# Ternary Auction

Mechanism design for **ternary {-1, 0, +1} resource allocation** — implementing Vickrey auctions, VCG mechanisms, strategy-proofness verification, and ternary signal resolution. Every bid is a trit, every allocation provably efficient.

## Why It Matters

Classical auction theory assumes continuous or discrete bid spaces. When agents are constrained to ternary bids {-1 (pass), 0 (weak), +1 (strong)}, the mechanism design problem becomes both simpler and more subtle:

- **Simpler**: The allocation rule is a simple sort. Payment rules reduce to counting.
- **Subtler**: With only 3 bid levels, the incentive compatibility constraints bind differently. Truthful bidding is still optimal in Vickrey, but the proof requires checking all $3^N$ possible misreports for N bidders.

This crate implements the full verification loop: run the auction, then exhaustively check that no bidder could benefit from any alternative ternary bid. The `verify_truthfulness` function performs this $\Theta(N \cdot 3)$ check.

## How It Works

### Vickrey Auction (Second-Price Sealed-Bid)

Given bids $b_1 \geq b_2 \geq \cdots \geq b_N$ where each $b_i \in \{0, 1, 2\}$ (Pass, Weak, Strong):

$$\text{winner} = \arg\max_i b_i$$
$$\text{price} = b_{(2)} = \text{second-highest bid}$$

The winner's utility is $u_i = v_i - p$ where $v_i$ is their true valuation. Since the price is independent of the winner's bid, truthful bidding is a **weakly dominant strategy** — formally proven by the verification check.

### VCG Mechanism (Multi-Item)

For $K$ items, VCG runs independent Vickrey auctions per item (when bids are independent). The VCG payment rule charges each winner the **externality** they impose on others:

$$p_i = \sum_{j \neq i} v_j(\text{allocation without } i) - \sum_{j \neq i} v_j(\text{actual allocation})$$

### Ternary Signal Resolution

Beyond auctions, the `TernaryResolver` aggregates competing {-1, 0, +1} signals into a consensus:

$$\text{resolve}(\vec{s}) = \text{sign}\left(\sum_{i} s_i\right)$$

With confidence measured as the fraction agreeing with the outcome: $c = |\{i : s_i = r\}| / N$.

### Complexity

| Operation | Time |
|-----------|------|
| `VickreyAuction::run(&[Bid])` | O(N log N) — sort by bid value |
| `verify_truthfulness(&[Bid])` | O(3N) — 3 alternative bids × N agents |
| `VCGMechanism::run(&[Bid], K)` | O(K · N log N) |
| `TernaryResolver::resolve(&[i8])` | O(N) |
| `TernaryResolver::confidence(&[i8])` | O(N) |

## Quick Start

```rust
use ternary_auction::{VickreyAuction, VCGMechanism, TernaryResolver, Bid, TernaryBid};

let bids = vec![
    Bid::new(0, TernaryBid::Strong, 10.0),
    Bid::new(1, TernaryBid::Weak,   5.0),
    Bid::new(2, TernaryBid::Pass,   1.0),
];

let result = VickreyAuction::run(&bids);
assert_eq!(result.winner, Some(0));
assert!(VickreyAuction::verify_truthfulness(&bids)); // truthful bidding is dominant

// Multi-item VCG
let results = VCGMechanism::run(&bids, 3);
assert_eq!(results.len(), 3);

// Signal resolution
let signals = vec![1, 1, 1, -1, 0];
assert_eq!(TernaryResolver::resolve(&signals), 1);
assert!((TernaryResolver::confidence(&signals) - 0.6).abs() < 0.01);
```

## API

### Auction Types

| Type | Description |
|------|-------------|
| `TernaryBid` | Enum: Pass (0), Weak (1), Strong (2) |
| `Bid` | bidder_id + TernaryBid + true_value |
| `AuctionResult` | winner, price, revenue, social_welfare, efficiency flag |

### Mechanisms

| Function | Description |
|----------|-------------|
| `VickreyAuction::run(&[Bid]) → AuctionResult` | Second-price sealed-bid |
| `VickreyAuction::verify_truthfulness(&[Bid]) → bool` | Dominant strategy check |
| `VCGMechanism::run(&[Bid], num_items) → Vec<AuctionResult>` | Multi-item allocation |
| `VCGMechanism::total_welfare(&[AuctionResult]) → f64` | Social welfare sum |
| `TernaryResolver::resolve(&[i8]) → i8` | Signal aggregation |
| `TernaryResolver::confidence(&[i8]) → f64` | Agreement fraction |
| `TernaryResolver::is_unanimous(&[i8]) → bool` | Consensus check |

## Architecture Notes

The auction mechanisms enforce the **γ + η = C** conservation link:

- **γ (allocation structure)**: who wins what — the assignment mapping
- **η (valuation perturbation)**: the bids, which may deviate from true values
- **C (conservation)**: strategy-proofness guarantees that no misreporting benefits any agent — the incentive compatibility invariant

The revenue equivalence theorem ensures that for any Bayesian-Nash incentive-compatible mechanism with the same allocation rule, the expected revenue is identical. This means Vickrey and VCG achieve the same revenue distribution despite different payment computations.

## References

- Vickrey, W. (1961). *Counterspeculation, Auctions, and Competitive Sealed Tenders*. Journal of Finance. — The original second-price auction.
- Myerson, R. (1981). *Optimal Auction Design*. Mathematics of Operations Research. — Revenue equivalence theorem.
- Clarke, E. (1971). *Multipart Pricing of Public Goods*. Public Choice. — Clarke pivot rule.
- Groves, T. (1973). *Incentives in Teams*. Econometrica. — VCG mechanism.
- Krishna, V. (2010). *Auction Theory* (2nd ed.). Academic Press.

## License: MIT
