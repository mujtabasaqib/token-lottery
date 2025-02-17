use anchor_lang::prelude::*;

declare_id!("95bGSewocVFv6g3TWZqSvntKYu61nemkvkLZxrET1THB");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConifg>, start: u64, end: u64, price: u64) -> Result<()> {
        ctx.accounts.token_lottery.bump = ctx.bumps.token_lottery;
        ctx.accounts.token_lottery.lottery_start = start;
        ctx.accounts.token_lottery.lottery_end = end;
        ctx.accounts.token_lottery.price = price;
        ctx.accounts.token_lottery.authority = ctx.accounts.payer.key();
        ctx.accounts.token_lottery.randomness_account = Pubkey::default();

        ctx.accounts.token_lottery.ticket_num = 0;
        ctx.accounts.token_lottery.winner_chosen = false;
        Ok(())
    }
    /*

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        Ok(())
    }

    pub fn commit_a_winner(ctx: Context<CommitWinner>) -> Result<()> {
        Ok(())
    }

    pub fn choose_a_winner(ctx: Context<ChooseWinner>) -> Result<()> {
        Ok(())
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        Ok(())
    }

    */
}

#[derive(Accounts)]
pub struct InitializeConifg<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + TokenLottery::INIT_SPACE,
        // Challenge: Make this be able to run more than 1 lottery at a time?
        seeds = [b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Box<Account<'info, TokenLottery>>,

    pub system_program: Program<'info, System>,
}
/* 

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
}

#[derive(Accounts)]
pub struct CommitWinner<'info> {
}

#[derive(Accounts)]
pub struct ChooseWinner<'info> {
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
}

*/

#[account]
#[derive(InitSpace)]
pub struct TokenLottery {
    pub bump: u8,
    pub winner: u64,
    pub winner_chosen: bool,
    pub lottery_start: u64,
    pub lottery_end: u64,
    // Is it good practice to store SOL on an account used for something else?
    pub lottery_pot_amount: u64,
    pub ticket_num: u64,
    pub price: u64,
    pub randomness_account: Pubkey,
    pub authority: Pubkey,
}