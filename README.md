
# Project Prime

**uniqueisrich.tg**

A NEAR smart contract for 888 proof-gated Genesis nodes across four shards (Truth, Sound, Maths, Skill; 222 each). Every node carries its own song and is bound to a story chapter that was published before it. Holders can anchor proofs of origin on-chain, first come first recorded.

Designed and written entirely on a Samsung Galaxy A02.

## Status

| Part | State |
|---|---|
| Contract design and source (`src/lib.rs`) | Complete |
| Interactive demo | Live. A **simulation** of the contract's rules in the browser |
| Compiled and tested | **Not yet.** I couldn't build on my phone |
| Deployed (testnet or mainnet) | **Not yet** |
| Security audit | **Not yet** |

I'm seeking support to build, deploy and review it. See [FUNDING.md](FUNDING.md).

## Demo

Interactive simulation: https://claude.ai/artifact/5KDtVNDYr1zLjdvXxDW9K2

It runs the contract's real rules (caps, gates, refunds, first-come anchoring) in your browser. File hashes are computed on your device and nothing is uploaded.

## What the contract does

- **Supply:** 888 total, 222 per shard, one node per account.
- **Collateral:** minting needs exactly 100 NEAR attached, held in escrow, refunded automatically if any gate fails, withdrawable after a 30-day lock.
- **Aje token check:** the contract calls the token's `ft_balance_of` itself. Nothing is self-reported.
- **Owner attestation:** community tasks and narrative proof can't be checked on-chain, so the owner attests them with `approve_miner`.
- **Story first:** `publish_chapter` stores a chapter's SHA-256 and link. A node can only mint against a published chapter.
- **A song per node:** minting takes a song title, audio link and SHA-256. The hash is anchored, so one song can't be bound to two nodes.
- **Proof of origin:** `anchor_proof(hash, label)` records the first account to anchor a hash, with block time and height.
- **Positron level:** each anchored proof adds a level (1 to 10) to the owner's node, and the node's on-chain SVG gains a harmonic ring.
- **On-chain art:** `nft_svg` renders a procedural SVG from each node's own hash, with the sound key 24949.

Audio and chapter files live off-chain (IPFS or similar). The chain holds the links and the SHA-256 fingerprints.

## Run it

See [DEPLOY.md](DEPLOY.md). Needs Rust, `cargo-near` and `near-cli`. The contract uses `near-sdk` 5.x.

## Files

- `src/lib.rs`: the contract
- `Cargo.toml`: manifest
- `DEPLOY.md`: build, deploy and demo commands
- `NARRATIVE.md`: the story behind the project
- `FUNDING.md`: what I'm asking for

## Roadmap (not in the current build)

NEP-171 compatibility, a living ledger of anchored human work, and zero-knowledge proofs for private anchoring.

## License

MIT
