pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("6UrqmN5T95eyF51XfWcnHNoQq7HBHjLn8onRd8iiEa3d");

#[program]
pub mod blueshift_anchor_flash_loan {
    use super::*;

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        borrow::handler(ctx, amount)
    }
    pub fn repay(ctx: Context<Repay>) -> Result<()> {
        repay::handler(ctx)
    }
}
