use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info,System>
}