use crate::model::{Auction, BidderActor};
use crate::screens::auction_widgets::mood_color;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn current_bid_caption(auction: &Auction) -> String {
    match auction.last_bidder.as_ref() {
        Some(BidderActor::Player) => {
            format!(
                "Current Bid · Paddle {} leads",
                auction.player_paddle_number()
            )
        }
        Some(BidderActor::Npc(index)) => format!(
            "Current Bid · {} leads",
            auction
                .bidders
                .get(*index)
                .map(|bidder| bidder.name.as_str())
                .unwrap_or("another bidder")
        ),
        Some(BidderActor::Vendor) => "Declared Vendor Bid · not yet selling".to_string(),
        None => "Opening Call · no leading bidder".to_string(),
    }
}

pub(super) fn draw_bidder_panel(rect: Rect, auction: &Auction) {
    let active = auction
        .bidders
        .iter()
        .filter(|bidder| bidder.active)
        .count();
    label(
        &format!("IN THE ROOM / {active} active"),
        rect.x,
        rect.y + 24.0,
        17,
        TEXT_DIM,
    );
    for (index, bidder) in auction.bidders.iter().enumerate() {
        let y = rect.y + 66.0 + index as f32 * 116.0;
        let leading = auction.last_bidder == Some(BidderActor::Npc(index));
        label_fit(
            &bidder.name,
            rect.x,
            y,
            rect.w,
            24,
            if bidder.active { TEXT_BRIGHT } else { TEXT_DIM },
        );
        label(bidder.bidder_type.label(), rect.x, y + 25.0, 17, TEXT_DIM);
        label(
            if leading {
                "Leading"
            } else {
                bidder.mood.label()
            },
            rect.x + 178.0,
            y + 25.0,
            17,
            if leading {
                POSITIVE
            } else {
                mood_color(bidder.mood)
            },
        );
        draw_wrapped_text(&bidder.tell, rect.x, y + 51.0, rect.w, 16, TEXT_DIM);
    }
}
