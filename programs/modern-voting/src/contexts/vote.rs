use anchor_lang::prelude::*;
use crate::contexts::propose_candidate::{CandidateAccount};

pub fn vote(ctx: Context<Vote>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct Vote<'info>{
    pub signer: Signer<'info>,
    #[account(mut)]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>,
}