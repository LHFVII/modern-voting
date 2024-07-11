pub mod contexts;

use anchor_lang::prelude::*;

use contexts::*;

declare_id!("688SEWu1VXekVJCVq4ZbJGH8LQSHUonUPTtc9Lq5gWfg");

#[program]
pub mod modern_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        contexts::initialize(ctx)
    }
}


