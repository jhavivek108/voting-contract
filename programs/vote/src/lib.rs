use anchor_lang::prelude::*;

declare_id!("3ZynTBMwRYRhi8jtyGZsJYappPo4Qa7WRwZompCa1tS3");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
