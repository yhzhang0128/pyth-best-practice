// crate dependencies are specified in Cargo.toml
use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, PriceStatus};

// TODO: understand how to obtain the program ID
declare_id!("Emb6QWJDK8RVzvH3CgoQz92R8mmcBfC5tBq5wsSJXDZa");

#[program]
pub mod status {
    // anchor_lang::prelude::super contains commonly used components of Anchor
    // see https://docs.rs/anchor-lang/latest/anchor_lang/prelude/index.html
    use super::*;

    // Context is defined in Anchor; Run is defined below
    pub fn onchain(ctx: Context<Run>) -> Result<()> {
        // load_price_feed_from_account_info() will construct a PriceFeed struct
        // see https://crates.io/crates/pyth-sdk-solana
        let price_feed: PriceFeed = load_price_feed_from_account_info(&ctx.accounts.price).unwrap();

        // Pyth return value is only valid if status is Trading
        if price_feed.status == PriceStatus::Trading {
            ctx.accounts.output.trading = true as u8;
        } else {
            ctx.accounts.output.trading = false as u8;
        }

        //TODO: return the price in addition to status
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(init, payer = funding, space = 9)]
    pub output: Account<'info, Output>,
    /// CHECK: Pyth does not use Anchor
    pub price: UncheckedAccount<'info>,
    #[account(mut)]
    pub funding: Signer<'info>,
    #[account()]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Output {
    trading: u8,
}
