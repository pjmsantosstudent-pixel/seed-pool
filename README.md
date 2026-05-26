# SeedPool
<img width="1893" height="899" alt="Screenshot 2026-05-26 115851" src="https://github.com/user-attachments/assets/99489623-46e1-44f2-b072-5899f71a1211" />
> On-chain micro-lending for smallholder farmers to buy seeds and fertilizer — no bank required.

---

## Problem
A cassava farmer in Isabela, Philippines needs ₱3,000 for seeds at the start of planting season but has no credit history and the nearest rural bank is 40km away — so they borrow from a 5-6 lender (loan sharks) at 20% monthly interest, losing 30–40% of harvest income to repayment.

## Solution
SeedPool allows an NGO or farmer cooperative to issue USDC micro-loans directly to farmer wallets via a Soroban smart contract. Repayments are tracked transparently on-chain. No credit score needed — just a Stellar wallet. Loan history on-chain builds a decentralized credit record over time.

---

## Stellar Features Used
- USDC transfers — disbursement and repayment in stablecoin
- Soroban smart contracts — loan issuance, repayment tracking, duplicate-loan prevention
- Trustlines — farmer must hold USDC trustline before receiving funds
- Stellar's sub-cent fees — viable for loans as small as $5 USD

---

## Vision & Purpose
Rural lending in SEA is dominated by informal lenders charging 240%+ APR. SeedPool gives cooperatives and NGOs a zero-infrastructure lending tool that creates verifiable on-chain credit history for farmers — enabling them to eventually access DeFi credit markets.

---

## Timeline
| Phase | Duration |
|-------|----------|
| Contract + 5 tests | Day 1–2 |
| Testnet deploy | Day 3 |
| NGO dashboard UI | Day 4–5 |
| Live demo with 2 loan flows | Day 6 |

---

## Prerequisites
- Rust 1.74+
- Soroban CLI 21.x
- Admin Stellar account funded with USDC

---

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/seed_pool.wasm \
  --source <ADMIN_SECRET> \
  --network testnet

---

## Sample CLI Invocations

### Initialize
soroban contract invoke \
  --id <CONTRACT_ID> --source <ADMIN_SECRET> --network testnet \
  -- init --admin GADMINADDRESS

### Issue loan to farmer
soroban contract invoke \
  --id <CONTRACT_ID> --source <ADMIN_SECRET> --network testnet \
  -- issue_loan \
  --admin GADMINADDRESS \
  --farmer GFARMERADDRESS \
  --token GUSDC_ADDRESS \
  --amount 30000000

### Farmer repays
soroban contract invoke \
  --id <CONTRACT_ID> --source <FARMER_SECRET> --network testnet \
  -- repay \
  --farmer GFARMERADDRESS \
  --token GUSDC_ADDRESS \
  --amount 30000000

---

## Reference
- https://github.com/armlynobinguar/Stellar-Bootcamp-2026
- https://github.com/armlynobinguar/community-treasury

---

## License
MIT
