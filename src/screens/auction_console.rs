//! The player's desk: a physical paddle, a risky jump and an observation control.
use crate::app::App;
use crate::model::{Auction, BidderActor};
use crate::screens::auction::AuctionUiAction;
use crate::sim::finance::{finance_snapshot, FinanceSnapshot};
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_console(
    app: &App,
    auction: &Auction,
    finance: FinanceSnapshot,
) -> Option<AuctionUiAction> {
    draw_rectangle(
        184.0,
        522.0,
        734.0,
        153.0,
        Color::from_rgba(37, 32, 29, 255),
    );
    draw_rectangle(184.0, 522.0, 734.0, 5.0, Color::from_rgba(92, 71, 49, 255));
    for x in [191.0, 910.0] {
        draw_circle(x, 530.0, 2.0, PANEL_EDGE);
    }
    if !auction.is_player_active {
        draw_centered_label(
            "PADDLE DOWN. YOU'RE OUT.",
            Rect::new(244.0, 541.0, 650.0, 30.0),
            26,
            TEXT_DIM,
        );
        return button(
            Rect::new(407.0, 590.0, 330.0, 58.0),
            "WATCH THE HAMMER FALL",
            true,
            ButtonTone::Primary,
        )
        .then_some(AuctionUiAction::QuickResolve);
    }
    let leading = auction.last_bidder == Some(BidderActor::Player);
    let enabled = finance.can_buy && !leading;
    let raise = Rect::new(421.0, 537.0, 263.0, 103.0);
    draw_paddle(raise, auction, enabled);
    if enabled && rect_clicked(raise) {
        return Some(AuctionUiAction::Bid);
    }
    let jump = Rect::new(203.0, 548.0, 188.0, 83.0);
    let jump_finance = finance_snapshot(&app.player, app.market(), auction.jump_bid());
    let jump_enabled = !leading && auction.jump_bid_available && jump_finance.can_buy;
    draw_jump(jump, auction, jump_enabled);
    if jump_enabled && rect_clicked(jump) {
        return Some(AuctionUiAction::JumpBid);
    }
    let observe = Rect::new(725.0, 542.0, 172.0, 93.0);
    draw_observe(
        observe,
        app.auction_read.as_ref().is_some_and(|(_, t)| *t > 0.0),
    );
    if rect_clicked(observe) {
        return Some(AuctionUiAction::Hold);
    }
    if !finance.can_buy && !leading {
        draw_centered_label(
            "Finance limit reached",
            Rect::new(240.0, 650.0, 656.0, 22.0),
            17,
            WARNING,
        );
    }
    if button(
        Rect::new(16.0, 532.0, 150.0, 54.0),
        "WALK AWAY",
        true,
        if auction.next_bid() >= auction.player_walkaway_price {
            ButtonTone::Danger
        } else {
            ButtonTone::Secondary
        },
    ) {
        return Some(AuctionUiAction::WalkAway);
    }
    None
}

fn draw_paddle(mut rect: Rect, auction: &Auction, enabled: bool) {
    let pointer = ui_pointer();
    if enabled && pointer.pressing(rect) {
        rect.y += 2.0;
    } else if enabled && pointer.hovering_over(rect) {
        rect.y -= 3.0;
    }
    let danger = auction.next_bid() >= auction.player_walkaway_price;
    let color = if !enabled {
        PANEL_EDGE
    } else if danger {
        WARNING
    } else {
        ACCENT
    };
    draw_rectangle(
        rect.x + rect.w * 0.5 - 12.0,
        rect.y + rect.h,
        24.0,
        7.0,
        color,
    );
    draw_rectangle(
        rect.x + 9.0,
        rect.y + 6.0,
        rect.w - 18.0,
        rect.h,
        BACKGROUND,
    );
    draw_rectangle(rect.x + 8.0, rect.y, rect.w - 16.0, rect.h, color);
    draw_rectangle(rect.x, rect.y + 8.0, rect.w, rect.h - 16.0, color);
    draw_rectangle(rect.x + 12.0, rect.y + 5.0, rect.w - 24.0, 2.0, TEXT_BRIGHT);
    draw_centered_label(
        "RAISE",
        Rect::new(rect.x, rect.y + 10.0, rect.w, 24.0),
        20,
        BACKGROUND,
    );
    draw_centered_label(
        &format_money(auction.next_bid()),
        Rect::new(rect.x, rect.y + 36.0, rect.w, 42.0),
        38,
        BACKGROUND,
    );
    draw_centered_label(
        &format!("PADDLE {}", auction.player_paddle_number()),
        Rect::new(rect.x, rect.y + 80.0, rect.w, 18.0),
        14,
        BACKGROUND,
    );
}

fn draw_jump(rect: Rect, auction: &Auction, enabled: bool) {
    let color = if enabled { WARNING } else { TEXT_DIM };
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, color);
    for offset in [0.0, 8.0] {
        draw_line(
            rect.x + 16.0 + offset,
            rect.y + 9.0,
            rect.x + 21.0 + offset,
            rect.y + 4.0,
            2.0,
            color,
        );
        draw_line(
            rect.x + 21.0 + offset,
            rect.y + 4.0,
            rect.x + 26.0 + offset,
            rect.y + 9.0,
            2.0,
            color,
        );
    }
    label(
        if auction.jump_bid_available {
            "JUMP"
        } else {
            "JUMP USED"
        },
        rect.x + 16.0,
        rect.y + 30.0,
        20,
        color,
    );
    label(
        &format_money(auction.jump_bid()),
        rect.x + 16.0,
        rect.y + 60.0,
        26,
        if enabled { TEXT_BRIGHT } else { TEXT_DIM },
    );
    if auction.jump_bid_available {
        label("1 USE", rect.x + 91.0, rect.y + 30.0, 14, color);
    }
}

fn draw_observe(rect: Rect, reading: bool) {
    let color = if reading { TEXT_BRIGHT } else { TEXT_DIM };
    draw_ellipse(rect.x + 23.0, rect.y + 14.0, 18.0, 9.0, 0.0, color);
    draw_circle(rect.x + 23.0, rect.y + 14.0, 6.0, BACKGROUND);
    label("WAIT", rect.x + 53.0, rect.y + 23.0, 22, color);
    label("& READ ROOM", rect.x + 3.0, rect.y + 50.0, 20, color);
}
