use anchor_lang::prelude::*;

declare_id!("G9LkvYjuoF8voAr1TZ8ha1nnq1ixZPnbK6yJ9qGRnocF");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourite {
    use super::*;

    pub fn set_favourite(
        ctx: Context<SetFavourite>,
        number: u64,
        colour: String,
        hobbies: Vec<String>,
        animal: String,
    ) -> Result<()> {

        
        msg!("Program ID: {}", ctx.program_id);

        let user_public_key = ctx.accounts.user.key();

        msg!(
            "User {} has favourite number {}, favourite colour {}, hobbies {:?}, and favourite animal {}",
            user_public_key,
            number,
            colour,
            hobbies,
            animal
        );

        
        ctx.accounts.favourite_account.set_inner(FavouriteData {
            number,
            colour,
            hobbies,
            animal,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct FavouriteData {
    pub number: u64,

    #[max_len(50)]
    pub colour: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>, 

    #[max_len(50)]
    pub animal: String,
}

#[derive(Accounts)]
pub struct SetFavourite<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + FavouriteData::INIT_SPACE,
        seeds = [b"IITROORKEE", user.key().as_ref()],
        bump
    )]
    pub favourite_account: Account<'info, FavouriteData>,

    pub system_program: Program<'info, System>,
}
