use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount{
    #[max_len(64)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_option_index: u64,
    pub poll_start_time: u64,
    pub poll_end_date:u64,
}

