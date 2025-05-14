use anchor_lang::prelude::*;
use integer_sqrt::IntegerSquareRoot;

declare_id!("G1iGih76uXLN8Bmht3Jp4rPZUUFDK5ok1eeNwMY6AXiB");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said: {:?}", message);
        msg!("You sent: {} and {}", a, b);
        Ok(())
    }

    // pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
    //     msg!("Your array: {:?}", arr);
    //     Ok(())
    // }

    // pub fn sub(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
    //     msg!("The answer to 0 - 1 is { }" , 0 - 1);
    //     Ok(())
    // }

    pub fn add(ctx: Context<Initialize> , a: u64, b:u64) -> Result<()> {
        let result = a.checked_add(b).unwrap();
        msg!("The answer to {} + {} is {}", a, b, result);
        Ok(())

    }

    pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.wrapping_sub(b); // Explicit underflow-safe subtraction
        msg!("The answer to {} - {} is {}", a, b, result);
        Ok(())
    }

    pub fn mul(ctx: Context<Initialize> , a: u64, b:u64) -> Result<()> {
        let result = a.checked_mul(b).unwrap();
        msg!("The answer to {} * {} is {}", a, b, result);
        Ok(())

    }

    pub fn div(ctx: Context<Initialize> , a: u64, b:u64) -> Result<()> {
        let result = a.checked_div(b).unwrap();
        msg!("The answer to {} / {} is {}", a, b, result);
        Ok(())

    }


    pub fn sqrt(_ctx: Context<Initialize>, a: u64) -> Result<()> {
        let result = a.integer_sqrt();
        msg!("The answer to Square Root of {} is {}", a, result);
        Ok(())
    }

    pub fn log10(_ctx: Context<Initialize>, a: u64) -> Result<()> {
        require!(a > 0, CustomError::LogOfZero);
        let mut result = 0;
        let mut value = a;
        while value >= 10 {
            value /= 10;
            result += 1;
        }
        msg!("The base-10 logarithm of {} is {}", a, result);
        Ok(())
    }
    
    
}

#[derive(Accounts)]
pub struct Initialize {}


#[error_code]
pub enum CustomError {
    #[msg("Logarithm of zero is undefined")]
    LogOfZero,
}