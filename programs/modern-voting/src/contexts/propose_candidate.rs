use anchor_lang::prelude::*;
use crate::contexts::initialize::{PollAccount};

pub fn propose_candidate(ctx: Context<ProposeCandidate>, 
                        poll_id:u64, 
                        candidate_index: u64,
                        name: String, 
                    ) -> Result<()> {
    ctx.accounts.candidate_account.name = name;
    ctx.accounts.candidate_account.num_votes = 0;
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_index: u64)]
pub struct ProposeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_index.to_le_bytes().as_ref(), ],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    #[account(mut)]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount{
    #[max_len(64)]
    pub name: String,
    pub num_votes: u64,
}