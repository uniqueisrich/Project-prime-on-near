//! Project Prime (uniqueisrich.near)
//! 888 Genesis nodes across 4 shards (222 each).
//!
//! Gates to mine a node:
//!  1. Collateral: exactly 100 NEAR attached to the call, escrowed in this contract
//!     (real, on-chain, cannot be faked by passing JSON).
//!  2. Aje token: verified by a cross-contract ft_balance_of call (> 0).
//!  3. Community tasks + narrative proof: attested on-chain by the owner via
//!     `approve_miner` (these are off-chain facts, so the owner signs off honestly).
//!
//! NOTE: not yet compiled/tested in the authoring environment. Run `cargo build`
//! and fix any small API mismatches before deploying.

use near_sdk::json_types::U128;
use near_sdk::store::{IterableMap, LookupMap, LookupSet};
use near_sdk::{
    env, ext_contract, near, require, AccountId, Gas, NearToken, PanicOnDefault, Promise,
    PromiseError,
};

pub const GLOBAL_CAP: u32 = 888;
pub const SHARD_CAP: u32 = 222;
pub const SOUND_KEY: u32 = 24949; // 0110000101110101
pub const COLLATERAL: NearToken = NearToken::from_near(100);
pub const LOCK_NS: u64 = 30 * 24 * 60 * 60 * 1_000_000_000; // 30 days

#[near(serializers = [json, borsh])]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shard {
    Truth,
    Sound,
    Maths,
    Skill,
}

impl Shard {
    fn index(&self) -> usize {
        match self {
            Shard::Truth => 0,
            Shard::Sound => 1,
            Shard::Maths => 2,
            Shard::Skill => 3,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Shard::Truth => "TRUTH",
            Shard::Sound => "SOUND",
            Shard::Maths => "MATHS",
            Shard::Skill => "SKILL",
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Node {
    pub token_id: String,
    pub owner_id: AccountId,
    pub shard: Shard,
    pub serial: u32,
    pub minted_at_ns: u64,
    pub unlock_at_ns: u64,
    pub collateral_released: bool,
    /// The story chapter this node is bound to (must be published before minting).
    pub chapter_id: u32,
    pub chapter_hash: String,
    /// This node's own song: title, location of the audio file, and its SHA-256.
    pub song_title: String,
    pub audio_uri: String,
    pub audio_hash: String,
}

/// A chapter of the Prime story, published on-chain by the owner BEFORE any node
/// can be minted against it. Only the hash and a link are stored, not the full text.
#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Chapter {
    pub id: u32,
    pub title: String,
    pub text_hash: String,
    pub uri: String,
    pub published_at_ns: u64,
}

fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_safe_text(s: &str, max: usize) -> bool {
    !s.is_empty() && s.len() <= max && !s.chars().any(|c| matches!(c, '<' | '>' | '&' | '\'' | '"'))
}

/// Proof of origin: a SHA-256 of a song, formula or manuscript, first come first anchored.
#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Anchor {
    pub hash: String,
    pub owner_id: AccountId,
    pub label: String,
    pub anchored_at_ns: u64,
    pub block_height: u64,
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    aje_token_id: AccountId,
    approved: LookupSet<AccountId>,
    nodes: IterableMap<String, Node>,
    minted_by: LookupSet<AccountId>, // one node per account
    anchors: LookupMap<String, Anchor>,
    anchor_counts: LookupMap<AccountId, u32>, // feeds the Positron level
    chapters: LookupMap<u32, Chapter>,
    shard_counts: [u32; 4],
    total: u32,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, aje_token_id: AccountId) -> Self {
        Self {
            owner_id,
            aje_token_id,
            approved: LookupSet::new(b"a"),
            nodes: IterableMap::new(b"n"),
            minted_by: LookupSet::new(b"m"),
            anchors: LookupMap::new(b"p"),
            anchor_counts: LookupMap::new(b"c"),
            chapters: LookupMap::new(b"h"),
            shard_counts: [0; 4],
            total: 0,
        }
    }

    // ---- Gate 3: owner attests community tasks + narrative proof ----
    pub fn approve_miner(&mut self, account_id: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Owner only");
        self.approved.insert(account_id);
    }

    // ---- The story comes first: chapters are published before nodes can exist ----
    /// Owner publishes a chapter: its SHA-256 (of the full text) and a link to read it.
    pub fn publish_chapter(&mut self, id: u32, title: String, text_hash: String, uri: String) -> Chapter {
        require!(env::predecessor_account_id() == self.owner_id, "Owner only");
        require!(!self.chapters.contains_key(&id), "Chapter already published");
        require!(is_sha256_hex(&text_hash), "text_hash must be 64 hex characters");
        require!(is_safe_text(&title, 80), "Bad title");
        require!(uri.len() <= 200, "uri too long");
        let chapter = Chapter {
            id,
            title,
            text_hash: text_hash.to_lowercase(),
            uri,
            published_at_ns: env::block_timestamp(),
        };
        self.chapters.insert(id, chapter.clone());
        chapter
    }

    pub fn get_chapter(&self, id: u32) -> Option<Chapter> {
        self.chapters.get(&id).cloned()
    }

    // ---- Mining ----
    /// Attach exactly 100 NEAR. Collateral is escrowed; if any gate fails it is refunded.
    /// Every node is minted against a PUBLISHED story chapter and carries its own song.
    #[payable]
    pub fn mine_l1_block(
        &mut self,
        shard: Shard,
        chapter_id: u32,
        song_title: String,
        audio_uri: String,
        audio_hash: String,
    ) -> Promise {
        let minter = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        require!(deposit == COLLATERAL, "Attach exactly 100 NEAR as collateral");
        require!(self.approved.contains(&minter), "Complete tasks and get approved first");
        require!(!self.minted_by.contains(&minter), "One node per account");
        require!(self.total < GLOBAL_CAP, "Genesis supply exhausted");
        require!(self.shard_counts[shard.index()] < SHARD_CAP, "Shard full");
        require!(self.chapters.contains_key(&chapter_id), "Story chapter not published yet");
        require!(is_safe_text(&song_title, 80), "Bad song title");
        require!(audio_uri.len() > 0 && audio_uri.len() <= 200, "Audio link required (max 200 chars)");
        require!(is_sha256_hex(&audio_hash), "audio_hash must be 64 hex characters");
        require!(
            !self.anchors.contains_key(&audio_hash.to_lowercase()),
            "This song is already bound to another node"
        );

        // Gate 2: real Aje token check. Final state changes happen in the callback.
        ext_ft::ext(self.aje_token_id.clone())
            .with_static_gas(Gas::from_tgas(5))
            .ft_balance_of(minter.clone())
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(20))
                    .on_aje_checked(minter, shard, deposit, chapter_id, song_title, audio_uri, audio_hash),
            )
    }

    #[private]
    pub fn on_aje_checked(
        &mut self,
        minter: AccountId,
        shard: Shard,
        deposit: NearToken,
        chapter_id: u32,
        song_title: String,
        audio_uri: String,
        audio_hash: String,
        #[callback_result] balance: Result<U128, PromiseError>,
    ) -> Option<String> {
        let has_aje = matches!(balance, Ok(b) if b.0 > 0);
        let audio_hash = audio_hash.to_lowercase();
        let has_room = self.total < GLOBAL_CAP
            && self.shard_counts[shard.index()] < SHARD_CAP
            && !self.minted_by.contains(&minter)
            && !self.anchors.contains_key(&audio_hash); // song still unclaimed

        if !has_aje || !has_room {
            // Refund collateral. The minter pays nothing on failure.
            let _ = Promise::new(minter).transfer(deposit);
            return None;
        }

        self.shard_counts[shard.index()] += 1;
        self.total += 1;
        let serial = self.shard_counts[shard.index()];
        let token_id = format!("{}-{}", shard.name(), serial);
        let now = env::block_timestamp();
        let chapter = self.chapters.get(&chapter_id).expect("Chapter missing").clone();
        let node = Node {
            token_id: token_id.clone(),
            owner_id: minter.clone(),
            shard,
            serial,
            minted_at_ns: now,
            unlock_at_ns: now + LOCK_NS,
            collateral_released: false,
            chapter_id,
            chapter_hash: chapter.text_hash,
            song_title: song_title.clone(),
            audio_uri,
            audio_hash: audio_hash.clone(),
        };
        // The node's song is also anchored as proof of origin, first come first served,
        // so the same song can never be bound to two nodes.
        self.anchors.insert(
            audio_hash.clone(),
            Anchor {
                hash: audio_hash,
                owner_id: minter.clone(),
                label: format!("Song: {}", song_title),
                anchored_at_ns: now,
                block_height: env::block_height(),
            },
        );
        self.nodes.insert(token_id.clone(), node);
        self.minted_by.insert(minter);
        Some(token_id)
    }

    /// After the lock period the holder may withdraw their 100 NEAR (node stays).
    pub fn release_collateral(&mut self, token_id: String) -> Promise {
        let node = self.nodes.get_mut(&token_id).expect("No such node");
        require!(node.owner_id == env::predecessor_account_id(), "Not your node");
        require!(!node.collateral_released, "Already released");
        require!(env::block_timestamp() >= node.unlock_at_ns, "Still locked");
        node.collateral_released = true;
        Promise::new(node.owner_id.clone()).transfer(COLLATERAL)
    }

    // ---- Proof of origin (Truth Shard feature) ----
    /// Anchor the SHA-256 (64 hex chars) of a file you made. The first account to
    /// anchor a hash is recorded permanently with the block time and height.
    /// Only node holders can anchor, which keeps storage spam out.
    /// Hash the file on your own device; the file itself never goes on-chain.
    pub fn anchor_proof(&mut self, hash: String, label: String) -> Anchor {
        let caller = env::predecessor_account_id();
        require!(self.minted_by.contains(&caller), "Hold a Prime node to anchor proofs");
        require!(hash.len() == 64, "Hash must be 64 hex characters (SHA-256)");
        require!(hash.chars().all(|c| c.is_ascii_hexdigit()), "Hash must be hex");
        require!(label.len() <= 80, "Label max 80 characters");
        let hash = hash.to_lowercase();
        require!(!self.anchors.contains_key(&hash), "Already anchored by someone first");

        let anchor = Anchor {
            hash: hash.clone(),
            owner_id: caller,
            label,
            anchored_at_ns: env::block_timestamp(),
            block_height: env::block_height(),
        };
        self.anchors.insert(hash, anchor.clone());
        let count = self.anchor_counts.get(&anchor.owner_id).copied().unwrap_or(0);
        self.anchor_counts.insert(anchor.owner_id.clone(), count + 1);
        anchor
    }

    /// Positron level: a node adapts as its owner anchors more proofs.
    /// Level 1 at mint, +1 per anchored proof, capped at 10. The node's SVG
    /// gains one harmonic ring per level, so the artwork evolves with real activity.
    pub fn positron_level(&self, token_id: String) -> u32 {
        let node = self.nodes.get(&token_id).expect("No such node");
        let count = self.anchor_counts.get(&node.owner_id).copied().unwrap_or(0);
        1 + count.min(9)
    }

    /// Who anchored this hash first, and when? None means nobody has.
    pub fn get_anchor(&self, hash: String) -> Option<Anchor> {
        self.anchors.get(&hash.to_lowercase()).cloned()
    }

    // ---- Views ----
    pub fn get_supply(&self) -> (u32, u32, [u32; 4]) {
        (self.total, GLOBAL_CAP, self.shard_counts)
    }

    pub fn is_approved(&self, account_id: AccountId) -> bool {
        self.approved.contains(&account_id)
    }

    pub fn get_node(&self, token_id: String) -> Option<Node> {
        self.nodes.get(&token_id).cloned()
    }

    /// Procedural on-chain SVG. Colors derive from the token's own hash, and the
    /// Sound key 24949 is embedded in every node.
    pub fn nft_svg(&self, token_id: String) -> String {
        let node = self.nodes.get(&token_id).expect("No such node");
        let h = env::sha256(token_id.as_bytes());
        let hue = (h[0] as u32 * 360) / 255;
        let mut cells = String::new();
        for i in 0..36usize {
            let on = (h[i % 32] >> (i % 8)) & 1 == 1;
            if on {
                let x = 20 + (i % 6) * 30;
                let y = 52 + (i / 6) * 30;
                cells.push_str(&format!(
                    "<rect x='{}' y='{}' width='24' height='24' rx='4' fill='hsl({},70%,60%)'/>",
                    x, y, (hue + i as u32 * 7) % 360
                ));
            }
        }
        let level = self.positron_level(token_id.clone());
        let mut rings = String::new();
        for l in 0..level {
            rings.push_str(&format!(
                "<circle cx='110' cy='150' r='{}' fill='none' stroke='hsl({},70%,60%)' stroke-width='1' opacity='0.5'/>",
                12 + l * 9,
                (hue + l * 25) % 360
            ));
        }
        format!(
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 220 260'>\
             <rect width='220' height='260' fill='#0b0b14'/>\
             <text x='110' y='28' fill='hsl({},70%,70%)' font-family='monospace' font-size='14' text-anchor='middle'>PRIME {}</text>\
             <text x='110' y='46' fill='#888' font-family='monospace' font-size='10' text-anchor='middle'>sound key {}</text>\
             {}{}\
             <text x='110' y='238' fill='#888' font-family='monospace' font-size='10' text-anchor='middle'>{} #{} | L{} | Ch.{}</text>\
             <text x='110' y='254' fill='hsl({},70%,70%)' font-family='monospace' font-size='9' text-anchor='middle'>song: {}</text>\
             </svg>",
            hue, node.token_id, SOUND_KEY, cells, rings, node.shard.name(), node.serial, level,
            node.chapter_id, hue, node.song_title
        )
    }
}
