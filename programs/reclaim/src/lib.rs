#![allow(unknown_lints)]
#![allow(ambiguous_glob_reexports)]

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("CGTjkfCkFqEPhp28aBK6afd2SaqeVTju1pdYZzdrX3dn");

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

#[program]
pub mod reclaim {
    use super::*;

    pub fn initialize_epoch_config(
        ctx: Context<InitializeEpochConfig>,
        args: InitializeEpochConfigArgs,
    ) -> Result<()> {
        epoch::initialize(ctx, args)
    }

    pub fn change_epoch_index_epoch_config(
        ctx: Context<ChangeEpochIndexEpochConfig>,
        args: ChangeEpochIndexEpochConfigArgs,
    ) -> Result<()> {
        epoch::change_epoch_index(ctx, args)
    }

    pub fn add_epoch(ctx: Context<AddEpoch>, args: AddEpochArgs) -> Result<()> {
        epoch::add(ctx, args)
    }

    pub fn create_group(ctx: Context<CreateGroup>, args: CreateGroupArgs) -> Result<()> {
        group::create(ctx, args)
    }

    // pub fn verify_proof(ctx: Context<VerifyProof>, args: VerifyProofArgs) -> Result<()> {
    //     group::verify_proof(ctx, args)
    // }

    pub fn create_dapp(ctx: Context<CreateDapp>, args: CreateDappArgs) -> Result<()> {
        dapp::create(ctx, args)
    }

    pub fn create_challenge(ctx: Context<Initialize>, challenge_id: String, wager_amount:u64) -> Result<()> {
        wager::initialize::init_challenge(ctx, challenge_id, wager_amount)?;
        Ok(())
    }

    pub fn join_challenge(ctx: Context<JoinChallenge>, challenge_id: String, wager_amount:u64) -> Result<()> {
        wager::join::join_challenge(ctx, challenge_id, wager_amount)?;
        Ok(())
    }

    pub fn settle_wager(ctx: Context<Settle>, challenge_id: String, args: VerifyProofArgs) -> Result<()> {
        wager::settle::settle_challenge_and_verify_proof(ctx, challenge_id, args)?;
        Ok(())
    }
}
