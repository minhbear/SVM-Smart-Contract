use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

cfg_if::cfg_if! {
 if #[cfg(feature = "dev")] {
    pub const PROGRAM_ID: Pubkey = pubkey!("HpY1m9BGSJKdzMUaZzboRhoicTUjePvXqVSz5UrKScnu");
  } else {
    // Default use for localnet
    pub const PROGRAM_ID: Pubkey = pubkey!("HpY1m9BGSJKdzMUaZzboRhoicTUjePvXqVSz5UrKScnu");
  }
}
