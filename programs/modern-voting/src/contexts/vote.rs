use anchor_lang::prelude::*;

use crate::error::{VoteProgramError};
use crate::contexts::initialize::{PollAccount};
use crate::contexts::propose_candidate::{CandidateAccount};

pub fn vote(ctx: Context<Vote>, poll_id: u64, candidate_name: String) -> Result<()>{
    let candidate_account = &mut ctx.accounts.candidate_account;
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time < ctx.accounts.poll_account.poll_end_time.try_into().unwrap(),
        VoteProgramError::VotingEnded
    );
    require!(
        current_time >= ctx.accounts.poll_account.poll_start_time.try_into().unwrap(),
        VoteProgramError::VotingHasNotStarted
    );
    candidate_account.candidate_votes += 1;
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct Vote<'info>{
    pub signer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>,
}

