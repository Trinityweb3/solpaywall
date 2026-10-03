use anchor_lang::prelude::*;

declare_id!("SPayw1111111111111111111111111111111111111");

#[program]
pub mod solpaywall {
    use super::*;

    pub fn initialize_paywall(
        ctx: Context<InitializePaywall>,
        content_id: String,
        price_lamports: u64,
        target_url: String,
    ) -> Result<()> {
        let paywall = &mut ctx.accounts.paywall;
        paywall.creator = ctx.accounts.creator.key();
        paywall.content_id = content_id;
        paywall.price_lamports = price_lamports;
        paywall.target_url = target_url;
        paywall.total_unlocks = 0;
        paywall.bump = ctx.bumps.paywall;
        Ok(())
    }

    pub fn unlock_content(ctx: Context<UnlockContent>) -> Result<()> {
        let paywall = &mut ctx.accounts.paywall;

        // Transfer funds directly from reader to creator
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.reader.key(),
            &paywall.creator,
            paywall.price_lamports,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.reader.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Record receipt
        let receipt = &mut ctx.accounts.receipt;
        receipt.paywall = paywall.key();
        receipt.reader = ctx.accounts.reader.key();
        receipt.timestamp = Clock::get()?.unix_timestamp;
        paywall.total_unlocks += 1;

        emit!(ContentUnlocked {
            paywall: paywall.key(),
            reader: ctx.accounts.reader.key(),
            timestamp: receipt.timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(content_id: String)]
pub struct InitializePaywall<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + (4 + 64) + 8 + (4 + 256) + 8 + 1,
        seeds = [b"paywall", creator.key().as_ref(), content_id.as_bytes()],
        bump
    )]
    pub paywall: Account<'info, PaywallConfig>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockContent<'info> {
    #[account(mut)]
    pub paywall: Account<'info, PaywallConfig>,
    #[account(mut)]
    pub reader: Signer<'info>,
    /// CHECK: Validated against paywall.creator
    #[account(mut, address = paywall.creator)]
    pub creator: AccountInfo<'info>,
    #[account(
        init,
        payer = reader,
        space = 8 + 32 + 32 + 8,
        seeds = [b"receipt", paywall.key().as_ref(), reader.key().as_ref()],
        bump
    )]
    pub receipt: Account<'info, AccessReceipt>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaywallConfig {
    pub creator: Pubkey,
    pub content_id: String,
    pub price_lamports: u64,
    pub target_url: String,
    pub total_unlocks: u64,
    pub bump: u8,
}

#[account]
pub struct AccessReceipt {
    pub paywall: Pubkey,
    pub reader: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ContentUnlocked {
    pub paywall: Pubkey,
    pub reader: Pubkey,
    pub timestamp: i64,
}
