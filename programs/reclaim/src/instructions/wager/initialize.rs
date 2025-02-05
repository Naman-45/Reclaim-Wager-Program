use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{ Mint, TokenAccount, transfer_checked, TransferChecked, TokenInterface }, associated_token::AssociatedToken};

use crate::state::Challenge;

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Challenge::INIT_SPACE,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = creator,
        associated_token::token_program = token_program
    )]
    pub creator_token_account : InterfaceAccount<'info, TokenAccount>,
   
    /// CHECK: program's escrow account
    #[account(
        init,
        payer = creator,
        associated_token::mint = token_mint,
        associated_token::authority = challenge,
        associated_token::token_program = token_program,
    )]
    pub program_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn init_challenge(ctx: Context<Initialize>, _challenge_id: String, wager_amount: u64) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;
        challenge.creator = ctx.accounts.creator.key();
        challenge.wager_amount = wager_amount;
        challenge.opponent = None;
        challenge.result_settled = false;
        challenge.winner = None;
        challenge.challenge_bump = ctx.bumps.challenge;

        let accounts = &ctx.accounts;

        let transfer_accounts = TransferChecked {
            from: accounts.creator_token_account.to_account_info(),
            mint: accounts.token_mint.to_account_info(),
            to: accounts.program_token_account.to_account_info(),
            authority: accounts.creator_token_account.to_account_info()
        };


        let context = CpiContext::new(accounts.token_program.to_account_info(),transfer_accounts);

        transfer_checked(context, wager_amount, accounts.token_mint.decimals)?;

        Ok(())
}
