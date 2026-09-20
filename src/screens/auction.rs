use crate::app::App;
use crate::model::{Auction, AuctionStatus};
use crate::screens::auction_debrief::draw_purchase_debrief;
use crate::screens::auction_lobby::{draw_auction_day_lobby, AuctionLobbyAction};
use crate::screens::auction_property_panel::draw_auction_property_panel;
use crate::screens::auction_stage::{draw_hammer, draw_live_stage};
use crate::screens::Screen;
use crate::sim::auction_events::{
    accept_post_auction_offer, post_auction_offer, test_vendor_at_passed_in_price, vendor_stance,
    PostAuctionTestResult,
};
use crate::sim::auction_sim::{
    award_discipline_reputation, begin_auction_calls, earned_discipline_reputation,
    hold_player_position, place_player_bid, place_player_jump_bid, quick_resolve_auction,
    stop_player_bidding,
};
use crate::sim::finance::{finance_snapshot, rental_underwrite};
use crate::sim::research::estimate_reserve;
use crate::sim::rival_notebook::record_completed_room;
use crate::sim::valuation::{cash_needed_to_settle, projected_purchase_margin};
use crate::ui::*;
use macroquad::prelude::*;

pub(super) enum AuctionUiAction {
    ReviewOutcome,
    FocusRival(usize),
    BeginAuction,
    Bid,
    JumpBid,
    Hold,
    WalkAway,
    QuickResolve,
    Settle,
    ReturnToListings,
    AcceptPostAuction,
    TestPostAuction,
}

impl App {
    pub(crate) fn draw_auction(&mut self) {
        let Some(auction) = self.current_auction.as_ref() else {
            self.screen = Screen::PropertyList;
            return;
        };

        let auction = auction.clone();
        let property = auction.property.clone();
        let next_bid = auction.next_bid();
        let finance = finance_snapshot(&self.player, self.market(), next_bid);
        let reserve_estimate = estimate_reserve(
            &property,
            self.market(),
            auction.player_research_level,
            self.player.reputation,
        );
        let panel_price = if auction.is_running() {
            next_bid
        } else if auction.status == Some(AuctionStatus::PassedIn) {
            post_auction_offer(&auction).unwrap_or(auction.current_bid)
        } else {
            auction.current_bid
        };
        let panel_finance = finance_snapshot(&self.player, self.market(), panel_price);
        let panel_margin = projected_purchase_margin(&property, panel_price, self.market());
        let panel_rental = rental_underwrite(&property, self.market(), panel_price);
        let mut action = None;

        let panel_h = ui_height() - 142.0;
        let left = Rect::new(28.0, 92.0, 302.0, panel_h);
        let center = Rect::new(352.0, 92.0, ui_width() - 380.0, panel_h);

        let live = auction.is_running() && auction.has_started;
        if !live && (auction.is_running() || self.auction_result_open) {
            draw_auction_property_panel(
                left,
                &auction,
                reserve_estimate,
                cash_needed_to_settle(panel_price),
                panel_finance.headroom_after,
                panel_margin,
                panel_rental.net_cashflow,
                if auction.status.is_some() {
                    "outcome"
                } else if auction.has_started {
                    "next bid"
                } else {
                    "opening bid"
                },
            );
        }
        if auction.is_running() && !auction.has_started {
            action = draw_auction_day_lobby(
                center,
                &auction,
                finance_snapshot(&self.player, self.market(), auction.player_walkaway_price),
            )
            .map(|lobby_action| match lobby_action {
                AuctionLobbyAction::Begin => AuctionUiAction::BeginAuction,
                AuctionLobbyAction::Leave => AuctionUiAction::ReturnToListings,
            });
        } else if auction.is_running() {
            action = draw_live_stage(self, &auction, finance);
        } else if let Some(status) = auction.status.clone() {
            action = if self.auction_result_open {
                self.draw_auction_result(center, &auction, status)
            } else {
                draw_hammer(self, &auction)
            };
        }

        match action {
            Some(AuctionUiAction::ReviewOutcome) => self.auction_result_open = true,
            Some(AuctionUiAction::FocusRival(index)) => {
                self.auction_focus = if self.auction_focus == Some(index) {
                    None
                } else {
                    Some(index)
                };
            }

            Some(AuctionUiAction::BeginAuction) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    begin_auction_calls(auction);
                    self.status =
                        "Bidding is live. Tap RAISE, JUMP, WAIT & READ ROOM, or WALK AWAY."
                            .to_string();
                    self.play_sound(crate::audio::SoundEffect::Button);
                }
            }
            Some(AuctionUiAction::Bid) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    self.auction_read = None;
                    place_player_bid(auction);
                    self.auction_beat =
                        Some(("Your paddle rises. The room turns to you.".to_string(), 4.0));
                    self.play_sound(crate::audio::SoundEffect::Bid);
                }
            }
            Some(AuctionUiAction::JumpBid) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    self.auction_read = None;
                    self.status = place_player_jump_bid(auction);
                    self.auction_beat =
                        Some(("A double step. You challenge the room.".to_string(), 4.0));
                    self.play_sound(crate::audio::SoundEffect::Bid);
                }
            }
            Some(AuctionUiAction::Hold) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    let read = hold_player_position(auction);
                    self.auction_read = Some((read.clone(), 6.0));
                    self.status = format!("Held position. {read}");
                }
            }
            Some(AuctionUiAction::WalkAway) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    stop_player_bidding(auction);
                    self.play_sound(crate::audio::SoundEffect::Button);
                }
            }
            Some(AuctionUiAction::QuickResolve) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    quick_resolve_auction(auction);
                    self.play_sound(crate::audio::SoundEffect::Hammer);
                }
            }
            Some(AuctionUiAction::Settle) => {
                let homes_before = self.player.properties.len();
                self.settle_purchase();
                if self.player.properties.len() > homes_before {
                    record_completed_room(&mut self.player.rival_notebook, &auction);
                }
            }
            Some(AuctionUiAction::ReturnToListings) => {
                record_completed_room(&mut self.player.rival_notebook, &auction);
                if let Some(auction) = &self.current_auction {
                    if award_discipline_reputation(&mut self.player, auction) {
                        self.status =
                            "Discipline reputation +1 for letting an overheated room win."
                                .to_string();
                    }
                }
                self.current_auction = None;
                self.screen = Screen::PropertyList;
            }
            Some(AuctionUiAction::AcceptPostAuction) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    if accept_post_auction_offer(auction) {
                        self.status =
                            "Vendor accepted. Review the numbers, then tap SETTLE PURCHASE."
                                .to_string();
                    }
                }
            }
            Some(AuctionUiAction::TestPostAuction) => {
                if let Some(auction) = self.current_auction.as_mut() {
                    match test_vendor_at_passed_in_price(auction) {
                        Some(PostAuctionTestResult::Accepted(price)) => {
                            self.status = format!(
                                "Vendor accepted your {} offer. Review, then tap SETTLE PURCHASE.",
                                format_money(price)
                            );
                        }
                        Some(PostAuctionTestResult::Rejected(counter)) => {
                            self.status = format!(
                                "Vendor rejected the test offer and holds at {}.",
                                format_money(counter)
                            );
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }

    fn draw_auction_result(
        &self,
        rect: Rect,
        auction: &Auction,
        status: AuctionStatus,
    ) -> Option<AuctionUiAction> {
        soft_panel(rect);
        match status {
            AuctionStatus::SoldToPlayer => {
                let debrief = self.purchase_debrief_for_auction(auction);
                draw_purchase_debrief(&debrief, rect);
                if button(
                    Rect::new(rect.x + 36.0, rect.y + rect.h - 64.0, rect.w - 72.0, 44.0),
                    "Settle Purchase",
                    true,
                    ButtonTone::Primary,
                ) {
                    return Some(AuctionUiAction::Settle);
                }
            }
            AuctionStatus::SoldToNpc(name) => {
                let walked_away = auction.player_exit_bid.is_some();
                label(
                    if walked_away {
                        "Walk-away Held"
                    } else {
                        "Outbid At The Hammer"
                    },
                    rect.x + 26.0,
                    rect.y + 46.0,
                    30,
                    TEXT_BRIGHT,
                );
                draw_wrapped_text(
                    &if walked_away {
                        format!(
                            "{name} bought it for {}. Your cash stayed out of a hotter deal.",
                            format_money(auction.current_bid)
                        )
                    } else {
                        format!(
                            "{name} held the final bid at {}. You kept the paddle down when the last call came.",
                            format_money(auction.current_bid)
                        )
                    },
                    rect.x + 26.0,
                    rect.y + 92.0,
                    rect.w - 52.0,
                    20,
                    TEXT,
                );
                if earned_discipline_reputation(auction) {
                    label(
                        "+1 discipline reputation when you return",
                        rect.x + 26.0,
                        rect.y + 158.0,
                        18,
                        POSITIVE,
                    );
                }
                if button(
                    Rect::new(rect.x + 36.0, rect.y + rect.h - 64.0, rect.w - 72.0, 44.0),
                    "Return To Listings",
                    true,
                    ButtonTone::Secondary,
                ) {
                    return Some(AuctionUiAction::ReturnToListings);
                }
            }
            AuctionStatus::PassedIn => {
                label(
                    "Passed In — Negotiation",
                    rect.x + 26.0,
                    rect.y + 46.0,
                    30,
                    TEXT_BRIGHT,
                );
                draw_wrapped_text(
                    "The public auction missed reserve. The vendor's agent will now name a private counteroffer; you may still leave.",
                    rect.x + 26.0,
                    rect.y + 92.0,
                    rect.w - 52.0,
                    20,
                    TEXT,
                );
                let offer = post_auction_offer(auction).unwrap_or(auction.reserve_price);
                let offer_finance = finance_snapshot(&self.player, self.market(), offer);
                let test_finance =
                    finance_snapshot(&self.player, self.market(), auction.current_bid);
                label(
                    "Vendor counteroffer",
                    rect.x + 26.0,
                    rect.y + 166.0,
                    17,
                    TEXT_DIM,
                );
                label(
                    &format_money(offer),
                    rect.x + 26.0,
                    rect.y + 208.0,
                    42,
                    if offer <= auction.player_walkaway_price {
                        POSITIVE
                    } else {
                        WARNING
                    },
                );
                let walkaway_relation = if offer <= auction.player_walkaway_price {
                    "below"
                } else {
                    "above"
                };
                label(
                    &format!(
                        "{} {walkaway_relation} walk-away | cash after settle {}",
                        format_money((offer - auction.player_walkaway_price).abs()),
                        format_money(offer_finance.cash_after_settle)
                    ),
                    rect.x + 28.0,
                    rect.y + 240.0,
                    16,
                    TEXT_DIM,
                );
                label_fit(
                    if auction.post_auction_tested {
                        "Your lower offer was rejected. The vendor counter still stands."
                    } else {
                        "Test the vendor at the passed-in price, or meet their counter now."
                    },
                    rect.x + 28.0,
                    rect.y + 270.0,
                    rect.w - 56.0,
                    15,
                    if auction.post_auction_tested {
                        WARNING
                    } else {
                        crate::ui::BLUE
                    },
                );
                label_fit(
                    if auction.player_research_level >= crate::model::ResearchLevel::FullDiligence {
                        vendor_stance(auction).label()
                    } else {
                        "Seller flexibility is unclear without full diligence."
                    },
                    rect.x + 28.0,
                    rect.y + 300.0,
                    rect.w - 56.0,
                    15,
                    if auction.player_research_level >= crate::model::ResearchLevel::FullDiligence {
                        POSITIVE
                    } else {
                        TEXT_DIM
                    },
                );
                if button(
                    Rect::new(rect.x + 26.0, rect.y + rect.h - 64.0, 148.0, 44.0),
                    &if auction.post_auction_tested {
                        "OFFER REJECTED".to_string()
                    } else {
                        format!("OFFER {}", format_money(auction.current_bid))
                    },
                    !auction.post_auction_tested && test_finance.can_buy,
                    ButtonTone::Secondary,
                ) {
                    return Some(AuctionUiAction::TestPostAuction);
                }
                if button(
                    Rect::new(rect.x + 184.0, rect.y + rect.h - 64.0, 166.0, 44.0),
                    &format!("MEET {}", format_money(offer)),
                    offer_finance.can_buy,
                    if offer <= auction.player_walkaway_price {
                        ButtonTone::Primary
                    } else {
                        ButtonTone::Danger
                    },
                ) {
                    return Some(AuctionUiAction::AcceptPostAuction);
                }
                if button(
                    Rect::new(rect.x + rect.w - 150.0, rect.y + rect.h - 64.0, 124.0, 44.0),
                    "LEAVE",
                    true,
                    ButtonTone::Ghost,
                ) {
                    return Some(AuctionUiAction::ReturnToListings);
                }
            }
        }
        None
    }
}
