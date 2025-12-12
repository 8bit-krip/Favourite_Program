use anchor_lang::prelude::*;

declare_id!("G9LkvYjuoF8voAr1TZ8ha1nnq1ixZPnbK6yJ9qGRnocF");

#[program]
pub mod favourite {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
