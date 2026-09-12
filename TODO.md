# Remaining work

## Documentation-alignment improvements

- Migrate the legacy `src/**/tests.rs` suites into the crate-level `tests/` directory and expose only intentional public seams; consolidate related cases toward the five-case-per-major-feature target while preserving high-value regression coverage.
- Refactor screen rendering and input to return `UiAction` or intent values, then centralize state mutation in an `App` dispatcher. Current `draw_*` methods and widgets mutate screen, status, selection, and player state directly.
- Move hardcoded player-facing copy and balance/configuration constants into typed JSON under `assets/`, load them through the toolkit, and add project-owned semantic validation. This includes campaign, finance, valuation, auction, rental, tutorial, and status text.
- Split oversized files and functions to match §§2.2 and 4.1: `src/app.rs` (752 lines), `src/screens/portfolio_widgets.rs` (675), `src/sim/auction_sim.rs` (627), and `src/screens/auction.rs` (613), plus the large drawing functions in `src/ui/mod.rs` and the screen modules.
- Remove or narrow the crate-level `clippy::too_many_arguments` allowance in `src/lib.rs`; refactor affected calls and document any remaining intentional `#[allow]`.

## Release gates

- Create or confirm the itch.io project for `webhatchery/auction-house-tycoon`, then rerun `publish-itch.ps1 -Status` until the configured `html5-demo` and `windows` channels are valid.
- Complete and record a full new-game campaign win and week-24 failure on both browser and Windows release builds, including the research, auction, pass-in, repair or renovation, leasing, rent review, refinance or sale, save/load, and final-ledger actions. The current record covers browser smoke, deterministic replay, and native capture states.
- After those gates pass, change `game_page.json` from Playable to Released and run the final publisher checks.
