# Project Prime: Support Request

**Builder:** uniqueisrich · uniqueisrich.near
**Stage:** Prototype. The contract design and code are complete; the interactive demo simulates its rules. The contract has not yet been compiled, deployed or audited.

## What I'm asking for

Help turning a finished prototype into a deployed, reviewed NEAR contract. I built this on a Samsung Galaxy A02 with no laptop and unreliable power and signal. The design is done. What I lack is the equipment and support to build, test and ship it.

## What I need and what it unlocks

| Need | Why | What it unlocks |
|---|---|---|
| A reliable computer (laptop) | I can't compile or test Rust on a phone | First build of the contract, bug fixing, tests |
| Graphics pad / drawing tablet | Node artwork, shard visuals and the project's design work are currently done on a phone | Better SVG and visual design for the nodes and the interface |
| A better phone | My Galaxy A02 is the only device I have, and it is failing | A dependable device for wallets, testing and mobile demos |
| Solar panel with battery | Power on the farm and at home is unreliable | Constant power for development and uploads |
| Internet access | Reliable access to the toolchain, testnet and tools | Daily development without walking for signal |
| NEAR for testnet and mainnet deployment | Gas and the 100 NEAR collateral mechanic must be tested | Deployment, full mint-and-release testing |
| Security review of the contract | It escrows funds and calls other contracts | Safe mainnet launch |
| Audio and chapter hosting (IPFS or similar) | Each node links to its own song and story chapter | Real song files behind every node |
| Mentorship from NEAR developers | Review of my cross-contract calls and storage design | A stronger, standards-aligned contract |

**Amount requested: about $5,000.** Proposed split (to be adjusted to current local quotes):

| Item | Estimate |
|---|---|
| Laptop | $1,200 |
| Solar panel, battery and inverter | $1,200 |
| Security / peer review of the contract | $800 |
| Better phone | $500 |
| NEAR for testnet and mainnet deployment and testing | $400 |
| Graphics pad / drawing tablet | $300 |
| Internet access (6 months) | $300 |
| File hosting and miscellaneous | $300 |
| **Total** | **$5,000** |

## Plan and milestones

1. **Build and test (weeks 1-2).** Compile the contract, fix errors, write unit tests, and deploy to testnet with a stand-in NEP-141 token.
2. **Demo on-chain (week 3).** Run the full flow on testnet: publish a chapter, mint a node with its own song, anchor a proof, release collateral. Replace the simulated demo with a live one.
3. **Review (weeks 4-5).** Security review and fixes. Add NEP-171 compatibility so nodes work with NFT marketplaces and wallets.
4. **Genesis (week 6).** Mainnet deployment and the first Genesis nodes, starting with the Lagos Anthem Sound Shard node.

*Adjust the timing to the program's own schedule.*

## What already exists

- Contract source: 888 cap, 222 per shard, one node per account, escrowed 100 NEAR collateral with a 30-day lock, Aje token check by cross-contract call, owner-attested community tasks.
- Every node bound to its own song (title, link, SHA-256) and to a story chapter published before it.
- First-come proof-of-origin anchoring (`anchor_proof`) and the Positron level system.
- Procedural on-chain SVG for every node.
- Interactive demo that runs these rules, clearly labeled as a simulation.

## Why this project, and why me

Builders and artists lose credit for work they made first, and the ones without capital or tools are hit hardest. Project Prime gives them a permanent, neutral record: this song, this story, this proof, anchored by this person at this time.

I built the whole design on the smallest device I had, walking to Idi-Ayunre for signal and syncing code between farm work and teaching. The prototype shows that this can be built under real constraints. With the right support it can ship, and it can show other builders in similar conditions that they can ship too.

## Roadmap after launch (not in the current build)

- NEP-171 standard for marketplace support
- A living ledger of human work: songs, research and writing, each with a verified first-anchor timestamp
- Zero-knowledge proofs for private anchoring

## Short version (about 100 words, paste-ready)

> Project Prime is a NEAR smart contract for 888 Genesis nodes in four shards. Each node carries its own song and is bound to a story chapter published before it, and holders can anchor proofs of origin on-chain, first come first recorded. The design and code are complete, and an interactive demo simulates the contract. I built all of it on a Samsung Galaxy A02, walking daily for signal. I'm requesting about $5,000 for a laptop, a drawing pad, a better phone, solar power, internet, NEAR for deployment and a security review, so I can deploy, audit and launch it.
