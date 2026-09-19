use auction_game::data::GameData;
use auction_game::model::{
    Auction, BidderActor, BidderMood, BidderType, Property, ResearchLevel, WalkawayStyle,
};
use auction_game::sim::auction_sim::{
    begin_auction_calls, create_auction, place_player_bid, update_auction,
};
use auction_game::ui::auction_reactions::{behavioral_event, live_call, Reaction};

fn room() -> Auction {
    let data = GameData::load();
    let mut auction = create_auction(
        &Property::from_template(&data.properties[0]),
        &data.market_events[0],
        &data.bidder_profiles,
        600_000,
        ResearchLevel::AgentPack,
        WalkawayStyle::Balanced,
    );
    begin_auction_calls(&mut auction);
    auction.vendor_bid_used = true;
    for bidder in &mut auction.bidders {
        bidder.active = false;
    }
    auction.bidders[0].active = true;
    auction.bidders[0].max_price = 1_000_000;
    auction
}

#[test]
fn opponents_signal_a_bid_before_raising_and_the_flash_expires() {
    let mut auction = room();
    let opening = auction.current_bid;
    for _ in 0..150 {
        update_auction(&mut auction, 0.1);
        if auction.bidders[0].preparing_bid {
            break;
        }
    }
    assert!(auction.bidders[0].preparing_bid);
    assert_eq!(auction.current_bid, opening);
    assert_eq!(
        Reaction::for_bidder(
            &auction.bidders[0],
            auction.next_bid(),
            auction.bid_increment
        ),
        Reaction::Preparing
    );
    for _ in 0..12 {
        update_auction(&mut auction, 0.1);
    }
    assert!(auction.current_bid > opening);
    assert!(!auction.bidders[0].preparing_bid);
    assert!(auction.bidders[0].bid_flash > 0.0);
    for _ in 0..15 {
        update_auction(&mut auction, 0.1);
    }
    assert_eq!(auction.bidders[0].bid_flash, 0.0);
    assert_eq!(auction.bidders[0].mood, BidderMood::Watching);
}

#[test]
fn a_prepared_bid_is_rechecked_when_the_player_changes_the_price() {
    let mut auction = room();
    auction.bidders[0].bidder_type = BidderType::Investor;
    auction.bidders[0].max_price = auction.next_bid();
    auction.bidders[0].preparing_bid = true;
    auction.bidders[0].reaction_timer = 0.05;
    let before = auction.bidders.clone();
    place_player_bid(&mut auction);
    let player_price = auction.current_bid;
    update_auction(&mut auction, 0.1);
    assert!(!auction.bidders[0].active);
    assert!(!auction.bidders[0].preparing_bid);
    assert_eq!(auction.current_bid, player_price);
    let message = behavioral_event(&before, &auction).unwrap();
    assert!(message.contains("lowers their paddle") && !message.contains('$'));
}

#[test]
fn reactive_calls_follow_bids_silence_reserve_and_reopened_final_calls() {
    let mut auction = room();
    place_player_bid(&mut auction);
    assert!(live_call(&auction).contains("Looking for"));
    auction.seconds_since_bid = 3.0;
    assert!(live_call(&auction).contains("Any advance"));
    auction.on_market_announced = true;
    auction.seconds_since_bid = 6.0;
    assert_eq!(live_call(&auction), "I'm selling...");
    auction.seconds_remaining = 5.0;
    assert_eq!(live_call(&auction), "Going once.");
    auction.seconds_remaining = 2.0;
    assert_eq!(live_call(&auction), "Going twice.");
    place_player_bid(&mut auction);
    assert!(live_call(&auction).contains("Looking for"));
    assert_eq!(auction.last_bidder, Some(BidderActor::Player));
}

#[test]
fn readable_states_distinguish_deferred_decisions_pressure_and_withdrawal() {
    let mut auction = room();
    let bidder = &mut auction.bidders[0];
    bidder.mood = BidderMood::Hesitating;
    assert_eq!(
        Reaction::for_bidder(bidder, 500_000, 10_000),
        Reaction::Hesitant
    );
    assert_eq!(
        Reaction::for_bidder(bidder, 990_000, 10_000),
        Reaction::NearLimit
    );
    bidder.mood = BidderMood::Stretching;
    assert_eq!(
        Reaction::for_bidder(bidder, 500_000, 10_000),
        Reaction::Frustrated
    );
    bidder.bid_flash = 1.0;
    assert_eq!(
        Reaction::for_bidder(bidder, 500_000, 10_000),
        Reaction::Aggressive
    );
    bidder.active = false;
    assert_eq!(
        Reaction::for_bidder(bidder, 500_000, 10_000),
        Reaction::Withdrawn
    );
}

#[test]
fn saves_preserve_pending_decisions_and_older_rooms_gain_safe_defaults() {
    let mut auction = room();
    auction.bidders[0].preparing_bid = true;
    auction.bidders[0].reaction_timer = 0.5;
    let encoded = serde_json::to_string(&auction).unwrap();
    let mut resumed: Auction = serde_json::from_str(&encoded).unwrap();
    for _ in 0..30 {
        update_auction(&mut auction, 0.1);
        update_auction(&mut resumed, 0.1);
    }
    assert_eq!(
        serde_json::to_value(&auction).unwrap(),
        serde_json::to_value(&resumed).unwrap()
    );
    let mut old = serde_json::to_value(&auction).unwrap();
    old.as_object_mut().unwrap().remove("seconds_since_bid");
    for bidder in old["bidders"].as_array_mut().unwrap() {
        bidder.as_object_mut().unwrap().remove("preparing_bid");
        bidder.as_object_mut().unwrap().remove("bid_flash");
    }
    let migrated: Auction = serde_json::from_value(old).unwrap();
    assert_eq!(migrated.seconds_since_bid, 0.0);
    assert!(migrated
        .bidders
        .iter()
        .all(|bidder| !bidder.preparing_bid && bidder.bid_flash == 0.0));
}
