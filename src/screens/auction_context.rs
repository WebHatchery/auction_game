//! A lot catalogue at the player's elbow and a single spatial risk instrument.
use crate::app::App;
use crate::model::Auction;
use crate::screens::auction::AuctionUiAction;
use crate::sim::finance::{max_financeable_bid, rental_underwrite, FinanceSnapshot};
use crate::sim::research::researched_value_range;
use crate::sim::valuation::projected_purchase_margin;
use crate::ui::auction_view::BidPressure;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_context(
    app: &App,
    auction: &Auction,
    finance: FinanceSnapshot,
) -> Option<AuctionUiAction> {
    if let Some(index) = app
        .auction_focus
        .filter(|index| *index < auction.bidders.len())
    {
        return draw_rival_notes(app, auction, index);
    }
    label(
        &format!("LOT {:02}", auction.property.id),
        24.0,
        99.0,
        26,
        ACCENT,
    );
    label("YOUR CATALOGUE", 24.0, 125.0, 14, TEXT_DIM);
    draw_house_art(Rect::new(24.0, 148.0, 174.0, 124.0), &auction.property);
    draw_wrapped_text(
        &auction.property.address,
        24.0,
        304.0,
        176.0,
        24,
        TEXT_BRIGHT,
    );
    label(&auction.property.suburb, 24.0, 365.0, 17, TEXT_DIM);
    if app.auction_notes_open {
        draw_notes(app, auction, finance);
    } else {
        let headroom =
            (max_financeable_bid(&app.player, app.market()) - auction.current_bid).max(0);
        label("FINANCE HEADROOM", 24.0, 414.0, 14, TEXT_DIM);
        label(
            &format_money(headroom),
            24.0,
            445.0,
            26,
            if finance.can_buy { TEXT } else { WARNING },
        );
        let margin =
            projected_purchase_margin(&auction.property, auction.current_bid, app.market());
        draw_wrapped_text(
            &format!("At this price: {} estimated margin.", format_money(margin)),
            24.0,
            484.0,
            177.0,
            17,
            if margin < 0 { WARNING } else { TEXT_DIM },
        );
    }
    if button(
        Rect::new(16.0, 557.0, 190.0, 44.0),
        if app.auction_notes_open {
            "CLOSE NOTES"
        } else {
            "PROPERTY NOTES"
        },
        true,
        ButtonTone::Ghost,
    ) {
        return Some(AuctionUiAction::ToggleNotes);
    }
    if !auction.is_running() {
        return None;
    }
    let near_limit = auction.next_bid() >= auction.player_walkaway_price;
    if auction.is_running()
        && button(
            Rect::new(92.0, 12.0, 128.0, 44.0),
            "WALK AWAY",
            auction.is_player_active,
            if near_limit {
                ButtonTone::Danger
            } else {
                ButtonTone::Ghost
            },
        )
    {
        return Some(AuctionUiAction::WalkAway);
    }
    None
}

fn draw_notes(app: &App, auction: &Auction, finance: FinanceSnapshot) {
    let rental = rental_underwrite(&auction.property, app.market(), auction.next_bid());
    for (index, (title, amount)) in [
        ("Cash after next bid", finance.cash_after_settle),
        ("Bank room after bid", finance.headroom_after),
        ("Rental cashflow / wk", rental.net_cashflow),
    ]
    .iter()
    .enumerate()
    {
        let y = 400.0 + index as f32 * 48.0;
        label(title, 24.0, y, 14, TEXT_DIM);
        label(&format_money(*amount), 24.0, y + 25.0, 20, TEXT);
    }
}

pub(super) fn draw_pressure(app: &App, auction: &Auction) {
    let (low, high) = researched_value_range(
        &auction.property,
        app.market(),
        auction.player_research_level,
        app.player.reputation,
    );
    let estimate = low + (high - low) / 2;
    let scale = BidPressure::new(
        auction.property.guide_price,
        estimate,
        auction.player_walkaway_price,
        auction.current_bid,
    );
    let x = |price| 252.0 + scale.position(price) * 638.0;
    let limit_x = x(auction.player_walkaway_price);
    let bid_x = x(auction.current_bid);
    let over = auction.current_bid >= auction.player_walkaway_price;
    draw_rectangle(252.0, 479.0, 638.0, 4.0, PANEL_EDGE);
    draw_rectangle(
        limit_x,
        478.0,
        890.0 - limit_x,
        6.0,
        Color::from_rgba(87, 49, 40, 255),
    );
    draw_rectangle(
        x(low),
        475.0,
        (x(high) - x(low)).max(2.0),
        12.0,
        Color::from_rgba(56, 81, 73, 255),
    );
    draw_rectangle(
        252.0,
        479.0,
        bid_x - 252.0,
        4.0,
        if over { NEGATIVE } else { ACCENT },
    );
    draw_line(limit_x, 469.0, limit_x, 490.0, 2.0, WARNING);
    draw_triangle(
        vec2(bid_x, 482.0),
        vec2(bid_x - 5.0, 490.0),
        vec2(bid_x + 5.0, 490.0),
        TEXT_BRIGHT,
    );
    label(
        &format!("WALK-AWAY {}", format_money(auction.player_walkaway_price)),
        (limit_x - 120.0).clamp(252.0, 660.0),
        468.0,
        14,
        if over { NEGATIVE } else { WARNING },
    );
    label(
        &format!("RESEARCH ESTIMATE {}", format_money(estimate)),
        252.0,
        511.0,
        14,
        TEXT_DIM,
    );
    label(
        if over {
            if auction.current_bid > auction.player_walkaway_price {
                "PAST YOUR LIMIT"
            } else {
                "AT YOUR LIMIT"
            }
        } else if auction.next_bid() >= auction.player_walkaway_price {
            "LAST STEP TO YOUR LIMIT"
        } else {
            "BID PRESSURE"
        },
        657.0,
        511.0,
        14,
        if over { NEGATIVE } else { TEXT_DIM },
    );
}

fn draw_rival_notes(app: &App, auction: &Auction, index: usize) -> Option<AuctionUiAction> {
    let bidder = &auction.bidders[index];
    label("STUDYING", 24.0, 99.0, 20, crate::ui::BLUE);
    crate::screens::auction_scene::actor(
        Rect::new(54.0, 130.0, 104.0, 122.0),
        bidder.bidder_type,
        bidder.mood,
        auction.last_bidder == Some(crate::model::BidderActor::Npc(index)),
    );
    draw_wrapped_text(&bidder.name, 24.0, 292.0, 178.0, 24, TEXT_BRIGHT);
    label(bidder.bidder_type.label(), 24.0, 356.0, 17, TEXT_DIM);
    draw_wrapped_text(&bidder.tell, 24.0, 395.0, 178.0, 20, TEXT);
    let history = app
        .player
        .rival_notebook
        .iter()
        .find(|record| record.name == bidder.name);
    let note = history
        .map(|record| {
            format!(
                "Seen in {} rooms. Won {}.",
                record.auctions_met, record.auctions_won
            )
        })
        .unwrap_or_else(|| "First meeting. Watch how they answer your next bid.".to_string());
    draw_wrapped_text(&note, 24.0, 481.0, 178.0, 17, TEXT_DIM);
    if button(
        Rect::new(16.0, 557.0, 190.0, 44.0),
        "BACK TO LOT",
        true,
        ButtonTone::Ghost,
    ) {
        return Some(AuctionUiAction::FocusRival(index));
    }
    if auction.is_running()
        && button(
            Rect::new(92.0, 12.0, 128.0, 44.0),
            "WALK AWAY",
            auction.is_player_active,
            if auction.next_bid() >= auction.player_walkaway_price {
                ButtonTone::Danger
            } else {
                ButtonTone::Ghost
            },
        )
    {
        return Some(AuctionUiAction::WalkAway);
    }
    None
}
