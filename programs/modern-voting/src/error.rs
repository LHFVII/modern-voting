use anchor_lang::prelude::*;

#[error_code]
pub enum VoteProgramError {
    
    #[msg("Voting time ended")]
    VotingEnded,
}