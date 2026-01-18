use std::alloc::System;

use anchor_lang::prelude::*;

declare_id!("3ZynTBMwRYRhi8jtyGZsJYappPo4Qa7WRwZompCa1tS3");

#[program]
pub mod vote {
    use super::*;

    pub fn register_candidate(ctx: Context<RegisterCandidate>) -> Result<()> {  
        Ok(())
    }

    pub fn register_voter(ctx: Context<RegisterVoter>) -> Result<()> {  
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(20)]
    c_name: String,
    vote_count: u64,
    candidate_id: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    #[max_len(20)]
    v_name: String,
    is_voted: bool,
    voter_id: Pubkey,
}


#[derive(Accounts)]
#[instruction(c_name: String)]
pub struct RegisterCandidate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init,
    payer= owner,
    space= 8 + Candidate::INIT_SPACE,
    seeds= [c_name.as_bytes(), owner.key().as_ref()],
    bump)]
    pub candidate: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(v_name: String)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init,
    payer= owner,
    space= 8 + Voter::INIT_SPACE,
    seeds= [v_name.as_bytes(), owner.key().as_ref()],
    bump)]
    pub voter: Account<'info, Voter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Account<'info, Voter>,

     #[account(mut)]
    pub candidate: Account<'info, Candidate>,

    #[account(mut)]
    pub voter_signer: Signer<'info>
}