use anchor_lang::prelude::*;
use crate::error::*;
use crate::utils::*;
use crate::state::*;
use anchor_spl::{
    token_interface::{ 
    Mint, 
    TokenAccount, 
    transfer_checked, 
    TransferChecked, 
    TokenInterface, 
    CloseAccount, 
    close_account 
}, 
associated_token::AssociatedToken};
use serde_json::Value;
use std::boxed::Box;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(challenge_id: String, args: VerifyProofArgs)]
pub struct Settle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub creator: SystemAccount<'info>,

    #[account(mut)]
    pub opponent: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_CHALLENGE, challenge_id.as_bytes()],
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
        associated_token::authority = opponent,
        associated_token::token_program = token_program
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = challenge,
        associated_token::token_program = token_program,
    )]
    pub program_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            SEED_PREFIX,
            epoch_config.key().as_ref(),
            SEED_EPOCH,
            &args.signed_claim.claim_data.epoch_index.to_le_bytes()
        ],
        bump = epoch.bump,
        has_one = epoch_config @ ReclaimError::Unauthorized,
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(
        seeds = [
            SEED_PREFIX,
            SEED_EPOCH_CONFIG,
            epoch_config.create_key.as_ref(),
        ],
        bump = epoch_config.bump,
    )]
    pub epoch_config: Account<'info, EpochConfig>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn settle_challenge_and_verify_proof(ctx: Context<Settle>, challenge_id: String, args: VerifyProofArgs) -> Result<()> {

    let challenge = &mut ctx.accounts.challenge;
    require!(!challenge.result_settled, WagerError::WagerAlreadySettled);

    let epoch = &ctx.accounts.epoch;

    let SignedClaim {
        claim_data,
        signatures,
    } = args.signed_claim;

    let received_identifier = append_0x(&hex::encode(claim_data.identifier));
    let expected_identifier = hash_claim_info(&args.claim_info);

    require!(
        received_identifier.eq(&expected_identifier),
        ReclaimError::InvalidIdentifier
    );

    let minimum_witnesses = usize::from(epoch.minimum_witnesses_for_claim);
    let witness_serialized_data = claim_data.serialize_for_witness(minimum_witnesses);

    let hashed_witness_serialized_data = hash_content(&witness_serialized_data);

    let selected_witnesses = select_witness_addresses(
        minimum_witnesses,
        &hashed_witness_serialized_data,
        &epoch.witnesses,
    )?;

    msg!("Selected Witnesses: {:?}", selected_witnesses);

    /* Recovering witnesses from signatures */
    let recovery_serialized_data = claim_data.serialize_for_recovery();
    let hashed_recovery_serialized_data = prepare_for_verification(&recovery_serialized_data);

    let recovered_witnesses =
        recover_witness_addresses(&hashed_recovery_serialized_data, &signatures)?;

    msg!("Recovered Witnesses: {:?}", recovered_witnesses);

    /* Checking selected vs recovered witnesses */
    for recovered_witness in recovered_witnesses {
        require!(
            selected_witnesses.contains(&recovered_witness),
            ReclaimError::InvalidWitnessSignature
        );
    }

    let json_data = parse_large_json(&args.claim_info.parameters);

    // Extract player results from JSON
    let white_result = json_data["responseMatches"][1]["value"].as_str();
    let black_result = json_data["responseMatches"][2]["value"].as_str();

    let winner = match (white_result, black_result) {
        (Some(wr), Some(_br)) if wr.contains("\"result\":\"win\"") => Some(challenge.creator),
        (Some(_wr), Some(br)) if br.contains("\"result\":\"win\"") => Some(challenge.opponent.unwrap()),
        _ => None, // Draw case
    };

    if let Some(winner_pubkey) = winner {
        require!(
            challenge.creator == winner_pubkey || 
            challenge.opponent.map_or(false, |o| o == winner_pubkey),
            WagerError::ThirdPersonWinner
        );
    }

    challenge.result_settled = true;
    challenge.winner = winner;

    // Distribute winnings
    distribute_winnings(ctx, challenge_id, winner)?;

    Ok(())

}

fn parse_large_json(json_str: &str) -> Box<Value> {
    let parsed_json: Value = serde_json::from_str(json_str).unwrap();
    Box::new(parsed_json) 
}

fn distribute_winnings(ctx: Context<Settle>, challenge_id: String, winner: Option<Pubkey>) -> Result<()> {
    let challenge_id_bytes = challenge_id.as_bytes();
    let seeds = &[
        b"challenge",
        challenge_id_bytes,
        &[ctx.accounts.challenge.challenge_bump],
    ];
    let signer_seeds = &[&seeds[..]];

    match winner {
        Some(winner_pubkey) => {
            let payout = ctx.accounts.program_token_account.to_account_info().lamports();
            msg!("Transferring {} lamports to winner", payout);
            let winner_account = if winner_pubkey == ctx.accounts.challenge.creator {
                ctx.accounts.creator.to_account_info()
            } else {
                ctx.accounts.opponent.to_account_info()
            };

            let transfer_accounts = TransferChecked {
                from: ctx.accounts.program_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: winner_account,
                authority: ctx.accounts.challenge.to_account_info(),
            };

            let context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_accounts, signer_seeds);
            transfer_checked(context, payout, ctx.accounts.token_mint.decimals)?;

            msg!("Transfer to winner successful");
        }
        None => {
            let per_amount = ctx.accounts.program_token_account.to_account_info().lamports() / 2;

            let transfer_accounts_creator = TransferChecked {
                from: ctx.accounts.program_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.challenge.to_account_info(),
            };

            let transfer_accounts_opponent = TransferChecked {
                from: ctx.accounts.program_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.opponent_token_account.to_account_info(),
                authority: ctx.accounts.challenge.to_account_info(),
            };

            let creator_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_accounts_creator, signer_seeds);
            let opponent_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_accounts_opponent, signer_seeds);

            transfer_checked(creator_context, per_amount, ctx.accounts.token_mint.decimals)?;
            transfer_checked(opponent_context, per_amount, ctx.accounts.token_mint.decimals)?;

            msg!("Draw: funds split equally");
        }
     
    }

    let accounts = CloseAccount {
        account: ctx.accounts.program_token_account.to_account_info(),
        destination: ctx.accounts.creator.to_account_info(),
        authority: ctx.accounts.challenge.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    close_account(cpi_context)?;  

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyProofArgs {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}
