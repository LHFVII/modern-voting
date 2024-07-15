pub mod contexts;
pub mod error;

use anchor_lang::prelude::*;

use contexts::*;

declare_id!("688SEWu1VXekVJCVq4ZbJGH8LQSHUonUPTtc9Lq5gWfg");

#[program]
pub mod modern_voting {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        poll_id: u64, 
        start_time: u64, 
        end_time:u64,
        name: String,
        description: String
    ) -> Result<()> {
        contexts::initialize(ctx, poll_id, start_time, end_time,name,description)
    }

    pub fn propose_candidate(
        ctx: Context<ProposeCandidate>, 
        candidate_index:u64,
        poll_id:u64,
        candidate_name:String,
        
    ) -> Result<()> {
        contexts::propose_candidate(ctx, candidate_index, poll_id, candidate_name)
    }

    pub fn vote(
        ctx: Context<Vote>,
        poll_id: u64,
    ) -> Result<()> {
        contexts::vote(ctx,poll_id)
    }
}


