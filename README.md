# Auction House Tycoon

Auction House Tycoon is a property strategy game about doing the work before Saturday, buying under pressure, and turning the right homes into a genuine rental portfolio.

Winning an auction is not automatically good. The real challenge is knowing what a property is worth, when to stop bidding, and whether a renovation will improve the deal or just make a loss prettier.

## The auction room

The live auction is a pixel-art room: a visible auctioneer at the rostrum,
seated rivals with raised or lowered paddles, and a bidding desk in front of
you. The centre connects the current leader, price, changing auctioneer calls,
reactions and your controls. No workflow navigation appears during bidding.

Tap the large RAISE paddle for one step. The outlined JUMP is a once-per-room
challenge that can rattle investors or provoke emotional bidders. The quieter
WAIT & READ ROOM yields tempo for an observation that fades. WALK AWAY sits
apart from the bidding desk as an auction decision; Menu remains a small
utility control. Mouse and touch releases use the toolkit's shared pointer handling.

Rivals calculate, prepare, hesitate, react to pressure and withdraw as their
actual decisions unfold. A committed bid has a short preparation beat and
rechecks the price before acting. Paddles lift briefly, bidding portraits flash,
hesitant bidders shift, and withdrawn bidders slump with lowered paddles.
Tap a rival for a contextual tell. Financial research stays out of the permanent
left column; estimated margin reappears when the next bid reaches your limit.

The price rail uses one current-bid dot and two reference marks for estimated
value and your limit. Auctioneer calls respond to new bids, silence, selling
status and final calls. The line below carries a fading behavioural observation,
without repeating the leader and price. Pending bidder decisions survive saves.

As the room slows, the auctioneer changes cadence. Final calls tighten the
spotlight, raise the gavel and count the calls visually. A precise clock only
appears in the last six seconds. Late bids reopen the calls through the
existing overtime rules. After the hammer, stay in the room for the result,
then tap REVIEW PURCHASE, REVIEW OUTCOME or TALK TO THE AGENT.

Scout -> Research -> Register -> Auction -> Outcome -> Recover remains the
campaign structure, explained outside the live auction.

## Screen decisions and viewport contract

The game uses a 1200×675 logical canvas with a preserved 16:9 viewport. The
normal canvas is 1280×720; 1024×576 is the supported minimum. A historically
captured 1008×537 drawable area is below the contract and may letterbox. The
same toolkit mapping is used for rendering, mouse, and touch, including
embedded non-16:9 canvases and display scaling.

| Screen | Current decision | Dominant focus and primary action | Supporting / deferred information |
| --- | --- | --- | --- |
| Preparation | Which listing deserves a registration? | Compare the current property and set a researched walk-away before registering. | Cash, research cost, settlement, rent and bank constraints stay beside the decision; long reports open on demand. |
| Auction | Should the next bid still fit the plan? | Current lot, price and RAISE/JUMP/WALK AWAY. | Rivals and cash-buffer warnings support the bid; detailed finance and rival notes open contextually. |
| Recovery | Which weekly action protects the portfolio? | Urgent review/maintenance first, otherwise scout, lease, improve, sell or advance the week. | Capital and goal progress remain visible; full statements, finance, and sale preparation disclose on tap. |
| Outcome | What did the deal actually produce? | Read settlement/profit or the binding campaign shortfall, then review the portfolio or continue. | Full deal autopsy and career history are expandable/retrievable rather than competing with the result. |

Every required primary control has a visible mouse/touch target and the input
conversion follows the rendered letterboxed viewport. Secondary collections
reflow or disclose before text is reduced; the auction room keeps its useful
framing at the minimum size.

## Gameplay

- Study a rotating Saturday schedule and spend up to two auction registrations each week.
- Inspect properties and estimate risk.
- Read distinct compact, premium, and large-block facades as quickly as their numbers.
- Set a walk-away price before bidding.
- Review unconditional auction-day terms, the 10% contract deposit, acquisition costs, and finance before tapping START AUCTION CALLS.
- Raise normally, make one assertive jump, wait for a rival tell, or walk away in live auctions.
- Watch different emotional rivals break their own limits for different reasons while investors, developers, and bargain hunters keep their ceilings.
- Resume a saved live auction without rerolling the room, and survive a suspended browser tab without losing the clock.
- Load older saves safely: rebuilt hammer ledgers, live-room state, leasing campaigns, and rent-review schedules all have migration defaults.
- Tap WAIT & READ ROOM to earn an observation; it remains visible briefly as a past read after the next bid.
- Meet the same named rivals across Saturdays and build a notebook of their appearances, wins, stretching behaviour, and highest rooms.
- Hear a legal vendor bid below the hidden reserve and the auctioneer's decisive on-market call.
- Open in $10,000 rises, then bid in tighter $5,000 steps after the on-market call.
- Negotiate a visible vendor counteroffer after a pass-in, or leave the private deal untouched.
- Test the vendor once at the public-room price before deciding whether to meet that counteroffer.
- See when the property is selling, and use full diligence to read a passed-in seller as flexible, negotiable, or firm without revealing their exact floor.
- Build a season record that credits disciplined exits alongside purchases and profitable sales.
- Close each season with the registrations deliberately passed up, final rent, realised sale profit, and auction record visible together.
- Pay down individual property loans when freeing bank headroom matters more than keeping cash liquid, with the exact weekly interest effect reported after payment.
- Refinance seasoned rental equity to fund another deposit without selling the home, with released cash, new debt, fee, and weekly interest increase stated separately.
- Read the seller-side auction ledger: reserve, bidder depth, debt cleared, settlement release, and true profit.
- Read a weekly portfolio statement instead of guessing where rent and cash went; mortgage interest is calculated to the nearest dollar so principal changes have proportionate effects.
- Renovate, fund a one-week rental campaign, place or end tenancies, collect rent, handle scheduled maintenance, hold, or sell purchased properties.
- Track every property, cash, debt, rent, weekly cashflow, fees, repairs, and net worth.
- Read each home's equity, loan-to-value ratio, and market-adjusted annual loan rate before deciding whether to hold, pay down, refinance, or sell.
- Underwrite the rent at your chosen walk-away price, including management, property outgoings, and loan interest, before registering to bid.
- Open PROPERTY NOTES for the rental cashflow forecast while bidding; keep the main stage focused on price and your limit.
- Carry a visible registered paddle from the bidder-terms lobby into the room, where the current leading bidder is always named.
- Translate a hammer win into a settlement ledger showing the 10% deposit, new loan, rent appraisal, and leveraged weekly cashflow before purchase settlement.
- Diagnose every owned home's true weekly result after its rent, management, loan interest, property costs, and maintenance impact.
- Handle scheduled eight-week rent reviews: renew safely, or ask for the current market rent and risk a vacancy in a weak tenant market.
- Carry completed reviews and review-created vacancies into the persistent season ledger and final portfolio debrief.
- Keep the original gross-rent campaign target, while the final ledger separately names true net weekly cashflow and warns when a winning portfolio still needs deleveraging.
- Preserve a safe cash buffer that grows by $8,000 per portfolio door; auction guidance warns when the next bid leaves too little resilience.
- When borrowing power reaches zero, the dashboard names the intended recovery routes: pay down debt, earn discipline reputation by leaving an overheated auction, or recycle an asset through sale.

## Goal

Before week 24, own three homes, earn $1,500 in weekly rent, and hold $240,000 net worth. A player usually needs both patient rental acquisitions and at least one value-creating sale or renovation to get there.

The campaign closes as soon as all three conditions are true—there is no need to advance another week and pay another round of holding costs. The authored economy includes a tested route through three disciplined, income-focused purchases, while stronger renovation and resale decisions create alternate routes.

## Controls

- Mouse/touch: every required action has a visible button or property card.

## Release candidate

Release-candidate research-to-auction-to-portfolio loop with six market pulses, twelve authored properties, live rival reactions, rentals, renovations, sale outcomes, and three-part campaign progress. Audio feedback and persisted volume/readability settings are available from the title screen or in-game menu.

Properties, bidder profiles, upgrades, and market events are data-driven from
`assets/game_data.json`.

See [RELEASE_NOTES.md](RELEASE_NOTES.md) for the release summary, [CREDITS.md](CREDITS.md) for shipped asset attribution, [LICENSE](LICENSE) for licensing, and [SUPPORT.md](SUPPORT.md) for bug reports and support details. The catalog remains Playable until the final external and manual release gates are complete.
