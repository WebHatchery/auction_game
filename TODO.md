# Remaining release gates

- Create or confirm the itch.io project for `webhatchery/auction-house-tycoon`, then rerun `publish-itch.ps1 -Status` until the configured `html5-demo` and `windows` channels are valid.
- Complete and record a full new-game campaign win and week-24 failure on both browser and Windows release builds, including the research, auction, pass-in, repair or renovation, leasing, rent review, refinance or sale, save/load, and final-ledger actions. The current record covers browser smoke, deterministic replay, and native capture states.
- After those gates pass, change `game_page.json` from Playable to Released and run the final publisher checks.
