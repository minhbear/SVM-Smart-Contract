use anchor_lang::prelude::*;

mod program_id;
use program_id::PROGRAM_ID;
mod common;
use common::*;
mod contexts;
use contexts::*;
mod states;
use states::*;

declare_id!(PROGRAM_ID);

#[program]
pub mod svm_smartcontract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }
}
