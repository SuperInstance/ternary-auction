# Ternary Auction

**Ternary Auction** implements mechanism design for ternary {-1, 0, +1} resource allocation — providing Vickrey (second-price) auctions, VCG mechanisms, and strategy-proof verification for systems where agents bid with ternary-valued signals.

## Why It Matters

Resource allocation in multi-agent systems is fundamentally an auction problem: who gets the scarce resource? Mechanism design theory provides auctions where truthful bidding is the dominant strategy — agents can't benefit from lying about their valuations. The ternary bid space {Pass (-1/0), Weak (+1 weak), Strong (+1 strong)} creates a coarse but efficient allocation mechanism suitable for fleet task assignment, where fine-grained bidding is unnecessary and ternary signaling matches the conservation framework.

## How It Works

### Vickrey (Second-Price Sealed-Bid) Auction

```
Winner = bidder with highest bid value
Price paid = second-highest bid value
```

Implementation: sort bids descending, winner = bids[0], price = bids[1].

- Sorting: **O(N log N)** for N bidders
- Truthfulness: guaranteed by the revelation principle — truthful bidding is a dominant strategy

### Ternary Bid Mapping

Ternary values map to bid levels:

```
-1 → Pass (no bid)
 0 → Pass (no bid)
+1 → Strong bid
```

For finer granularity, `TernaryBid` provides three levels:

```
Pass = 0, Weak = 1, Strong = 2
```

### Truthfulness Verification

The crate verifies the incentive-compatibility property:

```
for each agent i:
    truthful_utility = utility(i, bid_i, result)
    for each alternative bid b:
        alt_utility = utility(i, b, result_with_b)
        if alt_utility > truthful_utility + ε:
            return false  // Lying is profitable — not truthful
```

Verification: **O(N · B)** where B = number of alternative bid values.

### Social Welfare

```
social_welfare = Σ agent_utilities
```

In Vickrey auctions, the allocation is efficient: the winner is the agent with the highest true valuation, maximizing social welfare.

### VCG (Vickrey-Clarke-Groves)

VCG extends to combinatorial allocation:

```
payment_i = Σ_{j≠i} v_j(allocation_without_i) - Σ_{j≠i} v_j(allocation_with_i)
```

VCG is strategy-proof for combinatorial auctions where Vickrey handles single-item.

## Quick Start

```rust
use ternary_auction::{Bid, TernaryBid, VickreyAuction};

let bids = vec![
    Bid::new(0, TernaryBid::Strong, 100.0),
    Bid::new(1, TernaryBid::Weak, 50.0),
    Bid::new(2, TernaryBid::Pass, 0.0),
];

let result = VickreyAuction::run(&bids);
assert_eq!(result.winner, Some(0)); // Highest bid wins
assert_eq!(result.price, 1.0);      // Pays second-highest (Weak=1)

// Verify truthfulness
assert!(VickreyAuction::verify_truthfulness(&bids));
```

## API

| Type | Description |
|------|-------------|
| `TernaryBid` | `Pass (0)`, `Weak (1)`, `Strong (2)` |
| `Bid` | bidder_id, value, true_value |
| `AuctionResult` | winner, price, revenue, social_welfare, allocation_efficient |
| `VickreyAuction` | Second-price sealed-bid auction |
| `VickreyAuction::run(bids)` | Execute auction |
| `VickreyAuction::verify_truthfulness(bids)` | Check incentive compatibility |

## Architecture Notes

Ternary Auction provides resource allocation for fleet task assignment in SuperInstance. In γ + η = C, winning an auction represents γ (growth — acquiring resources for expansion) while losing represents η (avoidance — deferring to higher-value agents). The truthfulness property ensures γ + η = C holds: no agent can game the mechanism to violate the conservation of social welfare.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for fleet resource allocation.

## References

1. Vickrey, W. (1961). "Counterspeculation, Auctions, and Competitive Sealed Tenders." *Journal of Finance*, 16(1), 8–37.
2. Clarke, E. H. (1971). "Multipart Pricing of Public Goods." *Public Choice*, 11, 17–33.
3. Nisan, N. et al. (2007). *Algorithmic Game Theory*. Cambridge University Press.

## License

MIT
