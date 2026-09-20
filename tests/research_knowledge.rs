use auction_game::model::{Condition, DealArchetype, Property, ResearchLevel};
use auction_game::sim::research::{known_risk_level, KnownRisk};

fn sample_property(id: usize, hidden_defect_risk: f32) -> Property {
    Property {
        id,
        address: "1 Test Lane".to_string(),
        suburb: "Testville".to_string(),
        bedrooms: 3,
        bathrooms: 1,
        condition: Condition::Rough,
        land_size: 600,
        market_value: 500_000,
        guide_price: 450_000,
        reserve_price: 470_000,
        appeal: 60,
        renovation_potential: 40,
        hidden_defect_risk,
        holding_cost_per_week: 120,
        buyer_demand: 60,
        deal_archetype: DealArchetype::RiskyFixer,
        thesis: "Test thesis".to_string(),
        main_risk: "Test risk".to_string(),
        best_strategy: "Test strategy".to_string(),
        bad_strategy: "Test mistake".to_string(),
        notes: "Test notes".to_string(),
    }
}

#[test]
fn street_scan_never_claims_to_know_hidden_risk() {
    let property = sample_property(1, 0.95);
    assert_eq!(
        known_risk_level(&property, ResearchLevel::StreetScan),
        KnownRisk::Unverified
    );
}

#[test]
fn agent_pack_exposes_elevated_risk_at_the_high_threshold() {
    let property = sample_property(1, 0.25);
    assert_eq!(
        known_risk_level(&property, ResearchLevel::AgentPack),
        KnownRisk::Elevated
    );
}

#[test]
fn agent_pack_keeps_middle_risk_distinct() {
    let property = sample_property(1, 0.18);
    assert_eq!(
        known_risk_level(&property, ResearchLevel::AgentPack),
        KnownRisk::Moderate
    );
}

#[test]
fn agent_pack_can_earn_a_low_risk_label() {
    let property = sample_property(1, 0.08);
    assert_eq!(
        known_risk_level(&property, ResearchLevel::AgentPack),
        KnownRisk::Low
    );
}

#[test]
fn inspection_uses_material_defect_evidence_instead_of_the_raw_score() {
    let likely_defect = sample_property(1, 0.18);
    let clean_property = sample_property(2, 0.18);
    assert_eq!(
        known_risk_level(&likely_defect, ResearchLevel::BuildingInspection),
        KnownRisk::Elevated
    );
    assert_eq!(
        known_risk_level(&clean_property, ResearchLevel::BuildingInspection),
        KnownRisk::Low
    );
}
