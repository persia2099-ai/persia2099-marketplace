use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use mpl_core::instructions::TransferV1CpiBuilder;

declare_id!("4nwK6KMgGFAi1j363GSyi1vMfwQaL2iT5vELJJY6ZuHo");

pub const COLLECTION: Pubkey = pubkey!("6XGhHkPAHJ5XayXwn1p7t3XEUmyYEx3racfwnpbEwp6a");

#[program]
pub mod persia2099_escrow {
    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, price_lamports: u64) -> Result<()> {
        require!(price_lamports > 0, ErrorCode::InvalidPrice);
        require_keys_eq!(ctx.accounts.collection.key(), COLLECTION, ErrorCode::InvalidCollection);

        let seller_key = ctx.accounts.seller.key();
        let asset_key = ctx.accounts.asset.key();
        let collection_key = ctx.accounts.collection.key();
        let bump = ctx.bumps.listing;

        {
            let listing = &mut ctx.accounts.listing;
            listing.seller = seller_key;
            listing.asset = asset_key;
            listing.collection = collection_key;
            listing.price_lamports = price_lamports;
            listing.bump = bump;
        }

        let seeds: &[&[u8]] = &[b"listing", asset_key.as_ref(), &[bump]];

        TransferV1CpiBuilder::new(&ctx.accounts.mpl_core.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.seller.to_account_info())
            .authority(Some(&ctx.accounts.seller.to_account_info()))
            .new_owner(&ctx.accounts.listing.to_account_info())
            .system_program(Some(&ctx.accounts.system_program.to_account_info()))
            .invoke_signed(&[seeds])
            .map_err(|_| error!(ErrorCode::AssetTransferFailed))?;

        emit!(ListingCreated {
            seller: seller_key,
            asset: asset_key,
            price_lamports,
        });

        Ok(())
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        require_keys_eq!(ctx.accounts.listing.asset, ctx.accounts.asset.key(), ErrorCode::AssetMismatch);
        require_keys_eq!(ctx.accounts.listing.collection, ctx.accounts.collection.key(), ErrorCode::InvalidCollection);
        require_keys_eq!(ctx.accounts.listing.seller, ctx.accounts.seller.key(), ErrorCode::SellerMismatch);
        require!(ctx.accounts.buyer.key() != ctx.accounts.seller.key(), ErrorCode::SellerCannotBuy);

        let amount = ctx.accounts.listing.price_lamports;
        let bump = ctx.accounts.listing.bump;
        let asset_key = ctx.accounts.asset.key();

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.seller.to_account_info(),
                },
            ),
            amount,
        )?;

        let seeds: &[&[u8]] = &[b"listing", asset_key.as_ref(), &[bump]];

        TransferV1CpiBuilder::new(&ctx.accounts.mpl_core.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.buyer.to_account_info())
            .authority(Some(&ctx.accounts.listing.to_account_info()))
            .new_owner(&ctx.accounts.buyer.to_account_info())
            .system_program(Some(&ctx.accounts.system_program.to_account_info()))
            .invoke_signed(&[seeds])
            .map_err(|_| error!(ErrorCode::AssetTransferFailed))?;

        emit!(ListingSold {
            seller: ctx.accounts.seller.key(),
            buyer: ctx.accounts.buyer.key(),
            asset: asset_key,
            price_lamports: amount,
        });

        Ok(())
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        require_keys_eq!(ctx.accounts.listing.seller, ctx.accounts.seller.key(), ErrorCode::SellerMismatch);
        require_keys_eq!(ctx.accounts.listing.asset, ctx.accounts.asset.key(), ErrorCode::AssetMismatch);
        require_keys_eq!(ctx.accounts.listing.collection, ctx.accounts.collection.key(), ErrorCode::InvalidCollection);

        let bump = ctx.accounts.listing.bump;
        let asset_key = ctx.accounts.asset.key();
        let seeds: &[&[u8]] = &[b"listing", asset_key.as_ref(), &[bump]];

        TransferV1CpiBuilder::new(&ctx.accounts.mpl_core.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.seller.to_account_info())
            .authority(Some(&ctx.accounts.listing.to_account_info()))
            .new_owner(&ctx.accounts.seller.to_account_info())
            .system_program(Some(&ctx.accounts.system_program.to_account_info()))
            .invoke_signed(&[seeds])
            .map_err(|_| error!(ErrorCode::AssetTransferFailed))?;

        emit!(ListingCancelled {
            seller: ctx.accounts.seller.key(),
            asset: asset_key,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(price_lamports: u64)]
pub struct CreateListing<'info> {
    #[account(init, payer = seller, space = Listing::SPACE, seeds = [b"listing", asset.key().as_ref()], bump)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    #[account(address = COLLECTION)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub seller: Signer<'info>,
    /// CHECK: Fixed Metaplex Core program account; the CPI validates the program id.
    pub mpl_core: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut, seeds = [b"listing", asset.key().as_ref()], bump = listing.bump, close = seller)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    #[account(address = COLLECTION)]
    pub collection: UncheckedAccount<'info>,
    /// CHECK: Seller is authenticated by the listing's stored seller key.
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Fixed Metaplex Core program account; the CPI validates the program id.
    pub mpl_core: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut, seeds = [b"listing", asset.key().as_ref()], bump = listing.bump, close = seller)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    #[account(address = COLLECTION)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub seller: Signer<'info>,
    /// CHECK: Fixed Metaplex Core program account; the CPI validates the program id.
    pub mpl_core: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub asset: Pubkey,
    pub collection: Pubkey,
    pub price_lamports: u64,
    pub bump: u8,
}

impl Listing {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 8 + 1;
}

#[event]
pub struct ListingCreated {
    pub seller: Pubkey,
    pub asset: Pubkey,
    pub price_lamports: u64,
}

#[event]
pub struct ListingSold {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub asset: Pubkey,
    pub price_lamports: u64,
}

#[event]
pub struct ListingCancelled {
    pub seller: Pubkey,
    pub asset: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Price must be greater than zero.")]
    InvalidPrice,
    #[msg("Invalid Persia 2099 collection.")]
    InvalidCollection,
    #[msg("Asset does not match the listing.")]
    AssetMismatch,
    #[msg("Seller does not match the listing.")]
    SellerMismatch,
    #[msg("Seller cannot buy their own asset.")]
    SellerCannotBuy,
    #[msg("Core asset transfer failed.")]
    AssetTransferFailed,
}
