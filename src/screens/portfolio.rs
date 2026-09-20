use crate::app::App;
use crate::model::PropertyId;
use crate::screens::portfolio_finance_widgets::{
    draw_finance_summary, draw_loan_control, LoanAction,
};
use crate::screens::portfolio_rent_review::{draw_rent_review_decision, RentReviewChoice};
use crate::screens::portfolio_sale_widgets::{
    draw_marketing_selector, draw_sale_summary, draw_sell_decision,
};
use crate::screens::portfolio_widgets::{
    draw_active_project_decision, draw_contractor_selector, draw_empty_portfolio,
    draw_hold_decision, draw_lease_decision, draw_maintenance_decision, draw_problem_card,
    draw_rental_campaign, draw_skip_renovation, draw_upgrade_decision, recommended_upgrade,
};
use crate::sim::campaign::{
    annual_interest_rate_percent, portfolio_weekly_cashflow, weekly_debt_interest,
};
use crate::sim::finance::{borrowing_limit, property_cashflow, refinance_capacity, REFINANCE_FEE};
use crate::sim::rental::{
    leasing_cost, portfolio_rental_snapshot, proposed_review_rent, rent_review_due,
    rent_review_outlook, weekly_rent_for_owned,
};
use crate::sim::valuation::current_value;
use crate::ui::*;
use macroquad::prelude::*;

impl App {
    pub(crate) fn draw_portfolio(&mut self) {
        label("Next Move", 28.0, 106.0, 30, TEXT_BRIGHT);
        if self.player.properties.is_empty() {
            draw_empty_portfolio(self);
            return;
        }

        self.portfolio_index = self
            .portfolio_index
            .min(self.player.properties.len().saturating_sub(1));
        let rental = portfolio_rental_snapshot(&self.player);
        let portfolio_has_due_review = self.player.properties.iter().any(rent_review_due);
        let portfolio_cashflow = portfolio_weekly_cashflow(&self.player, self.market());
        label(
            &format!(
                "{} homes | {} leased | {} gross rent | {} weekly cashflow",
                self.player.properties.len(),
                self.player
                    .properties
                    .iter()
                    .filter(|owned| owned.is_leased)
                    .count(),
                format_money(rental.gross_rent),
                format_money(portfolio_cashflow)
            ),
            212.0,
            106.0,
            17,
            if portfolio_cashflow >= 0 {
                POSITIVE
            } else {
                WARNING
            },
        );

        let page_size = if self.player.properties.len() > 6 {
            5
        } else {
            6
        };
        let page_count = self.player.properties.len().div_ceil(page_size);
        self.portfolio_page = self.portfolio_page.min(page_count.saturating_sub(1));
        let page_start = self.portfolio_page * page_size;
        let page_end = (page_start + page_size).min(self.player.properties.len());
        if self.portfolio_index < page_start || self.portfolio_index >= page_end {
            self.portfolio_index = page_start;
        }
        let count = page_end.saturating_sub(page_start);
        let selector_gap = 10.0;
        let selector_w = (ui_width()
            - 56.0
            - if self.player.properties.len() > 6 {
                80.0
            } else {
                0.0
            }
            - selector_gap * (count.saturating_sub(1)) as f32)
            / count as f32;
        let mut selected = None;
        for (local_index, property) in self.player.properties[page_start..page_end]
            .iter()
            .enumerate()
        {
            let index = page_start + local_index;
            let property_week = property_cashflow(property, self.market());
            let rect = Rect::new(
                28.0 + if self.player.properties.len() > 6 {
                    80.0
                } else {
                    0.0
                } + local_index as f32 * (selector_w + selector_gap),
                124.0,
                selector_w,
                68.0,
            );
            if index == self.portfolio_index {
                highlight_panel(rect);
            } else {
                soft_panel(rect);
            }
            label_fit(
                &property.property.address,
                rect.x + 10.0,
                rect.y + 25.0,
                rect.w - 20.0,
                16,
                TEXT_BRIGHT,
            );
            let lease_label = if property.leasing_weeks_remaining > 0 {
                format!(
                    "On rental market | {} / wk",
                    format_money(property.weekly_rent)
                )
            } else if rent_review_due(property) {
                format!(
                    "Rent review due | {} / wk",
                    format_money(property.weekly_rent)
                )
            } else if property.is_leased {
                if let Some(issue) = &property.maintenance_issue {
                    format!(
                        "{} | {} net / wk",
                        issue.kind.label(),
                        format_money(property_week.net_cashflow)
                    )
                } else {
                    format!(
                        "Leased {} / wk | {} net",
                        format_money(property.weekly_rent),
                        format_money(property_week.net_cashflow)
                    )
                }
            } else {
                "Vacant".to_string()
            };
            label(
                &lease_label,
                rect.x + 10.0,
                rect.y + 50.0,
                14,
                if property.maintenance_issue.is_some() {
                    NEGATIVE
                } else if rent_review_due(property) {
                    WARNING
                } else if property.is_leased && property_week.net_cashflow >= 0 {
                    POSITIVE
                } else if property.is_leased {
                    WARNING
                } else if property.leasing_weeks_remaining > 0 {
                    crate::ui::BLUE
                } else {
                    WARNING
                },
            );
            if rect_clicked(rect) {
                selected = Some(index);
            }
        }
        if let Some(index) = selected {
            self.portfolio_index = index;
        }

        if self.player.properties.len() > 6 {
            if button(
                Rect::new(28.0, 124.0, 70.0, 30.0),
                "<",
                self.portfolio_page > 0,
                ButtonTone::Ghost,
            ) {
                self.portfolio_page = self.portfolio_page.saturating_sub(1);
            }
            if button(
                Rect::new(28.0, 160.0, 70.0, 30.0),
                ">",
                self.portfolio_page + 1 < page_count,
                ButtonTone::Ghost,
            ) {
                self.portfolio_page += 1;
            }
            label(
                &format!(
                    "{}–{} / {}",
                    page_start + 1,
                    page_end,
                    self.player.properties.len()
                ),
                30.0,
                194.0,
                14,
                TEXT_DIM,
            );
        }

        let owned = self.player.properties[self.portfolio_index].clone();
        let estimate = current_value(&owned, self.market());
        let property_week = property_cashflow(&owned, self.market());
        let paydown_amount = 10_000.min(owned.debt).max(0);
        let paydown_interest_saving = property_week.loan_interest
            - weekly_debt_interest(owned.debt - paydown_amount, self.market());
        let refinance_room = refinance_capacity(&self.player, owned.property.id, self.market());
        let refinance_interest_increase =
            weekly_debt_interest(owned.debt + refinance_room, self.market())
                - property_week.loan_interest;
        let position = estimate
            - owned.purchase_price
            - owned.purchase_fees
            - owned.upgrade_spend()
            - owned.holding_spend();
        let bank_room = borrowing_limit(&self.player, self.market()) - self.player.debt;
        let has_active_project = owned.active_renovation.is_some();
        let main = Rect::new(28.0, 208.0, ui_width() - 56.0, ui_height() - 250.0);
        soft_panel(main);

        draw_house_art(
            Rect::new(main.x + 16.0, main.y + 16.0, 330.0, 208.0),
            &owned.property,
        );
        label(
            &owned.property.address,
            main.x + 370.0,
            main.y + 42.0,
            30,
            TEXT_BRIGHT,
        );
        label_fit(
            &format!(
                "Bought {} | Held {}w | {} | {:.1}% loan · {}/wk interest",
                format_money(owned.purchase_price),
                owned.weeks_held,
                owned.property.condition.label(),
                annual_interest_rate_percent(self.market()),
                format_money(property_week.loan_interest)
            ),
            main.x + 372.0,
            main.y + 72.0,
            main.w - 690.0,
            17,
            TEXT_DIM,
        );
        label(
            &format!(
                "Deposit {} | Rent earned {} | Rental profit {}",
                format_money(owned.deposit_paid),
                format_money(owned.rent_received),
                format_money(owned.rental_profit())
            ),
            main.x + 372.0,
            main.y + 92.0,
            15,
            TEXT_DIM,
        );

        let stat_y = main.y + 112.0;
        draw_money_stat(
            "Current Estimate",
            &format_money(estimate),
            &format!("Equity {}", format_money(estimate - owned.debt)),
            Rect::new(main.x + 370.0, stat_y, 220.0, 78.0),
            if position >= 0 { POSITIVE } else { WARNING },
        );
        draw_money_stat(
            "Projected Position",
            &format_money(position),
            "Before sale result",
            Rect::new(main.x + 606.0, stat_y, 220.0, 78.0),
            if position >= 0 { POSITIVE } else { NEGATIVE },
        );
        draw_problem_card(
            Rect::new(main.x + 842.0, stat_y, main.w - 862.0, 128.0),
            &owned,
            bank_room,
            self.market(),
        );
        let loan_rect = Rect::new(main.x + main.w - 298.0, main.y + 14.0, 280.0, 104.0);
        let lvr_percent = if estimate > 0 {
            owned.debt as f32 / estimate as f32 * 100.0
        } else {
            0.0
        };
        let loan_action = if self.portfolio_finance_open {
            draw_loan_control(
                loan_rect,
                &owned,
                self.player.cash,
                paydown_interest_saving,
                refinance_room,
                (refinance_room - REFINANCE_FEE).max(0),
                refinance_interest_increase,
                lvr_percent,
            )
        } else if draw_finance_summary(loan_rect, &owned, lvr_percent) {
            self.portfolio_finance_open = true;
            None
        } else {
            None
        };

        let card_y = main.y + 272.0;
        let card_w = (main.w - 54.0) / 3.0;
        let mut upgrade_action: Option<(PropertyId, String)> = None;
        let mut hold_week = false;
        let mut lease = false;
        let mut end_tenancy_action = false;
        let mut repair_maintenance_action = false;
        let mut rent_review_action = None;

        if owned.maintenance_issue.is_some() {
            if draw_maintenance_decision(
                Rect::new(main.x + 18.0, card_y, card_w, 160.0),
                &owned,
                self.player.cash,
            ) {
                repair_maintenance_action = true;
            }
        } else if rent_review_due(&owned) {
            rent_review_action = draw_rent_review_decision(
                Rect::new(main.x + 18.0, card_y, card_w, 160.0),
                &owned,
                proposed_review_rent(&owned, self.market()),
                rent_review_outlook(&owned, self.market()),
            );
        } else if has_active_project {
            draw_active_project_decision(Rect::new(main.x + 18.0, card_y, card_w, 160.0), &owned);
        } else if owned.leasing_weeks_remaining > 0 {
            draw_rental_campaign(Rect::new(main.x + 18.0, card_y, card_w, 160.0), &owned);
        } else if owned.is_leased {
            if draw_skip_renovation(
                Rect::new(main.x + 18.0, card_y, card_w, 160.0),
                &owned,
                self.player.cash,
            ) {
                end_tenancy_action = true;
            }
        } else if let Some((upgrade, quote)) = recommended_upgrade(self, &owned) {
            if draw_upgrade_decision(
                Rect::new(main.x + 18.0, card_y, card_w, 160.0),
                upgrade,
                &quote,
                self.player.cash,
                owned.has_upgrade(&upgrade.id),
            ) {
                upgrade_action = Some((owned.property.id, upgrade.id.clone()));
            }
        } else {
            draw_skip_renovation(
                Rect::new(main.x + 18.0, card_y, card_w, 160.0),
                &owned,
                self.player.cash,
            );
        }

        let hold_rect = Rect::new(main.x + 36.0 + card_w, card_y, card_w, 160.0);
        if owned.is_leased || owned.leasing_weeks_remaining > 0 {
            if draw_hold_decision(
                hold_rect,
                &owned,
                property_week.net_cashflow,
                portfolio_has_due_review,
                self.campaign_status.is_finished(),
            ) {
                hold_week = true;
            }
        } else {
            let asking_rent = weekly_rent_for_owned(&owned, self.market());
            if draw_lease_decision(
                hold_rect,
                asking_rent,
                leasing_cost(asking_rent),
                self.player.cash,
                has_active_project
                    || (owned.hidden_defect_discovered && !owned.has_defect_repair()),
            ) {
                lease = true;
            }
        }

        let sale_rect = Rect::new(main.x + 54.0 + card_w * 2.0, card_y, card_w, 160.0);
        let sale_action = if self.portfolio_sale_open {
            let action = draw_sell_decision(
                sale_rect,
                position,
                has_active_project,
                self.selected_marketing_plan,
                self.player.cash,
            );
            if button(
                Rect::new(
                    sale_rect.x + sale_rect.w - 78.0,
                    sale_rect.y + 8.0,
                    62.0,
                    22.0,
                ),
                "CLOSE",
                true,
                ButtonTone::Ghost,
            ) {
                self.portfolio_sale_open = false;
                None
            } else {
                action
            }
        } else if draw_sale_summary(sale_rect, position, has_active_project) {
            self.portfolio_sale_open = true;
            None
        } else {
            None
        };

        let urgent = owned.maintenance_issue.is_some() || rent_review_due(&owned);
        let eligible_for_contractor = !urgent && !has_active_project && !owned.is_leased;
        if eligible_for_contractor && !self.portfolio_sale_open {
            draw_contractor_selector(self, main, false);
        }
        if self.portfolio_sale_open {
            draw_marketing_selector(self, main, &owned);
        }

        if loan_action == Some(LoanAction::PayDown) {
            self.pay_down_property_debt(owned.property.id);
        } else if loan_action == Some(LoanAction::Refinance) {
            self.refinance_owned_property(owned.property.id);
        } else if loan_action == Some(LoanAction::Close) {
            self.portfolio_finance_open = false;
        } else if let Some((property_id, upgrade_id)) = upgrade_action {
            self.buy_upgrade(property_id, &upgrade_id);
        }
        if repair_maintenance_action {
            self.repair_property_maintenance(owned.property.id);
        } else if let Some(choice) = rent_review_action {
            self.review_property_rent(owned.property.id, choice == RentReviewChoice::TestMarket);
        } else if end_tenancy_action {
            self.end_property_tenancy(owned.property.id);
        } else if lease {
            self.lease_property(owned.property.id);
        } else if let Some(choice) = sale_action {
            self.sell_property(owned.property.id, choice);
        } else if hold_week {
            self.advance_week();
        }
    }
}
