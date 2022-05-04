use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod level_1 {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       
        let account = &mut ctx.accounts.my_account_buffer;
        
        account.data = 0;      
        
        Ok(())
    }

    pub fn increment_by_ten(ctx: Context<IncrementByTen>) -> Result<()>{
        
        let account = &mut ctx.accounts.my_account_buffer;

        account.data += 10;
        
        Ok(())
    }
    

    pub fn set_data(ctx: Context<SetData>, data: u8) -> Result<()>{

        let account = &mut ctx.accounts.my_account_buffer;

        account.data = data;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
#[account(init, payer = authority, space = 48)] 
pub my_account_buffer: Account<'info, MyAccountBuffer>,
#[account(mut)]
pub authority: Signer<'info>,
pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct IncrementByTen<'info> {
    
    #[account(mut)]
    pub my_account_buffer: Account<'info, MyAccountBuffer>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetData<'info>{
#[account(mut)]
pub my_account_buffer: Account<'info, MyAccountBuffer>,
/// CHECK: using AccountInfo
pub authority: AccountInfo<'info>
}

#[account]
pub struct MyAccountBuffer{
    pub authority: Pubkey,
    pub data: u8,
}

