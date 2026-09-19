//! Presentation-only call cadence and a stable, bounded bid-pressure scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallPhase {
    Opening,
    Bidding,
    Slowing,
    FinalCalls,
    Once,
    Twice,
}
impl CallPhase {
    pub fn for_room(seconds: f32, has_bid: bool) -> Self {
        if seconds <= 3.0 {
            Self::Twice
        } else if seconds <= 6.0 {
            Self::Once
        } else if seconds <= 10.0 {
            Self::FinalCalls
        } else if seconds <= 18.0 {
            Self::Slowing
        } else if has_bid {
            Self::Bidding
        } else {
            Self::Opening
        }
    }
    pub fn urgent(self) -> bool {
        matches!(self, Self::Once | Self::Twice)
    }
    fn index(self) -> usize {
        match self {
            Self::Opening => 0,
            Self::Bidding => 1,
            Self::Slowing => 2,
            Self::FinalCalls => 3,
            Self::Once => 4,
            Self::Twice => 5,
        }
    }
    pub fn label(self) -> &'static str {
        &voice().phases[self.index()]
    }
    pub fn call(self, seconds: f32, bid: i64, next: i64) -> String {
        let lines = &voice().calls[self.index()];
        let line = &lines[(seconds.max(0.0) / 4.0) as usize % lines.len()];
        line.replace("{bid}", &crate::ui::format_money(bid))
            .replace("{next}", &crate::ui::format_money(next))
    }
}

pub struct BidPressure {
    pub low: i64,
    pub high: i64,
}
impl BidPressure {
    pub fn new(guide: i64, estimate: i64, limit: i64, bid: i64) -> Self {
        let low = guide.min(estimate).min(limit).min(bid).max(0) / 2;
        let ceiling = guide.max(estimate).max(limit).max(bid).max(1);
        Self {
            low,
            high: ceiling
                .saturating_add(ceiling / 8)
                .max(low.saturating_add(1)),
        }
    }
    pub fn position(&self, price: i64) -> f32 {
        ((price as f64 - self.low as f64) / (self.high as f64 - self.low as f64)).clamp(0.0, 1.0)
            as f32
    }
}

#[derive(serde::Deserialize)]
struct AuctionVoice {
    phases: [String; 6],
    calls: [Vec<String>; 6],
}
fn voice() -> &'static AuctionVoice {
    static VOICE: std::sync::OnceLock<AuctionVoice> = std::sync::OnceLock::new();
    VOICE.get_or_init(|| {
        let data: AuctionVoice =
            macroquad_toolkit::include_json!("../../assets/auction_voice.json")
                .expect("auction voice must be valid JSON");
        assert!(
            data.phases.iter().all(|label| !label.is_empty())
                && data
                    .calls
                    .iter()
                    .all(|calls| !calls.is_empty() && calls.iter().all(|line| !line.is_empty())),
            "auction voice needs labels and calls for every phase"
        );
        data
    })
}
