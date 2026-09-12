# Release validation record

Validated for the Auction House Tycoon 0.1.0 release candidate on 12 September 2026.

## Automated release gates

- `cargo clippy --all-targets --all-features -- -D warnings` passed.
- `cargo test --all-targets --all-features` passed: 72 library tests, 2 campaign replay tests, and 1 source-standards test.
- `publish.ps1` passed for the Windows and WebGL release artifacts.
- `publish-itch.ps1 -DryRun` passed for the `html5-demo` and `windows` channels.

## Browser smoke path

The self-contained WebGL package was exercised from a new campaign: briefing, week-one listings, full diligence, bidder registration, live auction calls, an assertive bid, walk-away, quick resolution, return to listings, save, load, settings page 1, settings page 2, text-size adjustment, Apply, and browser reload persistence.

The authored winning route and deterministic live-auction resume are covered by `tests/campaign_replay.rs`. The visual harness also verifies repair, renovation, leasing, rent review, refinance, paydown, sale, final ledger, and week-24 failure states.

## Visual coverage

The capture harness refreshed the verification states at 1200×675 and checked the title/settings, dashboard, auction, portfolio, and failure screens at 1024×576 and 1600×900. Required controls remained visible, readable, and touch-sized. Missing quicksave feedback is captured in `ui_title_load_missing.png`; malformed quicksave feedback is covered by the save tests.

## Remaining gates

The itch target `webhatchery/auction-house-tycoon` is configured, but the remote project does not yet exist: `publish-itch.ps1 -Status` returns itch.io API error 400 (`invalid game`). Creating that itch project is an account-side release action outside this workspace.

The capture harness covered the native Windows build path and all authored states, but a full manual Windows click-through still needs to be performed on a desktop interaction surface. The browser smoke path and deterministic campaign replay are complete; the full new-game win/failure route should be repeated interactively on both release platforms before changing the catalog status to Released.
