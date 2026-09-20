use crate::model::Auction;
use crate::ui::*;
use macroquad::prelude::*;

pub(super) fn draw_auction_property_panel(
    rect: Rect,
    auction: &Auction,
    reserve_estimate: i64,
    cash: i64,
    bank_room: i64,
    margin: i64,
    rental_cashflow: i64,
    price_basis: &str,
) {
    soft_panel(rect);
    draw_house_art(
        Rect::new(rect.x + 14.0, rect.y + 14.0, rect.w - 28.0, 154.0),
        &auction.property,
    );
    label(
        &auction.property.address,
        rect.x + 16.0,
        rect.y + 204.0,
        22,
        TEXT_BRIGHT,
    );
    label(
        &auction.property.suburb,
        rect.x + 16.0,
        rect.y + 230.0,
        17,
        TEXT_DIM,
    );
    let rows = [
        ("Reserve estimate".to_string(), reserve_estimate),
        ("Walk-away cap".to_string(), auction.player_walkaway_price),
        (format!("Cash to settle · {price_basis}"), cash),
        (format!("Bank room · {price_basis}"), bank_room),
        (format!("Margin after fees · {price_basis}"), margin),
        (
            format!("Rental cashflow / wk · {price_basis}"),
            rental_cashflow,
        ),
    ];
    for (index, (title, value)) in rows.iter().enumerate() {
        draw_value(
            title,
            &format_money(*value),
            rect.x + 16.0,
            rect.y + 276.0 + index as f32 * 34.0,
            rect.w - 32.0,
        );
    }
    label(
        if auction.is_running() {
            "Ready when you are."
        } else {
            "Hammer down. Review the outcome."
        },
        rect.x + 16.0,
        rect.y + rect.h - 24.0,
        17,
        TEXT_DIM,
    );
}
