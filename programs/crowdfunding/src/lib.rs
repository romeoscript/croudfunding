use anchor_lang::prelude::*;

declare_id!("9SSgp385BzpQYznpPczDWZzswNDUAuiD957DBmFmvKr5");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
