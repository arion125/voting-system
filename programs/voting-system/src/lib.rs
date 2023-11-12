use anchor_lang::prelude::*;

declare_id!("5NRWQgocGR6uN4YyM83gpu3HbeFnx9ojHYwzDMJKzSqj");

#[program]
pub mod voting_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
        let signer = &ctx.accounts.signer;
        program_config.authority = signer.key();
        Ok(())
    }

    pub fn create_candidate(ctx: Context<CreateCandidate>, name: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.name = name;
        candidate.votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, candidate_name: String) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        let candidate = &mut ctx.accounts.candidate;
        if !voter.has_voted {
            candidate.votes += 1;
            voter.has_voted = true;
            voter.candidate = candidate.key();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer=signer, 
        space = 8 + 32,
        seeds = [b"program_config"],
        bump
    )]
    pub program_config: Account<'info, ProgramConfig>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateCandidate<'info> {
    #[account(
        init, 
        payer=signer, 
        space = 8 + 32 + 8,
        seeds = [b"candidate", name.as_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    #[account(
        mut,
        address = program_config.authority
    )]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"program_config"],
        bump
    )]
    pub program_config: Account<'info, ProgramConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(candidate_name: String)]
pub struct Vote<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 1,
        seeds = [b"vote", signer.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,
    #[account(
        mut, 
        seeds = [b"candidate", candidate_name.as_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Candidate {
    pub name: String,
    pub votes: u64,
}

#[account]
pub struct Voter {
    pub has_voted: bool,
    pub candidate: Pubkey,
}

#[account]
pub struct ProgramConfig {
    pub authority: Pubkey
}
