# SolPaywall 

> Frictionless Micro-Monetization & Instant Content Paywalls powered by Solana Actions & Blinks.  
> Developed as a capstone student project for **Solana Startup Terminal by Superteam Ukraine**.

---

## Overview

Web2 micropayments fail because card processing minimums ($0.30 + 3%) eliminate sub-dollar purchases.  
**SolPaywall** enables creators to sell access to individual articles, code snippets, or research for as low as $0.05 / 0.001 SOL with 1-click execution inside X (Twitter) feeds.

### Key Features
- **Solana Blinks Support:** Readers unlock content directly in their Twitter/X timeline without opening an external dApp.
- **Anchor Smart Contract:** Transparent non-custodial paywall management and proof-of-access receipts.
- **Sub-Second Settlement:** Transaction confirmation in <600ms on Solana Devnet/Mainnet.
- **No Subscriptions Required:** Readers only pay for the exact content they consume.

---

## Tech Stack
- **Smart Contract:** Rust, Anchor Framework
- **Blinks / Actions:** `@solana/actions`, `@solana/web3.js`
- **Frontend / API:** Next.js 14, TypeScript
- **Network:** Solana Devnet / Mainnet

---

## Quickstart

### 1. Clone repository
```bash
git clone https://github.com/TrinityWeb3/solpaywall.git
cd solpaywall
```

###  2. Install dependencies
```Bash
npm install
```
### 3. Run Dev Server (Actions API)
```Bash
npm run dev
```
### 4. Build Smart Contract (Anchor)
```Bash
anchor build
anchor test
```
