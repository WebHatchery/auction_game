use crate::model::{Auction, BidderActor};
use crate::screens::auction_widgets::mood_color;
use crate::ui::auction_reactions::Reaction;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn current_bid_caption(auction: &Auction) -> String {
    match auction.last_bidder.as_ref() {
        Some(BidderActor::Player) => {
            format!("Your paddle {} leads", auction.player_paddle_number())
        }
        Some(BidderActor::Npc(index)) => format!(
            "{} leads",
            auction
                .bidders
                .get(*index)
                .map(|bidder| bidder.name.as_str())
                .unwrap_or("another bidder")
        ),
        Some(BidderActor::Vendor) => "Vendor bid".to_string(),
        None => String::new(),
    }
}

pub(super) fn draw_bidder_panel(
    rect: Rect,
    auction: &Auction,
    focus: Option<usize>,
) -> Option<crate::screens::auction::AuctionUiAction> {
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
        crate::screens::auction_scene::reacting_actor(
            Rect::new(rect.x, y + 15.0, 62.0, 73.0),
            bidder,
        );
        label(
            bidder.bidder_type.label(),
            rect.x + 79.0,
            y + 38.0,
            17,
            TEXT_DIM,
        );
        draw_circle(rect.x + 83.0, y + 64.0, 3.0, color);
        label_fit(
            Reaction::for_bidder(bidder, auction.next_bid(), auction.bid_increment).label(),
            rect.x + 94.0,
            y + 70.0,
            rect.w - 94.0,
            17,
            color,
        );
        if rect_clicked(Rect::new(rect.x - 8.0, y - 22.0, rect.w + 8.0, 160.0)) {
            action = Some(crate::screens::auction::AuctionUiAction::FocusRival(index));
        }
    }
    action
}
