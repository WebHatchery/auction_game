//! Small pixel actors shared by the auctioneer, competing seats and rival portraits.
use crate::model::{BidderMood, BidderType};
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn actor(rect: Rect, kind: BidderType, mood: BidderMood, raised: bool) {
    let unit = rect.w / 24.0;
    let mut skin = Color::from_rgba(204, 153, 112, 255);
    let (mut coat, hair) = match kind {
        BidderType::Investor => (
            Color::from_rgba(67, 98, 109, 255),
            Color::from_rgba(53, 46, 43, 255),
        ),
        BidderType::Renovator => (
            Color::from_rgba(182, 116, 66, 255),
            Color::from_rgba(87, 50, 36, 255),
        ),
        BidderType::Developer => (
            Color::from_rgba(103, 104, 130, 255),
            Color::from_rgba(40, 38, 43, 255),
        ),
        BidderType::FirstHomeBuyer => (
            Color::from_rgba(104, 141, 115, 255),
            Color::from_rgba(174, 135, 75, 255),
        ),
        BidderType::EgoBidder => (
            Color::from_rgba(151, 69, 71, 255),
            Color::from_rgba(43, 36, 32, 255),
        ),
        BidderType::BargainHunter => (
            Color::from_rgba(130, 116, 88, 255),
            Color::from_rgba(149, 150, 139, 255),
        ),
    };
    if mood == BidderMood::Out {
        coat = PANEL_EDGE;
        skin = TEXT_DIM;
    }
    let pixel = |x: f32, y: f32, w: f32, h: f32, color| {
        draw_rectangle(
            rect.x + x * unit,
            rect.y + y * unit,
            w * unit,
            h * unit,
            color,
        )
    };
    pixel(3.0, 19.0, 18.0, 9.0, coat);
    pixel(7.0, 16.0, 10.0, 4.0, coat);
    pixel(10.0, 14.0, 5.0, 5.0, skin);
    pixel(6.0, 4.0, 12.0, 10.0, hair);
    pixel(8.0, 7.0, 10.0, 9.0, skin);
    pixel(7.0, 3.0, 11.0, 5.0, hair);
    pixel(6.0, 7.0, 3.0, 6.0, hair);
    pixel(10.0, 10.0, 2.0, 2.0, BACKGROUND);
    pixel(15.0, 10.0, 2.0, 2.0, BACKGROUND);
    pixel(12.0, 14.0, 3.0, 1.0, hair);
    if matches!(kind, BidderType::Investor | BidderType::BargainHunter) {
        pixel(9.0, 9.0, 9.0, 1.0, TEXT_BRIGHT);
        pixel(9.0, 12.0, 4.0, 1.0, hair);
        pixel(14.0, 12.0, 4.0, 1.0, hair);
    }
    if kind == BidderType::Renovator {
        pixel(7.0, 19.0, 2.0, 9.0, WARNING);
        pixel(16.0, 19.0, 2.0, 9.0, WARNING);
    } else {
        pixel(11.0, 19.0, 4.0, 5.0, TEXT_BRIGHT);
        pixel(12.0, 20.0, 2.0, 8.0, hair);
    }
    if raised {
        pixel(20.0, 11.0, 3.0, 10.0, coat);
        pixel(20.0, 8.0, 3.0, 4.0, skin);
        pixel(21.0, 2.0, 1.0, 9.0, ACCENT);
        pixel(18.0, 0.0, 7.0, 5.0, ACCENT);
        pixel(20.0, 1.0, 3.0, 2.0, BACKGROUND);
    } else if matches!(mood, BidderMood::Hesitating | BidderMood::Stretching) {
        pixel(15.0, 15.0, 3.0, 4.0, skin);
        pixel(17.0, 18.0, 4.0, 5.0, coat);
    }
}

pub(super) fn room(auction: &crate::model::Auction, urgent: bool) {
    let wall = Color::from_rgba(30, 38, 40, 255);
    let wood = Color::from_rgba(72, 59, 49, 255);
    draw_rectangle(224.0, 70.0, 694.0, 384.0, wall);
    // Light narrows towards the rostrum as the room runs out of time.
    let spread = if urgent { 180.0 } else { 310.0 };
    draw_triangle(
        vec2(571.0, 71.0),
        vec2(571.0 - spread, 348.0),
        vec2(571.0 + spread, 348.0),
        Color::from_rgba(49, 49, 41, 255),
    );
    for x in [236.0, 896.0] {
        draw_rectangle(x, 70.0, 9.0, 295.0, wood);
        draw_rectangle(x - 5.0, 70.0, 19.0, 6.0, ACCENT);
    }
    draw_rectangle(251.0, 82.0, 640.0, 5.0, wood);
    actor(
        Rect::new(539.0, 92.0, 56.0, 65.0),
        BidderType::Investor,
        BidderMood::Interested,
        false,
    );
    draw_rectangle(529.0, 152.0, 91.0, 12.0, wood);
    draw_rectangle(538.0, 164.0, 73.0, 18.0, Color::from_rgba(51, 43, 37, 255));
    draw_rectangle(556.0, 168.0, 36.0, 3.0, ACCENT);
    // The gavel shifts from resting on the desk to a raised final-call pose.
    if urgent {
        draw_rectangle(596.0, 124.0, 7.0, 25.0, TEXT_DIM);
    }
    let hammer_y = if urgent { 119.0 } else { 147.0 };
    draw_rectangle(603.0, hammer_y, 19.0, 7.0, ACCENT);
    draw_rectangle(610.0, hammer_y + 5.0, 3.0, 19.0, wood);
    for (index, bidder) in auction.bidders.iter().enumerate() {
        let x = [252.0, 834.0, 770.0][index % 3];
        let y = if index == 2 { 116.0 } else { 151.0 };
        let leading = auction.last_bidder == Some(crate::model::BidderActor::Npc(index));
        actor(
            Rect::new(x, y, 45.0, 52.0),
            bidder.bidder_type,
            bidder.mood,
            leading,
        );
        draw_rectangle(
            x - 2.0,
            y + 45.0,
            48.0,
            18.0,
            if bidder.active { wood } else { PANEL_DARK },
        );
        if leading {
            draw_rectangle(x, y + 65.0, 44.0, 3.0, ACCENT);
        }
    }
    draw_rectangle(224.0, 440.0, 694.0, 14.0, wood);
    draw_line(224.0, 440.0, 918.0, 440.0, 2.0, PANEL_EDGE);
}
