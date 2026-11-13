# Project Prime
**uniqueisrich.tg**

*A NEAR smart contract for proof of origin, built entirely on a Samsung Galaxy A02.*

## The problem

Builders and artists lose credit for work they made first. A song, a formula or a manuscript gets copied, and the original maker has no neutral record showing it was theirs, or when it existed. Talent without tools or capital is the easiest to overlook.

## Why I built it

I started university in 2018 as a chemist and an artist. Strikes and lockdowns froze my progress, so I delivered food by day, studied chemistry by night, and taught myself smart-contract design on NEAR. In 2024, money pressure forced me to defer my studies and move to my grandmother's farm in Sonso, Ibadan. The farm had no network and no reliable power. The one device I had was a budget Galaxy A02, so I walked to Idi-Ayunre every day to catch a signal, push code and sync my wallet. I rebuilt my voice in a church choir and recorded *Lagos Anthem* with my brother. After a hard final month in Ibadan, I moved to Lagos with that same phone and finished the contract here.

Two sentences guided me the whole way: "Unto whom much is given, much is expected," and "With great power comes great responsibility." Project Prime is what I built to honor them: a record that holds what you made, and when.

## What Project Prime is

A smart contract on NEAR with **888 Genesis nodes** in four shards of **222** each. It enforces the supply caps in code.

| Shard | Theme | What it means in the contract |
|---|---|---|
| **Truth** | Origin | Records owner and mint time permanently; hosts proof-of-origin anchoring |
| **Sound** | Acoustic key | Every node embeds the key **24949**, drawn from *Lagos Anthem* |
| **Maths** | Logic | Capped tier with its own on-chain hex-matrix artwork |
| **Skill** | Adaptation | Nodes evolve through the Positron mechanism below |

## How it works

**Minting a node requires three gates, all enforced by the contract:**
1. **100 NEAR collateral**, attached to the call and held in escrow. It is refunded automatically if any other gate fails, and the holder can withdraw it after a 30-day lock.
2. **Aje token balance**, verified by the contract calling the token's `ft_balance_of`. Nothing is self-reported.
3. **Community tasks and narrative proof**, which cannot be checked on-chain, so the owner attests them with `approve_miner`.

Each account can hold one node, and the contract rejects mints once a shard or the total is full.

**Proof of origin (`anchor_proof`).** A node holder hashes a file (song, formula, manuscript) on their own device and anchors the SHA-256 on-chain. The first account to anchor a hash is recorded permanently with block time and height. Anyone can check with `get_anchor`. The file itself never leaves the owner's device.

**Positron adaptation (`positron_level`).** Every node starts at level 1 and gains a level for each proof its owner anchors, up to level 10. The node's on-chain SVG gains one harmonic ring per level, so the artwork evolves with real activity rather than staying fixed at mint.

**On-chain art (`nft_svg`).** Every node renders a unique procedural SVG derived from its own hash, with the sound key and Positron level built in.

## Hardware proof

The prototype was conceived, written and debugged on a Galaxy A02. If it can be built on this phone, anyone with any device can use it.

## What's next

After the 888 Genesis nodes, I plan to extend anchoring into a living ledger of human work: songs, research and writing, each with a verifiable first-anchor timestamp. Later versions will explore zero-knowledge proofs for private anchoring and NEP-171 compatibility for marketplace support. These are on the roadmap, not in the current build.

## Try it

See DEPLOY.md: approve a miner, mint a node with 100 NEAR, anchor a proof, then view `get_supply`, `get_anchor`, `positron_level` and `nft_svg`.
