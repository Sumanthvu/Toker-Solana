use anchor_lang::prelude::*;

declare_id!("9fka3EztcCih3UYhwqHpDYEcZtt4qt2QehrQ4cXLx8YE");

#[program]
pub mod vote_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
