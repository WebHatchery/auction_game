use crate::model::OwnedProperty;
use crate::ui::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum LoanAction {
    PayDown,
    Refinance,
    Close,
}

pub(super) fn draw_finance_summary(rect: Rect, owned: &OwnedProperty, lvr_percent: f32) -> bool {
    dark_panel(rect);
    label("Finance", rect.x + 14.0, rect.y + 25.0, 18, TEXT_BRIGHT);
    label_fit(
        &format!("Debt {}", format_compact_money(owned.debt)),
        rect.x + 14.0,
        rect.y + 51.0,
        112.0,
        15,
        TEXT_DIM,
    );
    label_fit(
        &format!("LVR {lvr_percent:.0}%"),
        rect.x + 14.0,
        rect.y + 77.0,
        112.0,
        15,
        TEXT_DIM,
    );
    button(
        Rect::new(rect.x + rect.w - 142.0, rect.y + 22.0, 126.0, 44.0),
        "OPEN FINANCE",
        true,
        ButtonTone::Secondary,
    )
}

pub(super) fn draw_loan_control(
    rect: Rect,
    owned: &OwnedProperty,
    cash: i64,
    paydown_interest_saving: i64,
    refinance_capacity: i64,
    refinance_cash: i64,
    refinance_interest_increase: i64,
    lvr_percent: f32,
) -> Option<LoanAction> {
    dark_panel(rect);
    if button(
        Rect::new(rect.x + 12.0, rect.y + 4.0, 82.0, 22.0),
        "CLOSE",
        true,
        ButtonTone::Ghost,
    ) {
        return Some(LoanAction::Close);
    }
    label_fit(
        &format!(
            "{lvr_percent:.0}% · -{}/w",
            format_compact_money(paydown_interest_saving)
        ),
        rect.x + 14.0,
        rect.y + 34.0,
        rect.w - 174.0,
        13,
        TEXT_DIM,
    );
    label(
        &format_money(owned.debt),
        rect.x + 14.0,
        rect.y + 58.0,
        24,
        TEXT_BRIGHT,
    );
    let refinance_note = if refinance_capacity >= 10_000 {
        format!(
            "Cost +{}/wk",
            format_compact_money(refinance_interest_increase)
        )
    } else if owned.weeks_held < 4 {
        "Refi after 4w".to_string()
    } else {
        "No refi headroom".to_string()
    };
    label_fit(
        &refinance_note,
        rect.x + 14.0,
        rect.y + 82.0,
        rect.w - 174.0,
        13,
        POSITIVE,
    );
    if button(
        Rect::new(rect.x + rect.w - 154.0, rect.y + 28.0, 138.0, 34.0),
        "Pay Down $10k",
        owned.debt > 0 && cash >= 10_000,
        ButtonTone::Secondary,
    ) {
        return Some(LoanAction::PayDown);
    }
    let refinance_label = if refinance_capacity >= 10_000 {
        format!("Release {}", format_compact_money(refinance_cash))
    } else {
        "Release Equity".to_string()
    };
    if button(
        Rect::new(rect.x + rect.w - 154.0, rect.y + 66.0, 138.0, 34.0),
        &refinance_label,
        refinance_capacity >= 10_000,
        ButtonTone::Primary,
    ) {
        return Some(LoanAction::Refinance);
    }
    None
}
