# Remaining work

## UI_STYLE review — 20 September 2026

The composition, research disclosure, portfolio recovery, auction finance,
dashboard, risk knowledge, and outcome-report implementation pass is complete.
Fresh visual evidence is recorded in `docs/verification/ui_review_20260920.md`.
The active list below is limited to verification that could not be completed
without an interactive browser or physical touch surface.

### Remaining interactive verification

- [ ] **UI-06 — Confirm transient feedback and reopenable help in a live run.**
  Exercise new-game → Inspect → research → register, repeat without prompts,
  reopen Help and dismiss it with touch, and exercise save/load success,
  missing/malformed saves, settings failure, and feedback after a paused
  auction. The implementation now expires routine feedback, retains critical
  errors, clears stale navigation context, and provides contextual Help; only
  live touch/menu confirmation remains.

- [ ] **UI-V1 — Tap through the dense portfolio and collection paths.**
  The legal dense portfolio now has an eight-home capture and five-item pages
  with visible previous/next controls. Confirm page switching, selected-home
  recovery actions, long holding labels, maintenance and rent-review reachability,
  and the listing collection at the declared sizes using touch-only input.

- [ ] **UI-V2 — Complete browser, resize, and touch interaction review.**
  Replace or extend the fresh captures when the interactive pass is available.
  Exercise touch-only new game, research tiers, lobby leave/start,
  RAISE/JUMP/WAIT, rival open/close, Walk Away/quick resolve, settlement,
  vendor test/counteroffer/leave, portfolio recovery, and settings Apply/Cancel.
  Check the embedded canvas and non-16:9 resize, enlarged text, paused-auction
  errors, pass-in negotiation, save/load, and every report dismissal. Record
  any physical-touch limitations honestly; the current evidence covers 1280×720,
  1024×576, and the historical 1008×537 drawable area but not a physical
  touch device or live browser session.

## Documentation-alignment improvements

- Migrate the legacy `src/**/tests.rs` suites into the crate-level `tests/` directory and expose only intentional public seams; consolidate related cases toward the five-case-per-major-feature target while preserving high-value regression coverage.
- Refactor screen rendering and input to return `UiAction` or intent values, then centralize state mutation in an `App` dispatcher. Current `draw_*` methods and widgets mutate screen, status, selection, and player state directly.
- Move hardcoded player-facing copy and balance/configuration constants into typed JSON under `assets/`, load them through the toolkit, and add project-owned semantic validation. This includes campaign, finance, valuation, auction, rental, tutorial, and status text.
- Plan cohesive splits for large files and functions under §§2.2 and 4.1: current `src/app.rs` (717 physical lines), `src/screens/portfolio_widgets.rs` (675), and `src/sim/auction_sim.rs` (662), plus large drawing functions in `src/ui/mod.rs` and screen modules. No current source file exceeds 800 lines; do not describe these as hard-limit violations. `src/screens/auction.rs` has already been split into focused screen modules. Keep every resulting file below 800 lines, use named module files when restructuring, and coordinate UI extraction with the review tasks above rather than duplicating that work.
- Remove or narrow the crate-level `clippy::too_many_arguments` allowance in `src/lib.rs`; refactor affected calls and document any remaining intentional `#[allow]`.

## Release gates

- Create or confirm the itch.io project for `webhatchery/auction-house-tycoon`, then rerun `publish-itch.ps1 -Status` until the configured `html5-demo` and `windows` channels are valid.
- Complete and record a full new-game campaign win and week-24 failure on both browser and Windows release builds, including the research, auction, pass-in, repair or renovation, leasing, rent review, refinance or sale, save/load, and final-ledger actions. The current record covers browser smoke, deterministic replay, and native capture states.
- After those gates pass, change `game_page.json` from Playable to Released and run the final publisher checks.
