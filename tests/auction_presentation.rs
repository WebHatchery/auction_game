use auction_game::ui::auction_view::{BidPressure, CallPhase};

#[test]
fn calls_escalate_without_requiring_a_leading_bidder() {
    let cases = [
        (40.0, CallPhase::Opening),
        (18.0, CallPhase::Slowing),
        (10.0, CallPhase::FinalCalls),
        (6.0, CallPhase::Once),
        (3.0, CallPhase::Twice),
        (0.0, CallPhase::Twice),
    ];
    for (seconds, expected) in cases {
        assert_eq!(CallPhase::for_room(seconds, false), expected);
    }
    assert_eq!(CallPhase::for_room(40.0, true), CallPhase::Bidding);
}

#[test]
fn a_late_bid_reopens_the_call_and_hides_the_urgent_clock() {
    assert!(CallPhase::for_room(2.0, true).urgent());
    assert!(!CallPhase::for_room(11.0, true).urgent());
    assert!(!CallPhase::for_room(7.0, true).urgent());
}

#[test]
fn risk_scale_preserves_price_order_even_when_the_estimate_is_beyond_the_limit() {
    for (estimate, limit, bid) in [
        (600_000, 550_000, 500_000),
        (500_000, 600_000, 610_000),
        (550_000, 550_000, 550_000),
    ] {
        let scale = BidPressure::new(450_000, estimate, limit, bid);
        let mut prices = [estimate, limit, bid];
        prices.sort();
        let positions = prices.map(|price| scale.position(price));
        assert!(positions[0] <= positions[1] && positions[1] <= positions[2]);
        assert!(positions
            .into_iter()
            .all(|position| (0.0..=1.0).contains(&position)));
    }
}

#[test]
fn risk_scale_survives_degenerate_and_extreme_saved_values() {
    for amount in [0, 1, -1, i64::MAX] {
        let scale = BidPressure::new(amount, amount, amount, amount);
        assert!(scale.position(amount).is_finite());
        assert_eq!(scale.position(-1), 0.0);
        assert_eq!(scale.position(i64::MAX), 1.0);
    }
}

#[test]
fn authored_calls_cover_every_phase_and_resolve_price_placeholders() {
    for seconds in [40.0, 25.0, 18.0, 10.0, 6.0, 3.0] {
        let phase = CallPhase::for_room(seconds, seconds < 40.0);
        assert!(!phase.label().is_empty());
        let call = phase.call(seconds, 450_000, 460_000);
        assert!(!call.is_empty() && !call.contains('{'));
    }
}
