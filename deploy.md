# Project Prime — build & deploy (testnet)

## Build
    cargo install cargo-near      # once
    cargo near build              # or: cargo build --target wasm32-unknown-unknown --release

## Deploy
Use any testnet Aje stand-in token contract (a standard NEP-141) for the demo.

    near contract deploy uniqueisrich-demo.testnet use-file target/near/uniqueisrich_prime.wasm \
      with-init-call new json-args '{"owner_id":"uniqueisrich-demo.testnet","aje_token_id":"<AJE_TOKEN>.testnet"}' \
      prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send

## Demo flow
1. Owner approves a miner (gate 3):
       near contract call-function as-transaction uniqueisrich-demo.testnet approve_miner json-args '{"account_id":"MINER.testnet"}' prepaid-gas '30 Tgas' attached-deposit '0 NEAR' sign-as uniqueisrich-demo.testnet network-config testnet
1b. Owner publishes a story chapter FIRST (hash the chapter text with sha256sum; nodes cannot mint without one):
       near contract call-function as-transaction uniqueisrich-demo.testnet publish_chapter json-args '{"id":1,"title":"Sonso","text_hash":"<64-hex-sha256-of-chapter>","uri":"<link-to-chapter>"}' prepaid-gas '30 Tgas' attached-deposit '0 NEAR' sign-as uniqueisrich-demo.testnet network-config testnet
2. Miner mints against that chapter, with the node's own song (needs 100 NEAR + Aje balance).
   Upload the song to IPFS/Arweave or any host, then hash it: sha256sum lagos_anthem.mp3
       near contract call-function as-transaction uniqueisrich-demo.testnet mine_l1_block json-args '{"shard":"Sound","chapter_id":1,"song_title":"Lagos Anthem","audio_uri":"ipfs://<cid>","audio_hash":"<64-hex-sha256-of-song>"}' prepaid-gas '80 Tgas' attached-deposit '100 NEAR' sign-as MINER.testnet network-config testnet
3. Anchor proof of origin (node holders only). Get a hash on your phone/PC first, e.g. `sha256sum lagos_anthem.mp3`:
       near contract call-function as-transaction uniqueisrich-demo.testnet anchor_proof json-args '{"hash":"<64-hex-sha256>","label":"Lagos Anthem demo"}' prepaid-gas '30 Tgas' attached-deposit '0 NEAR' sign-as MINER.testnet network-config testnet
       near contract call-function as-read-only uniqueisrich-demo.testnet get_anchor json-args '{"hash":"<64-hex-sha256>"}' network-config testnet
4. View it:
       near contract call-function as-read-only uniqueisrich-demo.testnet nft_svg json-args '{"token_id":"TRUTH-1"}' network-config testnet
       near contract call-function as-read-only uniqueisrich-demo.testnet get_supply json-args '{}' network-config testnet
