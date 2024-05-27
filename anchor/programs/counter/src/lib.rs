#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8vVva8Qz6LeZWRkTJCz8d3BMX3Cwv6XXk4gotpePoYLP");

#[program]
pub mod crud_app {
    use super::*;

    pub fn create_entry(
      ctx: Context<CreateEntry>,
      title:String,
      message:String,
    ) -> Result<()>{
      let entry = &mut ctx.accounts.entry;
      entry.owner = ctx.accounts.owner.key();
      entry.title = title;
      entry.message = message;
      Ok(())
    }

    pub fn update_entry(
      ctx: Context<UpdateEntry>,
      _title: String,
      new_message: String,
    ) -> Result<()>{
      let entry = &mut ctx.accounts.entry;
      entry.message = new_message;
      Ok(())
    }

    pub fn delete_entry(
      _ctx: Context<DeleteEntry>,
      _title: String,
    ) -> Result<()>{
      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info>{
  #[account(
    init,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    payer = owner,
    space = 8 + Entry::INIT_SPACE,
  )]
  pub entry:Account<'info, Entry>,
  #[account(mut)]
  pub owner:Signer<'info>,
  pub system_program:Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateEntry<'info>{
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    realloc = 8 + Entry::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true,
  )]
  pub entry:Account<'info, Entry>,
  #[account(mut)]
  pub owner:Signer<'info>,
  pub system_program:Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struc DeleteEntry<'info>{
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner,
  )]
  pub entry:Account<'info, Entry>,
  #[account(mut)]
  pub owner:Signer<'info>,
  pub system_program:Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Entry{
  pub owner:Pubkey,
  pub entry_id:u64,
  #[max_len(20)]
  pub title:String,
  #[max_len(200)]
  pub message:String,
}
