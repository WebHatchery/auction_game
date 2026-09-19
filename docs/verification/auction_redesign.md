# Auction UI redesign verification

Validated 19 September 2026.

The live auction uses a wide, borderless bidding stage with the current price,
current call, transient rival counters/exits, and four distinct visible actions.
Only the final six seconds expose the precise clock. Research notes are optional
and do not pause bidding. Room reads remain briefly visible as past observations.

Listings retain the existing property artwork and pixel typography, with one
opportunity, one risk, yield and an Inspect action per property. Registration
scarcity and research costs are stated above the listings. Navigation and the
briefing introduce Scout / Research / Register / Auction / Outcome / Recover.

Validation:
- All 75 existing tests passed, including deterministic replay and source limits.
- Strict Clippy passed for all targets and features; formatting is clean.
- Windows and WebGL release builds and default Preview deployment passed.
- Project Roost tracking could not connect to 127.0.0.1:80; deployment succeeded.
- Refreshed 12 scene captures, including listings, notes, room reads, limit
  warning, urgent calls, registration, purchase, walk-away and negotiation.
- Inspected the live room at 1024x576 with actual simulated rival counterbids;
  the price, event beat, full bidder names and all action controls fit.
- Visual checks use the native capture harness. No new interactive browser
  click-through was performed for this redesign.

Screenshots are stored directly in this directory, replacing prior captures of
matching states. The original release_playthrough.md records older release gates.
