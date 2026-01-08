use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Accounts)]

pub struct InitializeTreasury<'info>{
   #[account(mut)]
   pu authority:Signer<'info>,

   #account[account(
    init,
    payer=authority,
    space=8+TreasuryConfig::INIT_SPACE,
    seeds=[b"treasury_config"],
    bump
   )]
   pub treasury_config_account:Account<'info,TreasuryConfig> 

   pub system_program:Program<'info,System>
}