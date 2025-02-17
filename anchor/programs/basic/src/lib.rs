use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConifg>, start: u64, end: u64, price: u64) -> Result<()> {
        Ok(())
    }

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
}

#[derive(Accounts)]
pub struct InitializeConifg<'info> {
}

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

#[account]
#[derive(InitSpace)]
pub struct TokenLottery {
}