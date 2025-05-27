use anchor_lang::prelude::*;

declare_id!("HTReBUUkCxAJxZeGWCdngwzYvax4u4oyQy77wAT9UcG");

#[program]
pub mod strunery {
    use super::*;

    pub fn create_offer(
        ctx: Context<CreateOffer>,
        offered_amount: u64,
        requested_amount: u64,
        offered_currency: Currency,
        requested_currency: Currency,
        offered_token: TokenType,
    ) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        offer.maker = *ctx.accounts.maker.key;
        offer.offered_amount = offered_amount;
        offer.requested_amount = requested_amount;
        offer.offered_currency = offered_currency;
        offer.requested_currency = requested_currency;
        offer.offered_token = offered_token;
        offer.is_fulfilled = false;
        Ok(())
    }

    pub fn fulfill_offer(ctx: Context<FulfillOffer>) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        require!(!offer.is_fulfilled, OfferError::AlreadyFulfilled);
        offer.is_fulfilled = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    #[account(init, payer = maker, space = 8 + 128)]
    pub offer: Account<'info, Offer>,
    #[account(mut)]
    pub maker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillOffer<'info> {
    #[account(mut)]
    pub offer: Account<'info, Offer>,
}

#[account]
pub struct Offer {
    pub maker: Pubkey,
    pub offered_amount: u64,
    pub requested_amount: u64,
    pub offered_currency: Currency,
    pub requested_currency: Currency,
    pub offered_token: TokenType,
    pub is_fulfilled: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Currency {
    USD,
    CAD,
    COP,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TokenType {
    USDC,
    USDT,
    SOL,
}

#[error_code]
pub enum OfferError {
    #[msg("Offer has already been fulfilled.")]
    AlreadyFulfilled,
}
