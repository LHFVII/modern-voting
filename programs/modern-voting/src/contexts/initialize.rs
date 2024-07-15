use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<Initialize>,
    poll_id: u64, 
    start_time: u64, 
    end_time:u64,
    name: String,
    description: String
) -> Result<()> {
    ctx.accounts.poll_account.poll_start_time = start_time;
    ctx.accounts.poll_account.poll_end_time = end_time;
    ctx.accounts.poll_account.poll_name = name;
    ctx.accounts.poll_account.poll_description = description;
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
    pub poll_end_time:u64,
}

