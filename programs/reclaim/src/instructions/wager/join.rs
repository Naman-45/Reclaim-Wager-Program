use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{ Mint, TokenAccount, transfer_checked, TransferChecked, TokenInterface }, associated_token::AssociatedToken};

use crate::{state::Challenge, error::WagerError};

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct JoinChallenge<'info> {
    #[account(mut)]
    pub opponent: Signer<'info>,

    #[account(
        mut,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump = challenge.challenge_bump
    )]
    pub challenge: Account<'info, Challenge>,
   
   #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, Mint>, 

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = opponent,
        associated_token::token_program = token_program
    )]
    pub opponent_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = challenge,
        associated_token::token_program = token_program,
    )]
    pub program_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn join_challenge(ctx: Context<JoinChallenge>, _challenge_id: String, wager_amount: u64) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;

    require!(
        challenge.opponent.is_none(),
        WagerError::ChallengeAlreadyJoined
    );

    require!(
        challenge.creator != ctx.accounts.opponent.key(),
        WagerError::CannotJoinYourOwnChallenge
    );

    require!(
        challenge.wager_amount == wager_amount,
        WagerError::IncorrectWagerAmount
    );

    challenge.opponent = Some(ctx.accounts.opponent.key());

    let accounts = &ctx.accounts;

        let transfer_accounts = TransferChecked {
            from: accounts.opponent_token_account.to_account_info(),
            mint: accounts.token_mint.to_account_info(),
            to: accounts.program_token_account.to_account_info(),
            authority: accounts.opponent_token_account.to_account_info()
        };


        let context = CpiContext::new(accounts.token_program.to_account_info(),transfer_accounts);

        transfer_checked(context, wager_amount, accounts.token_mint.decimals)?;

    Ok(())
}