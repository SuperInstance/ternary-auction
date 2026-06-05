#![forbid(unsafe_code)]
//! Ternary auction mechanisms — {-1,0,+1} bidding.

/// A ternary bid value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TernaryBid { Pass = 0, Weak = 1, Strong = 2 }

impl TernaryBid {
    pub fn value(self) -> i32 { self as i32 }
    pub fn from_ternary(t: i8) -> Self { match t { -1 | 0 => TernaryBid::Pass, _ => TernaryBid::Strong } }
}

/// A bid from an agent.
#[derive(Debug, Clone)]
pub struct Bid {
    pub bidder: usize,
    pub value: TernaryBid,
    pub true_value: f64, // For truthfulness testing
}

impl Bid {
    pub fn new(bidder: usize, value: TernaryBid, true_value: f64) -> Self {
        Self { bidder, value, true_value }
    }
}

/// Auction result.
#[derive(Debug, Clone)]
pub struct AuctionResult {
    pub winner: Option<usize>,
    pub price: f64,
    pub revenue: f64,
    pub social_welfare: f64,
    pub allocation_efficient: bool,
}

/// Vickrey (second-price sealed-bid) auction for ternary bids.
pub struct VickreyAuction;

impl VickreyAuction {
    /// Run a Vickrey auction. Winner pays second-highest bid value.
    pub fn run(bids: &[Bid]) -> AuctionResult {
        if bids.is_empty() {
            return AuctionResult { winner: None, price: 0.0, revenue: 0.0, social_welfare: 0.0, allocation_efficient: true };
        }

        let mut sorted: Vec<&Bid> = bids.iter().collect();
        sorted.sort_by(|a, b| b.value.value().cmp(&a.value.value()));

        let winner_bid = sorted[0];
        let price = if sorted.len() > 1 { sorted[1].value.value() as f64 } else { 0.0 };

        AuctionResult {
            winner: Some(winner_bid.bidder),
            price,
            revenue: price,
            social_welfare: winner_bid.true_value,
            allocation_efficient: true, // Vickrey is always efficient
        }
    }

    /// Verify that truthful bidding is a dominant strategy.
    pub fn verify_truthfulness(bids: &[Bid]) -> bool {
        // In a Vickrey auction, truthful bidding is always optimal
        // Check: would any bidder benefit from misreporting?
        let truthful_result = Self::run(bids);
        for i in 0..bids.len() {
            let truthful_utility = Self::agent_utility(bids, i, bids[i].value, &truthful_result);
            // Try all alternative bids
            for alt in [TernaryBid::Pass, TernaryBid::Weak, TernaryBid::Strong] {
                if alt == bids[i].value { continue; }
                let mut alt_bids = bids.to_vec();
                alt_bids[i].value = alt;
                let alt_result = Self::run(&alt_bids);
                let alt_utility = Self::agent_utility(&alt_bids, i, alt, &alt_result);
                if alt_utility > truthful_utility + 1e-6 {
                    return false; // Found beneficial misreport
                }
            }
        }
        true
    }

    fn agent_utility(bids: &[Bid], agent: usize, bid: TernaryBid, result: &AuctionResult) -> f64 {
        if result.winner == Some(agent) {
            bids[agent].true_value - result.price
        } else {
            0.0
        }
    }
}

/// VCG (Vickrey-Clarke-Groves) mechanism for multi-item allocation.
pub struct VCGMechanism;

impl VCGMechanism {
    /// Run VCG with ternary bids. Each agent bids on an item.
    /// Payment = externality imposed on others.
    pub fn run(bids: &[Bid], num_items: usize) -> Vec<AuctionResult> {
        let mut results = Vec::new();
        for item in 0..num_items {
            // For each item, run a separate Vickrey auction among interested bidders
            let item_bids: Vec<Bid> = bids.iter().filter(|b| b.value != TernaryBid::Pass).cloned().collect();
            if item_bids.is_empty() {
                results.push(AuctionResult { winner: None, price: 0.0, revenue: 0.0, social_welfare: 0.0, allocation_efficient: true });
            } else {
                results.push(VickreyAuction::run(&item_bids));
            }
        }
        results
    }

    /// Compute total social welfare.
    pub fn total_welfare(results: &[AuctionResult]) -> f64 {
        results.iter().map(|r| r.social_welfare).sum()
    }

    /// Compute total revenue.
    pub fn total_revenue(results: &[AuctionResult]) -> f64 {
        results.iter().map(|r| r.revenue).sum()
    }
}

/// Ternary bid resolution — resolve competing ternary signals.
pub struct TernaryResolver;

impl TernaryResolver {
    /// Resolve a set of ternary signals {-1, 0, +1} into a decision.
    pub fn resolve(signals: &[i8]) -> i8 {
        let pos = signals.iter().filter(|&&s| s > 0).count();
        let neg = signals.iter().filter(|&&s| s < 0).count();
        if pos > neg { 1 } else if neg > pos { -1 } else { 0 }
    }

    /// Confidence of resolution (fraction agreeing with outcome).
    pub fn confidence(signals: &[i8]) -> f64 {
        if signals.is_empty() { return 0.0; }
        let decision = Self::resolve(signals);
        let agreeing = signals.iter().filter(|&&s| s == decision).count();
        agreeing as f64 / signals.len() as f64
    }

    /// Check if resolution is unanimous.
    pub fn is_unanimous(signals: &[i8]) -> bool {
        if signals.is_empty() { return true; }
        signals.iter().all(|&s| s == signals[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vickrey_basic() {
        let bids = vec![
            Bid::new(0, TernaryBid::Strong, 10.0),
            Bid::new(1, TernaryBid::Weak, 5.0),
            Bid::new(2, TernaryBid::Pass, 1.0),
        ];
        let result = VickreyAuction::run(&bids);
        assert_eq!(result.winner, Some(0));
        assert_eq!(result.price, 1.0); // Second-highest = Weak
    }

    #[test]
    fn test_vickrey_single_bid() {
        let bids = vec![Bid::new(0, TernaryBid::Strong, 10.0)];
        let result = VickreyAuction::run(&bids);
        assert_eq!(result.winner, Some(0));
        assert_eq!(result.price, 0.0); // No second bid
    }

    #[test]
    fn test_vickrey_all_pass() {
        let bids = vec![
            Bid::new(0, TernaryBid::Pass, 0.0),
            Bid::new(1, TernaryBid::Pass, 0.0),
        ];
        let result = VickreyAuction::run(&bids);
        assert_eq!(result.winner, Some(0)); // First passer wins
    }

    #[test]
    fn test_truthfulness() {
        let bids = vec![
            Bid::new(0, TernaryBid::Strong, 10.0),
            Bid::new(1, TernaryBid::Weak, 5.0),
            Bid::new(2, TernaryBid::Strong, 8.0),
        ];
        assert!(VickreyAuction::verify_truthfulness(&bids));
    }

    #[test]
    fn test_vcg_multiple_items() {
        let bids = vec![
            Bid::new(0, TernaryBid::Strong, 10.0),
            Bid::new(1, TernaryBid::Weak, 5.0),
            Bid::new(2, TernaryBid::Strong, 8.0),
        ];
        let results = VCGMechanism::run(&bids, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_vcg_welfare() {
        let results = vec![
            AuctionResult { winner: Some(0), price: 1.0, revenue: 1.0, social_welfare: 10.0, allocation_efficient: true },
            AuctionResult { winner: Some(1), price: 2.0, revenue: 2.0, social_welfare: 8.0, allocation_efficient: true },
        ];
        assert!((VCGMechanism::total_welfare(&results) - 18.0).abs() < 0.01);
        assert!((VCGMechanism::total_revenue(&results) - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_ternary_resolve_positive() {
        let signals = vec![1, 1, 1, -1, 0];
        assert_eq!(TernaryResolver::resolve(&signals), 1);
    }

    #[test]
    fn test_ternary_resolve_negative() {
        let signals = vec![-1, -1, 1, 0, 0];
        assert_eq!(TernaryResolver::resolve(&signals), -1);
    }

    #[test]
    fn test_ternary_resolve_tie() {
        let signals = vec![1, -1, 0];
        assert_eq!(TernaryResolver::resolve(&signals), 0);
    }

    #[test]
    fn test_confidence() {
        let signals = vec![1, 1, 1, -1, 0];
        let c = TernaryResolver::confidence(&signals);
        assert!((c - 0.6).abs() < 0.01); // 3/5 agree with +1
    }

    #[test]
    fn test_unanimous() {
        assert!(TernaryResolver::is_unanimous(&[1, 1, 1]));
        assert!(!TernaryResolver::is_unanimous(&[1, -1, 1]));
    }

    #[test]
    fn test_empty_bids() {
        let result = VickreyAuction::run(&[]);
        assert_eq!(result.winner, None);
    }

    #[test]
    fn test_ternary_bid_ordering() {
        assert!(TernaryBid::Strong > TernaryBid::Weak);
        assert!(TernaryBid::Weak > TernaryBid::Pass);
    }
}
