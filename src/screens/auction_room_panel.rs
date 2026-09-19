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

pub(super) fn draw_bidder_panel(
    rect: Rect,
    auction: &Auction,
    focus: Option<usize>,
) -> Option<crate::screens::auction::AuctionUiAction> {
    label("THE COMPETITION", rect.x, rect.y + 18.0, 17, TEXT_DIM);
    let mut action = None;
    for (index, bidder) in auction.bidders.iter().enumerate() {
        let y = rect.y + 50.0 + index as f32 * 170.0;
        let leading = auction.last_bidder == Some(BidderActor::Npc(index));
        let color = if leading {
            ACCENT
        } else {
            mood_color(bidder.mood)
        };
        if focus == Some(index) {
            draw_rectangle(rect.x - 10.0, y - 18.0, 3.0, 150.0, crate::ui::BLUE);
        }
        label(
            &bidder.name,
            rect.x,
            y,
            20,
            if bidder.active { TEXT_BRIGHT } else { TEXT_DIM },
        );
        crate::screens::auction_scene::actor(
            Rect::new(rect.x, y + 15.0, 62.0, 73.0),
            bidder.bidder_type,
            bidder.mood,
            leading,
        );
        label(
            bidder.bidder_type.label(),
            rect.x + 79.0,
            y + 38.0,
            17,
            TEXT_DIM,
        );
        draw_circle(rect.x + 83.0, y + 64.0, 3.0, color);
        label(
            if leading {
                "LEADING"
            } else {
                bidder.mood.label()
            },
            rect.x + 94.0,
            y + 70.0,
            17,
            color,
        );
        draw_wrapped_text(&bidder.tell, rect.x, y + 113.0, rect.w, 17, TEXT);
        if rect_clicked(Rect::new(rect.x - 8.0, y - 22.0, rect.w + 8.0, 160.0)) {
            action = Some(crate::screens::auction::AuctionUiAction::FocusRival(index));
        }
    }
    label("Tap a rival to study them.", rect.x, 654.0, 14, TEXT_DIM);
    action
}
