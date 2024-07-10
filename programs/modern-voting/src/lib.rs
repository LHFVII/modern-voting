use anchor_lang::prelude::*;

declare_id!("688SEWu1VXekVJCVq4ZbJGH8LQSHUonUPTtc9Lq5gWfg");

#[program]
pub mod modern_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
