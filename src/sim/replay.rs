use crate::data::GameData;
use crate::model::{
    Auction, AuctionStatus, BidderActor, CampaignStatus, OwnedProperty, Player, Property,
    ResearchLevel, WalkawayStyle,
};
use crate::sim::auction_sim::{
    award_discipline_reputation, begin_auction_calls, create_auction, quick_resolve_auction,
    stop_player_bidding,
};
use crate::sim::campaign::{apply_weekly_pressure, campaign_status};
use crate::sim::finance::finance_snapshot;
use crate::sim::rental::{
    leasing_cost, progress_leasing_campaigns, start_leasing_campaign, weekly_rent_for_owned,
};
use crate::sim::valuation::{cash_needed_to_settle, deposit, purchase_fees};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CampaignReplayReport {
    pub status: CampaignStatus,
    pub week: u32,
    pub homes: usize,
    pub weekly_rent: i64,
    pub net_worth: i64,
    pub resumed_auction_status: AuctionStatus,
}

pub fn run_authored_campaign_replay() -> Result<CampaignReplayReport, String> {
    let data = GameData::load();
    let mut player = Player::new();
    let mut week = 1;
    let market = &data.market_events[0];
    let mut registrations = 2;

    for property_id in [6, 8] {
        settle_authored_auction(&data, &mut player, property_id, market, &mut registrations)?;
    }
    advance_week(&mut player, market, &mut week);
    earn_authored_discipline(&data, &mut player, market)?;
    registrations = 2;
    settle_authored_auction(&data, &mut player, 5, market, &mut registrations)?;
    advance_week(&mut player, market, &mut week);

    let status = campaign_status(&player, market, week);
    if status != CampaignStatus::Won {
        return Err(format!(
            "authored replay stopped at week {week}: {status:?}, {} homes, {} rent",
            player.properties.len(),
            crate::sim::rental::portfolio_rental_snapshot(&player).gross_rent
        ));
    }

    let resumed_auction_status = replay_saved_live_auction(&data, market)?;
    Ok(CampaignReplayReport {
        status,
        week,
        homes: player.properties.len(),
        weekly_rent: crate::sim::rental::portfolio_rental_snapshot(&player).gross_rent,
        net_worth: crate::sim::valuation::net_worth(&player, market),
        resumed_auction_status,
    })
}

fn earn_authored_discipline(
    data: &GameData,
    player: &mut Player,
    market: &crate::model::MarketEvent,
) -> Result<(), String> {
    let property = Property::from_template(
        data.properties
            .first()
            .ok_or_else(|| "the catalogue has no discipline rehearsal property".to_string())?,
    );
    for _ in 0..3 {
        let mut auction = create_auction(
            &property,
            market,
            &data.bidder_profiles,
            property.reserve_price,
            ResearchLevel::AgentPack,
            WalkawayStyle::Balanced,
        );
        begin_auction_calls(&mut auction);
        auction.current_bid = property.reserve_price;
        auction.player_walkaway_price = property.reserve_price - 10_000;
        stop_player_bidding(&mut auction);
        auction.status = Some(AuctionStatus::SoldToNpc("the rehearsal rival".to_string()));
        if !award_discipline_reputation(player, &auction) {
            return Err("discipline rehearsal did not award reputation".to_string());
        }
    }
    Ok(())
}

fn settle_authored_auction(
    data: &GameData,
    player: &mut Player,
    property_id: usize,
    market: &crate::model::MarketEvent,
    registrations: &mut u8,
) -> Result<(), String> {
    if *registrations == 0 {
        return Err(format!(
            "no registration remained for property {property_id}"
        ));
    }
    let template = data
        .properties
        .iter()
        .find(|property| property.id == property_id)
        .ok_or_else(|| format!("property {property_id} is not authored"))?;
    let property = Property::from_template(template);
    let price = property.reserve_price - 10_000;
    let mut auction = create_auction(
        &property,
        market,
        &data.bidder_profiles,
        price,
        ResearchLevel::BuildingInspection,
        WalkawayStyle::Balanced,
    );
    begin_auction_calls(&mut auction);
    auction.current_bid = price;
    auction.last_bidder = Some(BidderActor::Player);
    auction.status = Some(AuctionStatus::SoldToPlayer);
    *registrations -= 1;

    let finance = finance_snapshot(player, market, price);
    if !finance.can_buy {
        return Err(format!("finance rejected authored property {property_id}"));
    }
    player.cash -= cash_needed_to_settle(price);
    let debt = price - deposit(price);
    player.debt += debt;
    let mut owned = OwnedProperty::new(
        property,
        price,
        purchase_fees(price),
        deposit(price),
        debt,
        price,
        ResearchLevel::BuildingInspection,
        WalkawayStyle::Balanced,
    );
    let rent = weekly_rent_for_owned(&owned, market);
    if !start_leasing_campaign(&mut owned, rent) {
        return Err(format!("property {property_id} could not enter leasing"));
    }
    player.cash -= leasing_cost(rent);
    player.properties.push(owned);
    Ok(())
}

fn advance_week(player: &mut Player, market: &crate::model::MarketEvent, week: &mut u32) {
    apply_weekly_pressure(player, market);
    progress_leasing_campaigns(player);
    *week += 1;
}

fn replay_saved_live_auction(
    data: &GameData,
    market: &crate::model::MarketEvent,
) -> Result<AuctionStatus, String> {
    let property = Property::from_template(
        data.properties
            .first()
            .ok_or_else(|| "the catalogue has no property".to_string())?,
    );
    let mut original = create_auction(
        &property,
        market,
        &data.bidder_profiles,
        property.reserve_price,
        ResearchLevel::StreetScan,
        WalkawayStyle::Balanced,
    );
    begin_auction_calls(&mut original);
    original.is_player_active = false;
    let encoded = serde_json::to_string(&original).map_err(|error| error.to_string())?;
    let mut resumed: Auction = serde_json::from_str(&encoded).map_err(|error| error.to_string())?;
    quick_resolve_auction(&mut original);
    quick_resolve_auction(&mut resumed);

    if original.status != resumed.status
        || original.current_bid != resumed.current_bid
        || original.rng_state != resumed.rng_state
    {
        return Err("saved live auction changed its deterministic outcome".to_string());
    }
    original
        .status
        .ok_or_else(|| "saved live auction did not reach a result".to_string())
}
