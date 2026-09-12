use auction_game::model::{AuctionStatus, CampaignStatus};
use auction_game::sim::replay::run_authored_campaign_replay;

#[test]
fn authored_campaign_replay_covers_settlement_week_advance_and_win() {
    let report = run_authored_campaign_replay().expect("authored route should remain playable");

    assert_eq!(report.status, CampaignStatus::Won);
    assert_eq!(report.homes, 3);
    assert!(report.week > 1);
    assert!(report.weekly_rent >= 1_500);
    assert!(report.net_worth >= 240_000);
}

#[test]
fn saved_live_auction_keeps_its_hammer_result() {
    let report = run_authored_campaign_replay().expect("replay should complete");

    assert!(matches!(
        report.resumed_auction_status,
        AuctionStatus::SoldToNpc(_) | AuctionStatus::PassedIn
    ));
}
