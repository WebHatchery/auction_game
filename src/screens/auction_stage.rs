//! A live rostrum connects the auctioneer, price, reactions and bidding desk.
use crate::app::App;
use crate::model::Auction;
use crate::screens::auction::AuctionUiAction;
use crate::screens::auction_console::draw_console;
use crate::screens::auction_context::{draw_context, draw_pressure};
use crate::screens::auction_room_panel::{current_bid_caption, draw_bidder_panel};
use crate::screens::auction_scene::room;
use crate::sim::finance::FinanceSnapshot;
use crate::ui::auction_reactions::live_call;
use crate::ui::auction_view::CallPhase;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_live_stage(
    app: &App,
    auction: &Auction,
    finance: FinanceSnapshot,
) -> Option<AuctionUiAction> {
    let phase = CallPhase::for_room(auction.seconds_remaining, auction.last_bidder.is_some());
    room(auction, phase.urgent());
    draw_narration(app, auction, phase);
    draw_pressure(app, auction);
    let rival_action = draw_bidder_panel(
        Rect::new(941.0, 74.0, 235.0, 578.0),
        auction,
        app.auction_focus,
    );
    let context_action = draw_context(app, auction);
    let console_action = draw_console(app, auction, finance);
    rival_action.or(context_action).or(console_action)
}

fn draw_narration(app: &App, auction: &Auction, phase: CallPhase) {
    let urgent = phase.urgent();
    let beat = app
        .auction_beat
        .as_ref()
        .filter(|(_, remaining)| *remaining > 0.0);
    let bump = auction.last_bidder.is_some() && auction.seconds_since_bid < 0.18;
    draw_centered_label(
        &current_bid_caption(auction),
        Rect::new(208.0, 230.0, 686.0, 24.0),
        17,
        TEXT_DIM,
    );
    draw_centered_label(
        &format_money(auction.current_bid),
        Rect::new(206.0, 260.0, 690.0, 82.0),
        if bump { 80 } else { 74 },
        ACCENT,
    );
    let call = live_call(auction);
    draw_centered_label(
        &call,
        Rect::new(206.0, 346.0, 690.0, 42.0),
        if urgent { 32 } else { 26 },
        if urgent { WARNING } else { TEXT_BRIGHT },
    );
    if urgent {
        let knocks = if phase == CallPhase::Twice { 2 } else { 1 };
        for i in 0..2 {
            draw_rectangle(
                828.0 + i as f32 * 24.0,
                128.0,
                18.0,
                4.0,
                if i < knocks { WARNING } else { PANEL_EDGE },
            );
        }
        label(
            &format!("{}s", auction.seconds_remaining.ceil() as i32),
            846.0,
            116.0,
            20,
            WARNING,
        );
    }
    if let Some(read) = app
        .auction_read
        .as_ref()
        .filter(|(_, t)| *t > 0.0)
        .map(|(read, _)| read.as_str())
    {
        let mut ink = crate::ui::BLUE;
        ink.a = app
            .auction_read
            .as_ref()
            .map_or(0.0, |(_, remaining)| remaining.min(1.0));
        draw_wrapped_text(read, 264.0, 406.0, 600.0, 17, ink);
    } else if let Some((text, remaining)) = beat {
        let mut ink = TEXT;
        ink.a = remaining.min(1.0);
        draw_centered_label(text, Rect::new(246.0, 397.0, 650.0, 25.0), 20, ink);
    }
}

pub(super) fn draw_hammer(app: &App, auction: &Auction) -> Option<AuctionUiAction> {
    use crate::model::AuctionStatus;
    room(auction, false);
    let (headline, call, next) = match auction.status.as_ref()? {
        AuctionStatus::SoldToPlayer => (
            "SOLD TO YOU",
            "The room is yours. Now make the purchase work.".to_string(),
            "REVIEW PURCHASE",
        ),
        AuctionStatus::SoldToNpc(name) => (
            if auction.player_exit_bid.is_some() {
                "YOU WALKED AWAY"
            } else {
                "OUTBID AT THE HAMMER"
            },
            format!("Sold to {name}. Your capital stays with you."),
            "REVIEW OUTCOME",
        ),
        AuctionStatus::PassedIn => (
            "PASSED IN",
            "The hammer falls below reserve. The agent wants a word.".to_string(),
            "TALK TO THE AGENT",
        ),
    };
    draw_centered_label(
        headline,
        Rect::new(246.0, 210.0, 650.0, 35.0),
        32,
        TEXT_BRIGHT,
    );
    draw_centered_label(
        &format_money(auction.current_bid),
        Rect::new(246.0, 265.0, 650.0, 85.0),
        74,
        ACCENT,
    );
    draw_wrapped_text(&call, 273.0, 385.0, 600.0, 22, TEXT);
    let context = draw_context(app, auction);
    let rivals = draw_bidder_panel(
        Rect::new(941.0, 74.0, 235.0, 578.0),
        auction,
        app.auction_focus,
    );
    draw_pressure(app, auction);
    draw_rectangle(
        224.0,
        522.0,
        694.0,
        153.0,
        Color::from_rgba(37, 32, 29, 255),
    );
    draw_centered_label(
        "THE ROOM FALLS QUIET",
        Rect::new(246.0, 539.0, 650.0, 25.0),
        17,
        TEXT_DIM,
    );
    if button(
        Rect::new(381.0, 584.0, 384.0, 58.0),
        next,
        true,
        ButtonTone::Primary,
    ) {
        return Some(AuctionUiAction::ReviewOutcome);
    }
    context.or(rivals)
}
