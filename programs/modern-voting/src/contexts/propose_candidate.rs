use anchor_lang::prelude::*;
use crate::contexts::initialize::{PollAccount};

pub fn propose_candidate(ctx: Context<ProposeCandidate>,
                        poll_id: u64,
                        candidate_name: String,
                    ) -> Result<()> {
    ctx.accounts.candidate_account.candidate_name = candidate_name;
    ctx.accounts.poll_account.poll_option_index += 1;
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct ProposeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount{
    #[max_len(64)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}