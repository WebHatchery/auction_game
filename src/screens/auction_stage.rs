//! The bidding stage keeps price, room signals and the next decision together.
use crate::app::App;
use crate::model::{Auction, BidderActor};
use crate::screens::auction::AuctionUiAction;
use crate::screens::auction_room_panel::{current_bid_caption, draw_bidder_panel};
use crate::screens::auction_widgets::{bid_verdict, money_color};
use crate::sim::finance::{finance_snapshot, rental_underwrite, FinanceSnapshot};
use crate::sim::research::estimate_reserve;
use crate::sim::valuation::projected_purchase_margin;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_live_stage(
    app: &App,
    auction: &Auction,
    finance: FinanceSnapshot,
) -> Option<AuctionUiAction> {
    let stage = Rect::new(40.0, 90.0, 790.0, 550.0);
    label(&auction.property.address, stage.x, 112.0, 26, TEXT_BRIGHT);
    label(&auction.property.suburb, stage.x, 140.0, 17, TEXT_DIM);
    draw_bid_focus(stage, auction, app.auction_beat.as_ref());
    draw_guardrails(app, auction, finance);
    if let Some(read) = app
        .auction_read
        .as_ref()
        .filter(|(_, remaining)| *remaining > 0.0)
        .map(|(read, _)| read.as_str())
        .or(auction.last_room_read.as_deref())
    {
        draw_wrapped_text(
            &format!("Last read: {read}"),
            884.0,
            502.0,
            276.0,
            15,
            crate::ui::BLUE,
        );
    } else {
        draw_wrapped_text(
            "Tap WAIT & READ ROOM to invite a response and reveal a tell.",
            884.0,
            502.0,
            276.0,
            15,
            TEXT_DIM,
        );
    }
    if app.auction_notes_open {
        draw_notes(app, auction);
    } else {
        draw_bidder_panel(Rect::new(884.0, 94.0, 276.0, 462.0), auction);
    }
    if button(
        Rect::new(884.0, 586.0, 276.0, 44.0),
        if app.auction_notes_open {
            "CLOSE PROPERTY NOTES"
        } else {
            "PROPERTY NOTES"
        },
        true,
        ButtonTone::Ghost,
    ) {
        return Some(AuctionUiAction::ToggleNotes);
    }
    draw_actions(stage, app, auction, finance)
}

fn draw_bid_focus(rect: Rect, auction: &Auction, beat: Option<&(String, f32)>) {
    let state = if auction.seconds_remaining <= 3.0 {
        "GOING TWICE"
    } else if auction.seconds_remaining <= 6.0 {
        "GOING ONCE"
    } else if auction.seconds_remaining <= 10.0 {
        "FINAL CALLS"
    } else if auction.seconds_remaining <= 18.0 {
        "SLOWING"
    } else {
        "STEADY INTEREST"
    };
    let urgent = auction.seconds_remaining <= 6.0;
    let color = if urgent { WARNING } else { TEXT_DIM };
    draw_centered_label(state, Rect::new(rect.x, 170.0, rect.w, 26.0), 22, color);
    draw_centered_label(
        &current_bid_caption(auction),
        Rect::new(rect.x, 207.0, rect.w, 24.0),
        17,
        TEXT_DIM,
    );
    draw_centered_label(
        &format_money(auction.current_bid),
        Rect::new(rect.x, 242.0, rect.w, 90.0),
        74,
        ACCENT,
    );
    draw_centered_label(
        &format!(
            "At {}, looking for {}.",
            format_money(auction.current_bid),
            format_money(auction.next_bid())
        ),
        Rect::new(rect.x, 351.0, rect.w, 30.0),
        24,
        TEXT_BRIGHT,
    );
    let call = if urgent {
        format!(
            "{:.0}s to the hammer{}",
            auction.seconds_remaining.ceil(),
            if auction.on_market_announced {
                " / selling"
            } else {
                " / reserve not met"
            }
        )
    } else if auction.on_market_announced {
        "On the market. The highest bid buys it.".to_string()
    } else {
        "Reserve not met. The vendor is not yet selling.".to_string()
    };
    draw_centered_label(&call, Rect::new(rect.x, 387.0, rect.w, 24.0), 16, color);
    if let Some((text, remaining)) = beat.filter(|(_, remaining)| *remaining > 0.0) {
        let mut ink = crate::ui::BLUE;
        ink.a = remaining.min(1.0);
        draw_centered_label(text, Rect::new(rect.x, 425.0, rect.w, 26.0), 20, ink);
    }
}

fn draw_guardrails(app: &App, auction: &Auction, finance: FinanceSnapshot) {
    let margin = projected_purchase_margin(&auction.property, auction.next_bid(), app.market());
    let rows = [
        (
            "WALK-AWAY LIMIT",
            auction.player_walkaway_price,
            if auction.next_bid() >= auction.player_walkaway_price {
                WARNING
            } else {
                TEXT_BRIGHT
            },
        ),
        (
            "CASH AFTER BID",
            finance.cash_after_settle,
            if finance.cash_after_settle < finance.cash_buffer_target {
                WARNING
            } else {
                TEXT_DIM
            },
        ),
        (
            "BANK ROOM",
            finance.headroom_after,
            if finance.headroom_after < 0 {
                WARNING
            } else {
                TEXT_DIM
            },
        ),
        ("EXPECTED MARGIN", margin, money_color(margin)),
    ];
    for (index, (title, value, color)) in rows.iter().enumerate() {
        let x = 40.0 + index as f32 * 200.0;
        label(title, x, 474.0, 14, TEXT_DIM);
        label(&format_money(*value), x, 502.0, 24, *color);
    }
}

fn draw_actions(
    rect: Rect,
    app: &App,
    auction: &Auction,
    finance: FinanceSnapshot,
) -> Option<AuctionUiAction> {
    if !auction.is_player_active {
        label(
            "Your paddle is down. Your limit is protected.",
            rect.x,
            553.0,
            20,
            TEXT_DIM,
        );
        return button(
            Rect::new(rect.x, 580.0, 330.0, 54.0),
            "QUICK RESOLVE",
            true,
            ButtonTone::Primary,
        )
        .then_some(AuctionUiAction::QuickResolve);
    }
    let over = auction.next_bid() > auction.player_walkaway_price;
    let leading = auction.last_bidder == Some(BidderActor::Player);
    let jump_finance = finance_snapshot(&app.player, app.market(), auction.jump_bid());
    if button(
        Rect::new(rect.x, 525.0, 330.0, 64.0),
        &format!("RAISE {}", format_money(auction.next_bid())),
        finance.can_buy && !leading,
        if over {
            ButtonTone::Danger
        } else {
            ButtonTone::Primary
        },
    ) {
        return Some(AuctionUiAction::Bid);
    }
    if button(
        Rect::new(390.0, 533.0, 220.0, 48.0),
        &if auction.jump_bid_available {
            format!("ASSERT {}", format_money(auction.jump_bid()))
        } else {
            "ASSERT USED".to_string()
        },
        auction.jump_bid_available && jump_finance.can_buy && !leading,
        ButtonTone::Danger,
    ) {
        return Some(AuctionUiAction::JumpBid);
    }
    label("Double step. One use.", 390.0, 605.0, 13, WARNING);
    if button(
        Rect::new(630.0, 533.0, 200.0, 48.0),
        "WAIT & READ ROOM",
        true,
        ButtonTone::Secondary,
    ) {
        return Some(AuctionUiAction::Hold);
    }
    let margin = projected_purchase_margin(&auction.property, auction.next_bid(), app.market());
    label(
        if leading {
            "You lead. Watch the room."
        } else if !finance.can_buy {
            finance.stress.label()
        } else {
            bid_verdict(
                margin,
                finance.cash_after_settle,
                finance.cash_buffer_target,
                auction.next_bid(),
                auction.player_walkaway_price,
            )
        },
        rect.x,
        609.0,
        17,
        if over { WARNING } else { TEXT_DIM },
    );
    if button(
        Rect::new(650.0, 599.0, 180.0, 44.0),
        "WALK AWAY",
        true,
        if auction.next_bid() >= auction.player_walkaway_price {
            ButtonTone::Danger
        } else {
            ButtonTone::Ghost
        },
    ) {
        return Some(AuctionUiAction::WalkAway);
    }
    None
}

fn draw_notes(app: &App, auction: &Auction) {
    draw_house_art(Rect::new(884.0, 100.0, 276.0, 130.0), &auction.property);
    label("RESEARCH NOTES", 884.0, 263.0, 20, TEXT_BRIGHT);
    label(
        "Bidding continues while these are open.",
        884.0,
        289.0,
        14,
        WARNING,
    );
    let reserve = estimate_reserve(
        &auction.property,
        app.market(),
        auction.player_research_level,
        app.player.reputation,
    );
    let rental = rental_underwrite(&auction.property, app.market(), auction.next_bid());
    draw_value(
        "Guide",
        &format_money(auction.property.guide_price),
        884.0,
        328.0,
        276.0,
    );
    draw_value(
        "Reserve estimate",
        &format_money(reserve),
        884.0,
        365.0,
        276.0,
    );
    draw_value(
        "Rental / week",
        &format_money(rental.net_cashflow),
        884.0,
        402.0,
        276.0,
    );
    label(
        auction.player_research_level.label(),
        884.0,
        444.0,
        17,
        TEXT_DIM,
    );
}
