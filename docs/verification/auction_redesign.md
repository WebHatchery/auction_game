# Major auction-room redesign verification

Validated 19 September 2026. Supersedes the earlier border-removal pass.

The live composition is a pixel-art auction room, with an auctioneer and gavel,
seated rivals, expressive portraits, a coherent central price/call stage and a
physical bidding desk. The side catalogue contains financial headroom and a
subordinate outcome estimate. Detailed finance is optional. The pressure rail
positions price, researched value range and the player's walk-away threshold.

The room changes with play: leaders raise paddles, withdrawn rivals fade,
price changes briefly enlarge the price, the spotlight narrows in final calls,
and the auctioneer raises the gavel. The numeric clock appears only in the
last six seconds. Late bids reopen the cadence using existing overtime rules.
After the hammer, the room holds the result until the player opens its review.

Validation:
- All 80 tests pass: 75 existing regression/replay/source checks and five new
  presentation cases covering call phases, overtime presentation, authored
  voice templates, price ordering and extreme-value pressure scale safety.
- Strict Clippy passes for all targets/features; formatting and diff checks pass.
- Every Rust file remains within the 800 physical-line limit.
- Default publish.ps1 builds Windows and WebGL and deploys Preview successfully.
- Project Roost tracking cannot connect to 127.0.0.1:80; deployment succeeds.
- Native captures cover opening, counterbids, research notes, rival study,
  room reads, limit warnings, final calls, withdrawal and hammer outcomes.
  The room was also inspected at 1024x576, including reopened late calls.
- Browser mouse playthrough exercised new campaign, listings, inspection,
  registration, starting calls, Raise, Wait, Assert, rival study, optional notes,
  Walk Away, quick resolution, the hammer result and outcome review.
- Rechecked the final pointer implementation and assertion feedback in-browser.
  A previous observation now clears on the player's own Raise/Assert.
- Walk Away sits beside Menu, clear of the host page's fixed footer overlays.
- Buttons and custom controls use macroquad-toolkit's touch-aware Pointer and
  act on release. Physical touchscreen hardware was not available for testing.

Existing deterministic auction rules, finance rules and save format remain
unchanged. Transient presentation state resets when loading a saved game.
Screenshots live directly in this directory, replacing matching prior states.
