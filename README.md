# ternary-auction

**Auction mechanisms where bids live in {-1, 0, +1}.**

Most auction theory assumes continuous bids — price is a real number, you shade your bid strategically, equilibrium is a calculus problem. But what happens when you only have three choices? *Pass*, *Weak*, or *Strong*. No fine-grained signaling. No gradual escalation. Just commit or don't.

That constraint turns out to be *clarifying*. Truthful bidding becomes easier to verify. Dominant strategies are more robust. The mechanism design questions get sharper because there's nowhere to hide.

## What's Inside

- **`TernaryBid`** — the three-valued bid: `Pass`, `Weak`, `Strong`
- **`VickreyAuction`** — second-price sealed-bid, the workhorse. Truthful bidding is a dominant strategy, and we prove it with `verify_truthfulness()`
- **`FirstPriceAuction`** — you pay what you bid. Now strategy matters — the game theory gets interesting
- **`AllPayAuction`** — everyone pays, winner takes the prize. Models lobbying, R&D races, contest theory
- **`AuctionResult`** — winner, price, revenue, social welfare, allocation efficiency — all the metrics economists care about

## Quick Example

```rust
use ternary_auction::*;

// Three bidders, one item
let bids = vec![
    Bid::new(0, TernaryBid::Strong, 100.0),  // Strong bidder, values item at 100
    Bid::new(1, TernaryBid::Weak,   60.0),   // Weak bidder, values at 60
    Bid::new(2, TernaryBid::Pass,   30.0),   // Passes entirely
];

// Vickrey auction: winner pays second price
let result = VickreyAuction::run(&bids);
assert_eq!(result.winner, Some(0));  // Strong bidder wins
assert_eq!(result.price, 1.0);       // Pays the Weak bid value

// Verify nobody benefits from lying
assert!(VickreyAuction::verify_truthfulness(&bids));
```

## Why Ternary Bids?

**Simplicity is a feature.** In real-world settings — spectrum auctions, ad markets, procurement — bidders often face coarse choice sets. Ternary auctions model that coarseness directly, making the game-theoretic analysis tractable and the implementations verifiable.

**Use cases:**
- **Mechanism design research** — test auction properties with minimal moving parts
- **Game theory education** — three bids make dominant strategies visible
- **Multi-agent systems** — lightweight bidding for resource allocation where full continuous auctions are overkill
- **Market simulation** — model coarse-grained strategic decisions
- **Fairness analysis** — verify allocation efficiency and truthfulness programmatically

## Install

```bash
cargo add ternary-auction
```

## License

MIT
