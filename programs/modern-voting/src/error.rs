use anchor_lang::prelude::*;

#[error_code]
pub enum VoteProgramError {

    #[msg("Voting has not started yet")]
    VotingHasNotStarted,
    
    #[msg("Voting time ended")]
    VotingEnded,
}