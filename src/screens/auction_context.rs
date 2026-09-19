//! A lot catalogue at the player's elbow and a single spatial risk instrument.
use crate::app::App;
use crate::model::Auction;
use crate::screens::auction::AuctionUiAction;
use crate::sim::research::researched_value_range;
use crate::sim::valuation::projected_purchase_margin;
use crate::ui::auction_view::BidPressure;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_context(app: &App, auction: &Auction) -> Option<AuctionUiAction> {
    if let Some(index) = app
        .auction_focus
        .filter(|index| *index < auction.bidders.len())
    {
        return draw_rival_notes(auction, index);
    }
    draw_house_art(Rect::new(20.0, 104.0, 142.0, 104.0), &auction.property);
    draw_wrapped_text(
        &auction.property.address,
        20.0,
        240.0,
        146.0,
        20,
        TEXT_BRIGHT,
    );
    if auction.is_running() && auction.next_bid() >= auction.player_walkaway_price {
        let margin = projected_purchase_margin(&auction.property, auction.next_bid(), app.market());
        draw_wrapped_text(
            &format!("Est. margin {}", format_money(margin)),
            20.0,
            340.0,
            146.0,
            17,
            WARNING,
        );
    }
    None
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
    let x = |price| 214.0 + scale.position(price) * 676.0;
    let limit_x = x(auction.player_walkaway_price);
    let bid_x = x(auction.current_bid);
    let over = auction.current_bid >= auction.player_walkaway_price;
    draw_line(214.0, 482.0, 890.0, 482.0, 2.0, PANEL_EDGE);
    draw_circle(bid_x, 482.0, 5.0, if over { NEGATIVE } else { ACCENT });
    draw_line(x(estimate), 477.0, x(estimate), 487.0, 2.0, TEXT_DIM);
    draw_line(limit_x, 475.0, limit_x, 489.0, 2.0, WARNING);
    label(
        &format!("Est. {}", format_money(estimate)),
        (x(estimate) - 60.0).clamp(214.0, 752.0),
        508.0,
        14,
        TEXT_DIM,
    );
    label(
        &format!("Limit {}", format_money(auction.player_walkaway_price)),
        (limit_x - 70.0).clamp(214.0, 740.0),
        468.0,
        14,
        if over { NEGATIVE } else { WARNING },
    );
}

fn draw_rival_notes(auction: &Auction, index: usize) -> Option<AuctionUiAction> {
    let bidder = &auction.bidders[index];
    crate::screens::auction_scene::reacting_actor(Rect::new(35.0, 104.0, 104.0, 122.0), bidder);
    draw_wrapped_text(&bidder.name, 20.0, 265.0, 146.0, 20, TEXT_BRIGHT);
    label(bidder.bidder_type.label(), 20.0, 328.0, 14, TEXT_DIM);
    draw_wrapped_text(&bidder.tell, 20.0, 370.0, 146.0, 17, TEXT);
    if button(
        Rect::new(16.0, 450.0, 150.0, 44.0),
        "BACK TO LOT",
        true,
        ButtonTone::Ghost,
    ) {
        return Some(AuctionUiAction::FocusRival(index));
    }
    None
}
