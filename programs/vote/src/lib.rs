use std::alloc::System;

use anchor_lang::prelude::*;

declare_id!("3ZynTBMwRYRhi8jtyGZsJYappPo4Qa7WRwZompCa1tS3");

#[program]
pub mod vote {
    use super::*;

    pub fn register_candidate(ctx: Context<RegisterCandidate>, name: String) -> Result<()> {
        require!(name.len() > 0, CandidateError::ShortName);
        require!(name.len() <= 20, CandidateError::LongName);

        let candidate: &mut _= &mut ctx.accounts.candidate;
        candidate.set_inner(Candidate {
            c_name: name,
            vote_count: 0,
            candidate_id: ctx.accounts.owner.key(),
        });
        msg!("Candidate is registered successfully"); 

        Ok(())
    }

    pub fn register_voter(ctx: Context<RegisterVoter>, name: String ) -> Result<()> { 
        require!(name.len() > 0, VoterError::ShortName);
        require!(name.len() <= 20, VoterError::LongName);

        let voter: &mut _= &mut ctx.accounts.voter;
        voter.set_inner(Voter {
            v_name: name,
            is_voted: false,
            voter_id: ctx.accounts.owner.key(),
        });
        msg!("Voter is registered successfully");  

        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>) -> Result<()> { 
        let voter: &mut _= &mut ctx.accounts.voter;
        let candidate: &mut _= &mut ctx.accounts.candidate;

        require!(ctx.accounts.voter_signer.key() == voter.voter_id, VoterError::UnauthorizedVoter);
        require!(voter.is_voted== false, VoterError::AlreadyVoted);
        require!(candidate.vote_count>= 0, CandidateError::CandidateNotFound);
        
        candidate.vote_count += 1;
        voter.is_voted = true;

        msg!("Voter {} has voted for candidate {}", voter.voter_id, candidate.candidate_id);

        Ok(())
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
        pub voter_signer: Signer<'info>,
    }

    #[error_code]
    pub enum CandidateError {
        #[msg("Name is too short.")]
        ShortName,
        #[msg("Name is too long.")]
        LongName,
        #[msg("Candidate not found.")]
        CandidateNotFound,
    }

    #[error_code]
    pub enum VoterError {
        #[msg("You are not authorized to vote.")]
        UnauthorizedVoter,
        #[msg("You have already voted.")]
        AlreadyVoted,
        #[msg("Name is too short.")]
        ShortName,
        #[msg("Name is too long.")]
        LongName,
    }

}