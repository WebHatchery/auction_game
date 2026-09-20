# Remaining work

## UI_STYLE review — 20 September 2026

Audit/planning only. Implement the tasks below in dependency order; do not treat
this review as a completed UI change. References are project-relative.

### Evidence and scope

- Read `AGENTS.md`, `UI_STYLE.md`, `CODE_STANDARDS.md`,
  `GAME_DEVELOPMENT_GUIDE.md`, `README.md`, and `GAME_DESIGN.md`. No
  `PROJECT_AGENTS.md` was found in this project. Inspected screen rendering,
  navigation, auction feedback, the fixed camera/pointer mapping, and capture code.
- Visually inspected existing `docs/verification/` images: `ui_dashboard.png`,
  `ui_detail.png`, `ui_detail_full.png`, `ui_portfolio.png`,
  `ui_portfolio_review.png`, `ui_sale_result.png`, `ui_conclusion_failed.png`
  (each 1184×636); `ui_listings.png`, `ui_briefing.png`,
  `ui_auction_limit.png`, `ui_auction_lobby.png`, `ui_passed_in.png`
  (each 1264×681); and `ui_auction_small.png` (1008×537).
  These are actual PNG dimensions, not the requested window sizes in prior reports.
- Images are historical evidence, not fresh verification of HEAD. Several show
  old Dashboard/Listings labels; the passed-in capture still shows PROPERTY NOTES
  and a finance sidebar removed from the current live stage. Findings below use
  current source to distinguish surviving problems from already completed work.
- Preserve the completed auction improvements recorded in
  `docs/verification/auction_redesign.md`: dominant room/price/RAISE, quieter WAIT,
  separate WALK AWAY and Menu, contextual rival inspection, timed reactions,
  and the simplified price rail. Do not restore the old permanent finance column.
  Existing release history remains in `docs/verification/release_playthrough.md`.
- No literal template-demo copy or live gameplay/menu action grouping defect was
  established. The remaining dashboard/card and permanent-help patterns should
  be adapted through the composition tasks, not labelled proven template leftovers.
  Preserve the current utility/menu separation throughout those changes.
- No live run, fresh capture, browser interaction, physical touch test, or publish
  was performed for this documentation-only audit. The existing screenshots and
  source suffice to plan the verified work; runtime uncertainties are separated below.
- Shared verification baseline for every implementation task: normal **1280×720
  canvas**, minimum candidate **1024×576 canvas**, plus the historically captured
  **1008×537 drawable area**. README/GDD currently declare neither supported size;
  confirm the contract in UI-01. Test actual canvas dimensions, not window chrome.
  Use mouse and touch-only paths, default and largest supported text setting,
  long addresses and large monetary values. Replace equivalent screenshots directly
  in `docs/verification/`; record actual sizes and interactions. After meaningful
  implementation changes run `./publish.ps1` without parameters and report blockers.

### Verified findings — composition and decision safety first

- [ ] **UI-01 — Define screen decisions and a responsive layout contract.**
  **Scope:** `README.md`/`GAME_DESIGN.md`; `src/ui/mod.rs` (`begin_ui_frame`,
  `ui_pointer`, `button`); `src/app.rs::draw_settings_editor`; `src/main.rs`.
  **Observed:** every screen is stretched through a fixed 1200×675 camera with
  independent X/Y pointer scaling and no responsive reflow. Research buttons are
  30 logical pixels high, strategy buttons 28, filters 32; at a 576-pixel canvas
  height these become approximately 26, 24, and 27 pixels. The source establishes
  undersized geometry, not an observed missed-tap rate. README/GDD describe the
  loop but omit the required per-screen decision briefs and viewport contract.
  **Change:** document dominant focus, action/constraint, supporting/deferred
  information and touch flow for preparation, auction, recovery and outcomes.
  Establish shared wide/compact layout metrics using toolkit viewport/layout and
  pointer helpers; use the same conversion for game controls and settings. Reflow
  or disclose secondary content before shrinking it. Preserve the auction room's
  useful framing; this game does not need an invented map or zoom feature.
  **Acceptance:** each screen has at most 2–3 strongly competing regions; primary
  decisions fill the useful area; required targets are at least 44×44 displayed
  pixels at the declared minimum, with readable labels and matching hit regions.
  **Verify:** baseline sizes above, resize between them, non-16:9 embedding and
  display scaling; tap research, strategy, filters, rival inspection, Menu and
  settings controls at their edges. Record behavior below the supported minimum.
  This is the layout prerequisite for UI-02 through UI-08, not a decorative pass.

- [ ] **UI-02 — Recompose property research around one actionable bid plan.**
  **Scope:** property detail at Street Scan and Full Diligence;
  `src/screens/property_detail.rs` (`draw_detail_summary`, `draw_walkaway_panel`).
  **Observed:** the historical detail images show six badges, nested thesis and
  diligence boxes, ellipsized risk/trap/research conclusions, and the walk-away
  and margin repeated in the summary and bottom strip. Current code retains this
  structure, truncates the full takeaway into one line, and puts an unbounded
  cash/margin/rent/bank sentence beside the registration controls. Reading the
  evidence needed to buy safely is harder than finding the orange Register button.
  **Change:** make a single selected-limit/underwriting decision area dominant,
  with research evidence as its support. Keep guide versus researched range and
  recommended versus manually chosen cap distinctly labelled; remove redundant
  condition/confidence badges and duplicate margin displays. Replace nested
  thesis/diligence cards with a readable summary and visible expandable report.
  Show essential defect warnings in full; reveal comparisons and long rationale
  on tap with a visible close/back action. Resize the image and reflow costs to
  fit this hierarchy rather than shrinking text. Keep research prices, settlement
  cash, net rent, bank constraints and registration availability beside their actions.
  **Acceptance:** the player can explain the selected cap and its risk before
  registering, read every purchased finding without guessing an ellipsis, and
  distinguish a recommendation from a manually adjusted price.
  **Verify:** UI-01 sizes; tap each research tier, all strategies, +/-10k and
  Register/Recover; cover unaffordable research, zero registrations, full reports,
  long notes, and compact/premium/large-block properties. No report overlays an
  active purchase control or loses its touch dismissal.

- [ ] **UI-03 — Make the selected holding's next decision dominate Portfolio.**
  **Scope:** vacant, leased, maintenance, works, leasing and review-due states;
  `src/screens/portfolio.rs`, `portfolio_widgets.rs`,
  `portfolio_finance_widgets.rs`, `portfolio_sale_widgets.rs`,
  `portfolio_rent_review.rs`.
  **Observed:** portfolio/review screenshots and current rendering simultaneously
  expose property selectors, loan actions, valuation/position stats, diagnosis,
  contractor and marketing selectors, and three bottom action cards. Contractor
  choices remain visible on a leased home and during rent review; sale actions
  compete with a review that blocks week advancement. Diagnosis and finance
  explanations truncate while the repeated selected address/rent consumes space.
  **Change:** keep a quiet holding selector, one selected-property summary and one
  decision workspace. Promote required maintenance/review when due; otherwise
  present the relevant lease/hold/improve choice. Put finance and sale behind
  explicit labelled disclosures, with contractor choices inside an eligible
  renovation and marketing/reserve choices inside sale preparation. Remove
  redundant diagnosis boxes and historical purchase/upgrade prose from ordinary
  play; keep history and full cashflow breakdown retrievable. Keep sale/paydown
  accessible as recovery routes even when another action is blocked.
  **Acceptance:** 2–3 attention regions; the player sees why progress is blocked
  and the exact next action. Opening a financial or sale decision reveals its
  fee, debt/interest change, vacancy/turnover cost or loss risk before execution.
  No irrelevant contractor controls compete with a tenant review.
  **Verify:** baseline sizes, every listed state, switching holdings, opening and
  closing finance/sale, insufficient cash, and a review due on a different home.
  Tap through repair → lease → advance → review and verify the week blocker routes
  to the affected holding. Preserve equal emphasis for genuinely equal choices.

- [ ] **UI-04 — Keep bid-specific financial warnings beside RAISE and JUMP.**
  **Scope:** live auction and bidder-terms lobby;
  `src/screens/auction_console.rs::draw_console`, `auction_context.rs::draw_context`,
  `auction_stage.rs`, `auction.rs::draw_auction`, `auction_lobby.rs`,
  `auction_property_panel.rs`; `src/sim/finance.rs::finance_snapshot`.
  **Observed (current code):** live controls check `can_buy` but never display
  `cash_after_settle`, `cash_buffer_target` or `stress`; only outright inability
  gets a generic finance-limit message. The margin appears only at the walk-away
  threshold, and selecting a rival replaces that context. A legal bid can breach
  the resilience buffer before reaching the personal cap. JUMP can be disabled
  for affordability while RAISE remains enabled, with no explanation. The lobby's
  side ledger uses the next bid while its main ledger uses the cap without equally
  explicit labels, as also seen in the historical lobby screenshot.
  **Change:** integrate a compact textual warning into the bidding desk when
  the executable action breaches cash-buffer/bank constraints or the cap; include
  cash remaining versus buffer and an actionable disabled reason. Evaluate JUMP
  at its own price. Keep warnings visible during rival inspection. Provide a
  discoverable contextual financial breakdown for net rental cashflow and fees;
  do not bring back a permanent finance panel. In the lobby choose one default
  pricing basis or explicitly label opening-bid versus cap forecasts and remove
  duplicated cap rows. Update stale README PROPERTY NOTES claims to the result.
  **Acceptance:** players can tell affordable-but-tight from unaffordable before
  tapping either bid action, while the room/price and RAISE remain dominant.
  Warnings survive temporary reactions and do not depend on color alone.
  **Verify:** baseline sizes; healthy, tight cash below cap, low bank headroom,
  maxed finance, raise-affordable/jump-unaffordable, player-leading and final-call
  states. Tap a rival and Back to Lot while a warning is active; keep Walk Away
  and Menu visually separate. Preserve existing unconditional/deposit disclosures.

- [ ] **UI-05 — Turn Recover into a weekly decision, not a permanent dashboard.**
  **Scope:** initial dashboard, closed weekly statement and shortfall states;
  `src/screens/dashboard.rs` (`draw_dashboard`, `draw_dashboard_stats`,
  `draw_market_pulse`, `draw_weekly_statement`, `dashboard_property_card`).
  **Observed:** the dashboard image/source distribute emphasis among four stat
  tiles, two report panels, unlock strip, career counters and three highlighted
  property actions. The initial empty statement permanently explains Advance Week;
  the first property receives a selection-like border solely because it is first.
  Market strategy text is truncated. The current decision has no dominant area.
  **Change:** make the next weekly choice (scout, address an urgent holding, or
  advance) the focus with a compact capital/goal summary as support. Collapse an
  absent statement; after a week closes show a concise dated result with a visible
  route to the complete ledger. Keep debt-funded shortfall and recovery routes
  conspicuous until addressed. Move career totals and unlock explanations into
  optional progress/history; remove redundant THIS WEEK/NEXT framing and arbitrary
  first-card emphasis. Keep opportunity comparison coherent, not scattered remnants.
  **Acceptance:** the next action is apparent at a glance, essential funds/goals
  remain accessible, and the full market advice/statement can be read on demand.
  **Verify:** baseline sizes; first week, profitable/negative statement, debt
  shortfall, zero buying power, no listings and due review. Tap through the
  suggested route and back; verify utilities never share a gameplay-action group.

- [ ] **UI-06 — Separate persistent state, action feedback and reopenable teaching.**
  **Scope:** non-auction screens, menu errors and first-use help;
  `src/app.rs` (`status`, action handlers, `draw`),
  `src/app/navigation.rs::draw_status_bar`, `src/screens/title.rs`,
  `briefing.rs`, `esc_menu.rs`, `property_list.rs` and portfolio action handlers.
  **Observed:** one unbounded, persistent status string carries tutorial prose,
  action outcomes and errors until overwritten. Historical detail images even
  carry a prior briefing instruction; current navigation does not clear status.
  The live auction suppresses the status bar, so menu save/load feedback routed
  only through `status` has no visible live-room home. Briefing says “assert” and
  “Tap ADVERTISE FOR RENT”, while controls say JUMP and List For Rent. Menu has
  no help reentry. The title renders a permanent Desk note even without an error.
  **Change:** expire routine success feedback near its action; retain critical
  errors inline in the owning menu/dialog with recovery/dismissal. Store important
  financial outcomes in retrievable history. Replace permanent tutorial footers
  with first-use, dismissible prompts naming exact controls, and add a visible
  Help route to reopen them. Clear stale context on navigation; omit the idle
  title Desk note. Reclaim footer space in the screen compositions above.
  **Acceptance:** ordinary play shows current state, completed prompts disappear,
  past success cannot masquerade as current advice, and failed saves remain
  understandable from any screen, including a paused auction.
  **Verify:** baseline sizes; new game → Inspect → research → register, then
  repeat without prompts; reopen Help and dismiss using touch. Exercise menu
  save/load success, missing/malformed save and settings failure paths; verify
  feedback after navigation and after its timer expires. Runtime behavior of
  those menu/error paths still needs interactive confirmation.

- [ ] **UI-07 — Present researched confidence instead of exposing hidden risk.**
  **Scope:** Scout filters/cards, dashboard opportunities and detail badges;
  `src/screens/property_list.rs` (`listing_matches`, `risk_badge`, `risk_color`),
  `dashboard.rs::verdict_color`, `property_detail.rs` risk helpers;
  `src/sim/research.rs` and `src/model/research.rs`.
  **Observed (source, with visible labels in captures):** Low Risk filtering and
  risk labels/colors directly read `hidden_defect_risk` irrespective of research
  level. Full Diligence narrows other information, but these early labels already
  classify hidden risk and weaken the uncertainty/research decision.
  **Change:** use a shared player-known assessment for risk filters, labels and
  colors; distinguish public condition, unverified risk and researched findings.
  Keep unknown listings discoverable instead of silently treating them as safe
  or excluding them without explanation. Disclose advanced metrics through
  inspection, retaining guide price and clearly labelled estimated yield for
  comparison. Do not remove earned actionable warnings to simplify a card.
  **Acceptance:** an unresearched home does not receive a definitive hidden-risk
  verdict; research visibly explains what became known and filters use the same
  knowledge as the card. This task follows the research hierarchy in UI-02.
  **Verify:** baseline sizes; inspect the same risky and low-risk property at
  every research level, toggle Low Risk/All, and save/reload earned knowledge.
  Add focused knowledge-boundary regression coverage in `tests/` if logic changes.

- [ ] **UI-08 — Make result explanations fully readable without adding more panels.**
  **Scope:** sale result and campaign conclusion;
  `src/screens/sale_result.rs::draw_sale_result`,
  `src/screens/dashboard.rs::draw_campaign_conclusion`.
  **Observed:** the sale screenshot truncates purchase/research/marketing/timing
  lessons into short ledger cells. Current code retains `label_fit` there and
  one long unwrapped reserve/campaign/demand/cost row. Conclusion repeats final
  rent in its goal summary and career prose. Dense detail competes with the
  explanation of why a deal succeeded or failed.
  **Change:** retain price, settlement release after debt/fees, actual profit and
  the main lesson as the result focus. Make the full deal autopsy an expandable,
  wrapped/scrollable report; consolidate decorative headings and duplicate facts.
  In the conclusion retain all three goal shortfalls and the binding constraint,
  with career detail disclosed separately. Keep progression and review actions
  clearly distinguished, including the cost/time consequence of Next Week.
  **Acceptance:** the player can read the complete reason for a loss or missed
  goal and see where the money went; no critical sentence ends only in ellipsis.
  **Verify:** baseline sizes, profitable/loss/pass-in sale, campaign win with
  negative net cashflow, week-24 failure and longest report strings; tap expand,
  scroll, dismiss, review portfolio and continue without hidden controls.

### Further inspection — do not report these as reproduced defects

- [ ] **UI-V1 — Verify dense collections and reachability before choosing pagination.**
  **Scope:** `src/screens/portfolio.rs` selector (`take(6)` and unbounded lease
  labels), `property_list.rs` fixed three-column rows, campaign/content limits and
  `src/app/capture.rs`. **Evidence:** source has a six-holding display cap and no
  scrolling for either collection; inspected portfolio images contain only three
  holdings. A reachable seventh holding or overflowing listing count was not
  established. **Action:** establish the maximum legal dense state, including
  review/maintenance labels. If entries or required controls become inaccessible,
  add touch paging/scrolling and wrap selected-state summaries using UI-01 metrics;
  otherwise document the enforced bound. **Acceptance/verification:** every legal
  holding/listing and urgent review can be selected at baseline sizes, with long
  labels and expanded text. Include an actual dense capture and tap path.

- [ ] **UI-V2 — Refresh stale evidence and complete browser/touch interaction review.**
  **Scope:** `src/app/capture.rs`, `docs/verification/`, generated browser canvas,
  settings, all changed screens. **Evidence:** older snapshots disagree with
  current navigation and auction layout; recorded window sizes differ from PNG
  dimensions. No present-pass runtime proof exists for resize, enlarged text,
  paused-auction errors or pass-in negotiation. **Action:** after the tasks above,
  replace equivalent captures at actual baseline sizes and record scene, revision,
  canvas dimensions and text setting. Exercise touch-only new game, research,
  lobby leave/start, RAISE/JUMP/WAIT, rival open/close, Walk Away/quick resolve,
  settlement, vendor test/counteroffer/leave, and portfolio recovery. Test title
  and in-game settings Apply/Cancel and save/load, including reopening a live room.
  **Acceptance:** no hover/keyboard dependency, click-through, lost dismissal,
  clipped warning or hidden primary control; exact costs and final outcomes remain
  readable after transient feedback ends. Check the actual embedded canvas and
  non-16:9 resize, not screenshots alone. Report physical-touch limitations honestly.
  This supplements, rather than duplicates, the full campaign release gate below.

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
