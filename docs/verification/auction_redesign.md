# Auction interaction verification

Validated 19 September 2026. The established room layout is retained, with
40 more virtual pixels of room width and a utility bar reduced from 68 to 44.
The permanent Notes control and financial sidebar are gone. A contextual
margin appears at the next-bid limit; rival tells remain available on selection.

Bidders use a short preparation beat before a committed bid, recheck affordability
before acting, briefly raise their paddle and flash, then lower it. Hesitation
adds small movement; withdrawal lowers and desaturates the portrait. Observable
states replace generic interest text. The auctioneer acknowledges bids with a
brief gesture and changes calls with price, silence, selling status and final calls.
Behaviour messages fade after three seconds. Wait observations expire after six
seconds or a new bid; no persistent last-read or duplicate bid-price event remains.

The rail contains a current-bid dot, an estimate tick and a personal-limit tick.
RAISE remains dominant. JUMP has a limited-use outline treatment; WAIT is quiet.
WALK AWAY is separate from Menu and clear of the bottom edge used by host overlays.

Validation:
- All 85 tests pass, including five new interaction cases for staged bidding,
  price rechecks, reactive calls, distinct opponent states and save compatibility.
- Strict all-target Clippy, formatting, diff checks and the source-size gate pass.
- Default publish.ps1 builds Windows and WebGL and deploys Preview successfully.
  Project Roost tracking cannot connect to its local service on 127.0.0.1:80.
- Native deterministic captures cover opening, preparation, a bid flash,
  hesitation, rival withdrawal, limit context, reads, final calls and player exit.
  Preparation, bid and withdrawal captures advance the actual simulation.
- Smaller-screen captures check the opening, preparation and final-call layouts
  at 1024x576. These supplement the 1280x720 capture set.
- Input uses the existing toolkit release-based mouse/touch targets. This pass
  does not claim a new browser playthrough or physical touchscreen verification.

Pending decisions and reaction timers serialize with the auction. Older saves
receive safe defaults. Prices and hammer results remain deterministic across a
save/resume during preparation. The added preparation delay can change when an
opponent acts compared with an older version; the campaign replay still passes.
