use std::array;
use std::cell::RefMut;
use std::str::FromStr;
use anchor_lang::prelude::*;
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};



// Metadata Wallet

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
    fn id() -> Pubkey {
        MetadataTokenId
    }
}


// Personal Wallets

#[account(zero_copy)]
#[repr(C)]
pub struct AuthorityInfo {
    pub total_mints: i64,
    pub mintlist: [Pubkey; 128], 
}

// Token Auth

#[account(zero_copy)]
#[repr(C)]
pub struct WalletInfo {
    pub total_mints: i64,
    pub tokenlist: [Pubkey; 128], 
}

// PDA for Token (special to token)

#[account]
pub struct TokenInfo {
    pub permission: bool,
    pub locked: bool,
    pub token_mint: Pubkey,
    pub token_authority: Pubkey,
}



// State For Token

#[derive(Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Clone)]
pub enum TokenState {
    Secured,
    Free,
}
