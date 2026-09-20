use crate::app::App;
use crate::model::{Property, ResearchLevel, WalkawayStyle};
use crate::screens::Screen;
use crate::sim::finance::{finance_snapshot, rental_underwrite};
use crate::sim::rental::weekly_rent_for;
use crate::sim::research::{
    comparable_sale_value, due_diligence_note, estimate_reserve, known_risk_level,
    recommended_walkaway, research_cost, research_takeaway, researched_value_range, risk_summary,
    KnownRisk,
};
use crate::sim::valuation::{cash_needed_to_settle, projected_purchase_margin};
use crate::ui::*;
use macroquad::prelude::*;

impl App {
    pub(crate) fn draw_property_detail(&mut self, index: usize) {
        let Some(property) = self.available_properties.get(index).cloned() else {
            self.screen = Screen::PropertyList;
            return;
        };
        if self.property_report_open {
            draw_full_property_report(self, &property);
            return;
        }
        let research_level = self.research_level(property.id);
        let mut research_action = None;

        label(
            "Research / set your limit / register",
            28.0,
            106.0,
            18,
            crate::ui::BLUE,
        );
        label(&property.address, 28.0, 136.0, 32, TEXT_BRIGHT);
        label(
            &format!(
                "{} | {} bed, {} bath | {}sqm | {}",
                property.suburb,
                property.bedrooms,
                property.bathrooms,
                property.land_size,
                property.condition.label()
            ),
            30.0,
            164.0,
            17,
            TEXT_DIM,
        );

        let hero = Rect::new(28.0, 188.0, 500.0, 246.0);
        soft_panel(hero);
        draw_house_art(
            Rect::new(hero.x + 14.0, hero.y + 14.0, hero.w - 28.0, 176.0),
            &property,
        );
        draw_badge(
            property.condition.label().to_uppercase().as_str(),
            Rect::new(hero.x + 18.0, hero.y + 204.0, 92.0, 26.0),
            condition_color(&property),
        );
        draw_badge(
            risk_badge(&property, research_level),
            Rect::new(hero.x + 120.0, hero.y + 204.0, 142.0, 26.0),
            risk_color(&property, research_level),
        );
        label_fit(
            &property.notes,
            hero.x + 18.0,
            hero.y + 232.0,
            hero.w - 36.0,
            14,
            TEXT_DIM,
        );

        let decision = Rect::new(552.0, 122.0, ui_width() - 580.0, 416.0);
        soft_panel(decision);
        draw_detail_summary(self, decision, &property, research_level);

        for (button_index, level) in research_level.next_levels().iter().take(3).enumerate() {
            let button_rect = Rect::new(
                decision.x + 22.0 + button_index as f32 * 126.0,
                decision.y + decision.h - 46.0,
                114.0,
                42.0,
            );
            if button(
                button_rect,
                &research_button_label(*level, self.player.reputation),
                self.player.cash >= research_cost(*level, self.player.reputation),
                ButtonTone::Secondary,
            ) {
                research_action = Some(*level);
            }
        }

        if button(
            Rect::new(
                decision.x + decision.w - 286.0 + 16.0,
                decision.y + 62.0 + 276.0 - 76.0,
                228.0,
                40.0,
            ),
            "READ REPORT",
            true,
            ButtonTone::Ghost,
        ) {
            self.property_report_open = true;
        }

        let walk = Rect::new(28.0, ui_height() - 142.0, ui_width() - 56.0, 92.0);
        soft_panel(walk);
        draw_walkaway_panel(self, walk, &property);

        if let Some(level) = research_action {
            self.buy_research(property.id, level);
        }
    }
}

fn draw_detail_summary(app: &App, rect: Rect, property: &Property, research_level: ResearchLevel) {
    let (low, high) = researched_value_range(
        property,
        app.market(),
        research_level,
        app.player.reputation,
    );
    let walkaway = recommended_walkaway(
        property,
        app.market(),
        research_level,
        app.walkaway_style,
        app.player.reputation,
    );
    let margin = projected_purchase_margin(property, walkaway, app.market());
    let weekly_rent = weekly_rent_for(property, app.market());
    let gross_yield = weekly_rent as f32 * 52.0 / walkaway.max(1) as f32 * 100.0;

    label(
        &format!(
            "{}  ·  {} demand  ·  {}",
            property.deal_archetype.label(),
            demand_word(property.buyer_demand),
            research_level.confidence_label()
        ),
        rect.x + 22.0,
        rect.y + 30.0,
        16,
        ACCENT,
    );
    label("Guide price", rect.x + 22.0, rect.y + 70.0, 16, TEXT_DIM);
    label(
        &format_money(property.guide_price),
        rect.x + 22.0,
        rect.y + 104.0,
        34,
        ACCENT,
    );

    label(
        &format!("Known range · {}", research_level.label()),
        rect.x + 22.0,
        rect.y + 136.0,
        16,
        TEXT_DIM,
    );
    label(
        &format!("{} - {}", format_money(low), format_money(high)),
        rect.x + 22.0,
        rect.y + 164.0,
        23,
        TEXT_BRIGHT,
    );
    label_fit(
        &format!(
            "Reserve {}  ·  comparable {}",
            format_compact_money(estimate_reserve(
                property,
                app.market(),
                research_level,
                app.player.reputation
            )),
            format_compact_money(comparable_sale_value(property, app.market(), 0)),
        ),
        rect.x + 22.0,
        rect.y + 186.0,
        300.0,
        14,
        TEXT_DIM,
    );
    label_fit(
        &format!(
            "Rent {}/wk  ·  {:.1}% gross yield",
            format_compact_money(weekly_rent),
            gross_yield
        ),
        rect.x + 22.0,
        rect.y + 208.0,
        300.0,
        14,
        if gross_yield >= 5.0 {
            POSITIVE
        } else {
            WARNING
        },
    );

    label("Suggested cap", rect.x + 22.0, rect.y + 246.0, 16, TEXT_DIM);
    label(
        &format_money(walkaway),
        rect.x + 22.0,
        rect.y + 278.0,
        30,
        if margin >= 0 { POSITIVE } else { WARNING },
    );
    label(
        &format!(
            "{} plan  ·  projected margin {}",
            app.walkaway_style.label(),
            format_money(margin)
        ),
        rect.x + 22.0,
        rect.y + 304.0,
        16,
        if margin >= 0 { POSITIVE } else { WARNING },
    );

    let cap = app.walkaway_price;
    let finance = finance_snapshot(&app.player, app.market(), cap);
    let rental = rental_underwrite(property, app.market(), cap);
    let cap_label = if cap == walkaway {
        "Manual cap matches suggestion"
    } else {
        "Manual cap · suggestion not applied"
    };
    label_fit(
        cap_label,
        rect.x + 22.0,
        rect.y + 322.0,
        300.0,
        14,
        if finance.can_buy { TEXT_DIM } else { NEGATIVE },
    );
    label_fit(
        &format!(
            "Settle {}  ·  bank after {}",
            format_compact_money(cash_needed_to_settle(cap)),
            format_compact_money(finance.headroom_after)
        ),
        rect.x + 22.0,
        rect.y + 340.0,
        300.0,
        14,
        if finance.can_buy { TEXT_DIM } else { NEGATIVE },
    );
    label_fit(
        &format!(
            "Net rent {}/wk  ·  cash left {}",
            format_compact_money(rental.net_cashflow),
            format_compact_money(finance.cash_after_settle)
        ),
        rect.x + 22.0,
        rect.y + 358.0,
        300.0,
        14,
        if rental.net_cashflow >= 0 {
            POSITIVE
        } else {
            WARNING
        },
    );

    let report = Rect::new(rect.x + rect.w - 286.0, rect.y + 62.0, 260.0, 276.0);
    dark_panel(report);
    label(
        "Underwriting report",
        report.x + 16.0,
        report.y + 28.0,
        20,
        TEXT_BRIGHT,
    );
    label_fit(
        compact_research_question(research_level),
        report.x + 16.0,
        report.y + 58.0,
        report.w - 32.0,
        14,
        crate::ui::BLUE,
    );
    label_fit(
        &compact_risk_summary(property, research_level),
        report.x + 16.0,
        report.y + 92.0,
        report.w - 32.0,
        14,
        risk_color(property, research_level),
    );
    label_fit(
        compact_research_fit(research_level),
        report.x + 16.0,
        report.y + 126.0,
        report.w - 32.0,
        14,
        TEXT_DIM,
    );
    label_fit(
        "Full rationale + comparisons",
        report.x + 16.0,
        report.y + 186.0,
        report.w - 32.0,
        13,
        crate::ui::BLUE,
    );
}

fn compact_research_question(level: ResearchLevel) -> &'static str {
    match level {
        ResearchLevel::StreetScan => "Rough value + demand?",
        ResearchLevel::AgentPack => "Hidden risks + value?",
        ResearchLevel::BuildingInspection => "Defects + repair room?",
        ResearchLevel::FullDiligence => "Final risk + walk-away?",
    }
}

fn compact_risk_summary(property: &Property, level: ResearchLevel) -> String {
    match known_risk_level(property, level) {
        KnownRisk::Unverified => "Risk unverified: judge the house.".to_string(),
        KnownRisk::Low => "Risk low: no material defect flagged.".to_string(),
        KnownRisk::Moderate => "Risk moderate: keep a repair buffer.".to_string(),
        KnownRisk::Elevated => "Risk elevated: protect the downside.".to_string(),
    }
}

fn compact_research_fit(level: ResearchLevel) -> &'static str {
    match level {
        ResearchLevel::StreetScan => "Gap: more evidence needed.",
        ResearchLevel::AgentPack => "Gap: hidden risk remains.",
        ResearchLevel::BuildingInspection => "Fit: inspect defects before bidding.",
        ResearchLevel::FullDiligence => "Fit: final diligence applied.",
    }
}

fn draw_walkaway_panel(app: &mut App, rect: Rect, property: &Property) {
    let margin = projected_purchase_margin(property, app.walkaway_price, app.market());
    let finance = finance_snapshot(&app.player, app.market(), app.walkaway_price);
    let rental = rental_underwrite(property, app.market(), app.walkaway_price);
    label(
        "Walk-away Strategy",
        rect.x + 18.0,
        rect.y + 30.0,
        23,
        TEXT_BRIGHT,
    );
    let styles = [
        WalkawayStyle::Conservative,
        WalkawayStyle::Balanced,
        WalkawayStyle::Aggressive,
    ];
    for (index, style) in styles.iter().enumerate() {
        let selected = app.walkaway_style == *style;
        if button(
            Rect::new(
                rect.x + 18.0 + index as f32 * 112.0,
                rect.y + 42.0,
                102.0,
                28.0,
            ),
            style.label(),
            true,
            if selected {
                ButtonTone::Primary
            } else {
                ButtonTone::Ghost
            },
        ) {
            app.walkaway_style = *style;
            app.walkaway_price = recommended_walkaway(
                property,
                app.market(),
                app.research_level(property.id),
                app.walkaway_style,
                app.player.reputation,
            );
            app.status = format!(
                "{} walk-away selected: {}",
                style.label(),
                style.description()
            );
        }
    }
    label(
        app.walkaway_style.description(),
        rect.x + 18.0,
        rect.y + 84.0,
        14,
        TEXT_DIM,
    );
    label(
        &format_money(app.walkaway_price),
        rect.x + 386.0,
        rect.y + 52.0,
        34,
        if margin >= 0 { POSITIVE } else { WARNING },
    );
    label(
        walkaway_verdict(margin),
        rect.x + 600.0,
        rect.y + 48.0,
        19,
        if margin >= 0 { POSITIVE } else { WARNING },
    );
    label(
        &format!(
            "Cash: {} | Margin: {} | Rental cashflow: {}/wk | Bank: {}",
            format_money(cash_needed_to_settle(app.walkaway_price)),
            format_money(margin),
            format_money(rental.net_cashflow),
            format_money(finance.headroom_after)
        ),
        rect.x + 386.0,
        rect.y + 76.0,
        16,
        TEXT_DIM,
    );
    if button(
        Rect::new(rect.x + rect.w - 392.0, rect.y + 24.0, 74.0, 40.0),
        "-10k",
        true,
        ButtonTone::Ghost,
    ) {
        app.walkaway_price = (app.walkaway_price - 10_000).max(property.guide_price - 80_000);
    }
    if button(
        Rect::new(rect.x + rect.w - 308.0, rect.y + 24.0, 74.0, 40.0),
        "+10k",
        true,
        ButtonTone::Ghost,
    ) {
        app.walkaway_price += 10_000;
    }
    let can_register = app.auction_registrations > 0;
    if button(
        Rect::new(rect.x + rect.w - 220.0, rect.y + 18.0, 190.0, 52.0),
        if can_register {
            "REGISTER TO BID"
        } else {
            "RECOVER"
        },
        true,
        if can_register {
            ButtonTone::Primary
        } else {
            ButtonTone::Secondary
        },
    ) {
        if can_register {
            app.start_auction(property.id);
        } else {
            app.screen = Screen::Dashboard;
            app.status = "Tap ADVANCE WEEK to refresh your two registrations.".to_string();
        }
    }
}

fn draw_full_property_report(app: &mut App, property: &Property) {
    let level = app.research_level(property.id);
    let report = Rect::new(64.0, 86.0, ui_width() - 128.0, ui_height() - 126.0);
    soft_panel(report);
    label(
        "Full property report",
        report.x + 26.0,
        report.y + 42.0,
        30,
        TEXT_BRIGHT,
    );
    label(
        &format!(
            "{}  ·  {}  ·  {}",
            property.address,
            level.label(),
            level.confidence_label()
        ),
        report.x + 28.0,
        report.y + 72.0,
        17,
        TEXT_DIM,
    );

    let left = report.x + 28.0;
    let right = report.x + report.w * 0.52;
    let column_w = report.w * 0.43;
    let left_text = format!(
        "Thesis: {}\n\nMain risk: {}\n\nBest strategy: {}\n\nDo not: {}",
        property.thesis, property.main_risk, property.best_strategy, property.bad_strategy
    );
    let right_text = format!(
        "{}\n\n{}\n\n{}\n\n{}",
        risk_summary(property, level),
        research_takeaway(property, app.market(), level),
        due_diligence_note(property, level),
        property.notes
    );
    draw_wrapped_text(&left_text, left, report.y + 112.0, column_w, 17, TEXT);
    draw_wrapped_text(&right_text, right, report.y + 112.0, column_w, 17, TEXT);

    if button(
        Rect::new(
            report.x + report.w - 178.0,
            report.y + report.h - 60.0,
            150.0,
            44.0,
        ),
        "CLOSE REPORT",
        true,
        ButtonTone::Secondary,
    ) {
        app.property_report_open = false;
    }
}

fn walkaway_verdict(margin: i64) -> &'static str {
    if margin >= 45_000 {
        "Safe bid plan"
    } else if margin >= 0 {
        "Thin margin"
    } else {
        "Bad deal line"
    }
}

fn research_button_label(level: ResearchLevel, reputation: i32) -> String {
    match level {
        ResearchLevel::StreetScan => "Street $0".to_string(),
        ResearchLevel::AgentPack => format!(
            "Agent {}",
            format_compact_money(research_cost(level, reputation))
        ),
        ResearchLevel::BuildingInspection => {
            format!(
                "Build {}",
                format_compact_money(research_cost(level, reputation))
            )
        }
        ResearchLevel::FullDiligence => {
            format!(
                "Full {}",
                format_compact_money(research_cost(level, reputation))
            )
        }
    }
}

fn risk_badge(property: &Property, level: ResearchLevel) -> &'static str {
    match known_risk_level(property, level) {
        KnownRisk::Unverified => "RISK UNVERIFIED",
        KnownRisk::Low => "LOW RISK KNOWN",
        KnownRisk::Moderate => "MODERATE RISK",
        KnownRisk::Elevated => "ELEVATED RISK",
    }
}

fn risk_color(property: &Property, level: ResearchLevel) -> Color {
    match known_risk_level(property, level) {
        KnownRisk::Unverified => crate::ui::BLUE,
        KnownRisk::Low => POSITIVE,
        KnownRisk::Moderate => WARNING,
        KnownRisk::Elevated => NEGATIVE,
    }
}

fn demand_word(score: i32) -> &'static str {
    if score >= 72 {
        "hot"
    } else if score >= 55 {
        "steady"
    } else {
        "soft"
    }
}

fn condition_color(property: &Property) -> Color {
    match property.condition {
        crate::model::Condition::Rough => NEGATIVE,
        crate::model::Condition::Tired => WARNING,
        crate::model::Condition::Solid => POSITIVE,
        crate::model::Condition::Premium => crate::ui::BLUE,
    }
}
