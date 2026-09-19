//! Observable bidder decisions and calls; no extra random presentation state.
use crate::model::{Auction, Bidder, BidderActor, BidderMood};
use crate::ui::auction_view::{reaction_text, CallPhase};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reaction {
    Watching,
    Calculating,
    Hesitant,
    Frustrated,
    Aggressive,
    Preparing,
    NearLimit,
    Withdrawn,
}

impl Reaction {
    pub fn for_bidder(bidder: &Bidder, next_bid: i64, increment: i64) -> Self {
        if !bidder.active {
            Self::Withdrawn
        } else if bidder.preparing_bid {
            Self::Preparing
        } else if bidder.mood == BidderMood::Stretching {
            if bidder.bid_flash > 0.0 {
                Self::Aggressive
            } else {
                Self::Frustrated
            }
        } else if bidder.mood == BidderMood::Hesitating {
            if next_bid >= bidder.max_price - increment {
                Self::NearLimit
            } else {
                Self::Hesitant
            }
        } else if bidder.bid_flash > 0.0 {
            Self::Aggressive
        } else if bidder.reaction_timer <= 1.0 || bidder.mood == BidderMood::Interested {
            Self::Calculating
        } else {
            Self::Watching
        }
    }

    pub fn label(self) -> &'static str {
        reaction_text(match self {
            Self::Watching => "watching",
            Self::Calculating => "calculating",
            Self::Hesitant => "hesitant",
            Self::Frustrated => "frustrated",
            Self::Aggressive => "aggressive",
            Self::Preparing => "preparing",
            Self::NearLimit => "near_limit",
            Self::Withdrawn => "withdrawn",
        })
    }
}

pub fn behavioral_event(before: &[Bidder], auction: &Auction) -> Option<String> {
    // A withdrawal takes precedence over a new preparation in the same tick.
    for key in ["withdrew", "bid", "hesitated", "prepared"] {
        for (previous, bidder) in before.iter().zip(&auction.bidders) {
            let changed = match key {
                "withdrew" => previous.active && !bidder.active,
                "bid" => bidder.bid_count > previous.bid_count,
                "hesitated" => {
                    bidder.active
                        && bidder.mood == BidderMood::Hesitating
                        && previous.mood != BidderMood::Hesitating
                }
                _ => !previous.preparing_bid && bidder.preparing_bid,
            };
            if changed {
                let text_key = if key == "bid"
                    && bidder.mood != BidderMood::Hesitating
                    && previous.mood != BidderMood::Hesitating
                {
                    "quick_bid"
                } else {
                    key
                };
                return Some(reaction_text(text_key).replace("{name}", &bidder.name));
            }
        }
    }
    None
}

pub fn live_call(auction: &Auction) -> String {
    let phase = CallPhase::for_room(auction.seconds_remaining, auction.last_bidder.is_some());
    let key = if phase == CallPhase::Twice {
        "twice"
    } else if phase == CallPhase::Once {
        "once"
    } else if auction.last_bidder == Some(BidderActor::Vendor) && auction.seconds_since_bid < 2.0 {
        "vendor"
    } else if auction.last_bidder.is_none() {
        "opening"
    } else if auction.seconds_since_bid < 2.5 {
        "price_call"
    } else if auction.on_market_announced
        && (auction.seconds_since_bid >= 5.0 || phase == CallPhase::FinalCalls)
    {
        "selling"
    } else {
        "advance"
    };
    reaction_text(key)
        .replace("{bid}", &crate::ui::format_money(auction.current_bid))
        .replace("{next}", &crate::ui::format_money(auction.next_bid()))
}
