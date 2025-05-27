# Strunery – P2P Stablecoin Exchange on Solana (Phase 1)

**Strunery** is a decentralized P2P protocol built on **Solana** that facilitates trust-minimized stablecoin remittances. It is designed for the Latin American diaspora to seamlessly exchange USD and CAD-backed stablecoins (e.g., USDC/USDT) into Colombian Pesos (COP) using on-chain offer matching and off-chain fiat settlement.

> 🔁 Efficient.  
> 🛡️ Trust-minimized.  
> 🌍 Inclusive remittance rails powered by Solana.

---

## 📌 Motivation

Every year, billions of dollars are sent as remittances from North America to Latin America. However, these remittance flows are dominated by expensive centralized intermediaries. **Strunery** addresses this by:

- Leveraging **stablecoins** and **Solana’s low fees** to simplify cross-border value transfer.
- Enabling **peer-to-peer (P2P)** offers between senders abroad and receivers in Latin America.
- Promoting **stablecoin adoption** among underserved populations.

---

## 🚀 Functionality (Phase 1 MVP)

This initial MVP is built with [Anchor](https://github.com/coral-xyz/anchor) and deployed on Solana Playground. It provides basic offer creation and fulfillment logic.

### ✅ Create Offer
- A **maker** can create an offer with:
  - Offered amount & stablecoin (e.g., 100 USDC)
  - Requested fiat amount (e.g., 400,000 COP)
  - Offered currency: `USD`, `CAD`
  - Requested currency: `COP`
  - Offered token: `USDC`, `USDT`, or `SOL`

### ✅ Fulfill Offer
- A **taker** can fulfill the offer, marking it as completed.
- **Note:** Fiat settlement is done off-chain. No escrow logic implemented yet.

---

## 🔧 Technical Overview

### 📁 Program Logic (Rust with Anchor)

```rust
pub fn create_offer(ctx: Context<CreateOffer>, offered_amount: u64, requested_amount: u64, offered_currency: Currency, requested_currency: Currency, offered_token: TokenType) -> Result<()> {
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
    require!(!offer.is_fulfilled, ErrorCode::OfferAlreadyFulfilled);
    offer.is_fulfilled = true;
    Ok(())
}
