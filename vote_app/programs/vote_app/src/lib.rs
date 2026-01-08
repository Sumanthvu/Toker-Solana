use anchor_lang::prelude::*;
mod state;
mod contexts;
use contexts::*;

declare_id!("9fka3EztcCih3UYhwqHpDYEcZtt4qt2QehrQ4cXLx8YE");

#[program]
pub mod vote_app {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

