use anchor_lang::prelude::*;

declare_id!("EBeVoDeji7v5X581ifYqgPW7bEuJ6wbwFyuCwShUAhBK");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
