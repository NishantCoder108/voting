#![allow(clippy::result_large_err)]

use std::task::Poll;

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(_ctx: Context<InitializePoll>, _pll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePoll<'info> {
    // Signer Account
    // we want signer so , we have a mutable account , where we update some data inside account
    #[account(mut)]
    pub signer: Signer<'info>,

    // Poll account
    #[account(
      init, //account will automatically initialize
      payer = payer,
      space = 8 + Poll::INIT_SPACE, //8 is defaut bytes space and poll have defined init space , that space will come here
    )]
    pub poll: Account<'info, Poll>,
}

#[account]
#[derive(InitSpace)] // It will give derive spaces , no need to calculations
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)] // give macro for string , max length
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}
