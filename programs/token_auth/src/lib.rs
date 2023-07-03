
use anchor_lang::prelude::*;

use anchor_lang::solana_program::system_program;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};

use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};




use std::str::FromStr;

pub mod account;
pub mod constants;
pub mod errors;

use account::*; 
use errors::*;

declare_id!("DQHN7u5tejZPTCLmqaH8HqkLMXHx83CZbzyv7BAbhn8D");

#[program]
pub mod token_auth {
    use super::*;

    pub fn secure_token(ctx: Context<Secure>, auth: Pubkey) -> Result<()> {
        msg!("Secure Token");
        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nft_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;

        msg!("Freezing token account");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        anchor_lang::solana_program::program::invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_token_account.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_token_account.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.metadata_program.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;

        Ok(())
    }



    pub fn revoke_token(ctx: Context<RevokeAuthority>) -> Result<()> {

        msg!("Revoke Token Access");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auth: Pubkey)]
pub struct Secure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
        /// CHECK: manual check
    #[account(seeds = ["mint".as_bytes().as_ref()], bump)]
    pub stake_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Test
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,


    // Add Pdas. 
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<TokenInfo>() + 8,
        seeds = [user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub token_account_pda: Account<'info, TokenInfo>,

    #[account(
        init_if_needed,
        payer=user,
        space = 4 * 1024 as usize,
        seeds = [user.key().as_ref(), b"token_auth".as_ref()],
        bump
    )]
    pub user_pda: AccountLoader<'info, AuthorityInfo>,

    #[account(
        init_if_needed,
        payer=user,
        space = 4 * 1024 as usize,
        seeds = [auth.to_bytes().as_ref(), b"token_auth".as_ref()],
        bump
    )]
    pub authority_pda: AccountLoader<'info, AuthorityInfo>,



    /// CHECK: Just authority
    #[account(mut, seeds=["authority".as_bytes().as_ref()], bump)]
    pub program_authority: UncheckedAccount<'info>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,



}

#[derive(Accounts)]
pub struct GivePermRevoke<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
        /// CHECK: manual check
    #[account(seeds = ["mint".as_bytes().as_ref()], bump)]
    pub stake_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Test
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<TokenInfo>() + 8,
        seeds = [user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub token_account_pda: Account<'info, TokenInfo>,

    #[account(
        init_if_needed,
        payer=user,
        space = 4 * 1024 as usize,
        seeds = [user.key().as_ref(), b"token_auth".as_ref()],
        bump
    )]
    pub user_pda: AccountLoader<'info, AuthorityInfo>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(auth: Pubkey)]
pub struct RevokeAuthority<'info> {    
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
        /// CHECK: manual check
    #[account(seeds = ["mint".as_bytes().as_ref()], bump)]
    pub stake_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Test
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,


    // Add Pdas. 
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<TokenInfo>() + 8,
        seeds = [user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub token_account_pda: Account<'info, TokenInfo>,

    #[account(
        init_if_needed,
        payer=user,
        space = 4 * 1024 as usize,
        seeds = [user.key().as_ref(), b"token_auth".as_ref()],
        bump
    )]
    pub user_pda: AccountLoader<'info, AuthorityInfo>,

    #[account(
        init_if_needed,
        payer=user,
        space = 4 * 1024 as usize,
        seeds = [auth.to_bytes().as_ref(), b"token_auth".as_ref()],
        bump
    )]
    pub authority_pda: AccountLoader<'info, AuthorityInfo>,



    /// CHECK: Just authority
    #[account(mut, seeds=["authority".as_bytes().as_ref()], bump)]
    pub program_authority: UncheckedAccount<'info>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,
}

