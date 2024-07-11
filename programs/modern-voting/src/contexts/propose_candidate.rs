use anchor_lang::prelude::*;

pub fn propose_candidate(ctx: Context<ProposeCandidate>) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct ProposeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount{
    #[max_len(64)]
    pub name: String,
    pub num_votes: u64,
}