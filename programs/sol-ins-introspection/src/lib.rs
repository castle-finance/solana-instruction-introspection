use anchor_lang::{prelude::*, solana_program::{sysvar::instructions::{ID as instructions_id, load_current_index_checked, load_instruction_at_checked, check_id as check_instruction_id}}, error::ErrorCode::StateInvalidAddress};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const UPDATE_TIMESTAMP_OPCODE: u64 = 0x1; 

#[program]
pub mod sol_ins_introspection {
    use std::convert::TryInto;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hi?");
        let ref mut basic_state = ctx.accounts.state_account;

        let time_stamp = Clock::get()?.unix_timestamp;

        msg!("Initializing state account of address: {}", &basic_state.key());

        basic_state.bump = *ctx.bumps.get("state_account").unwrap();
        basic_state.value = 0;
        basic_state.timestamp = time_stamp as u64;
        basic_state.authority = ctx.accounts.authority.key();
        
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let ref mut state_account = ctx.accounts.state_account;

        let instruction_sysvar = &ctx.accounts.instruction_sysvar.to_account_info();

        require!(check_instruction_id(&instruction_sysvar.key()), StateInvalidAddress);

        let current_instruction_index = load_current_index_checked(instruction_sysvar)?;

        msg!("Current instruction index {}", current_instruction_index);

        let instruction = load_instruction_at_checked((current_instruction_index + 1).into(), instruction_sysvar)?;

        let instruction_pubkeys = instruction.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<String>>();

        let op_code_le = u64::from_le_bytes(instruction.data[..8].try_into().unwrap());
        let op_code_be = u64::from_be_bytes(instruction.data[..8].try_into().unwrap());

        msg!("OP CODE BE {}\n OP CODE LE {}", op_code_be, op_code_le);

        msg!("instruction data {:?}\n instruction program_id {} \n instruction accounts {:?}", instruction.data, instruction.program_id, instruction_pubkeys);

        let new_value = state_account.value.checked_add(1).ok_or(ProgramError::InvalidAccountData)?;

        msg!("Increment state address' value from {} to {}", state_account.value, new_value);

        state_account.value = new_value;
        
        Ok(())
    }

    pub fn update_timestamp(ctx: Context<UpdateTimestamp>) -> Result<()> {
        let ref mut state_account = ctx.accounts.state_account;
        let current_timestamp = Clock::get()?.unix_timestamp;
        msg!("Updating checkpoint timestamp {}", current_timestamp);
        state_account.timestamp = current_timestamp as u64;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 8 + 1 + 1 + 32,
        seeds = [b"basic", authority.key().as_ref()],
        bump
    )]
    state_account: Account<'info, BasicState>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        seeds = [b"basic", authority.key().as_ref()],
        bump = state_account.bump,
        has_one = authority
    )]
    state_account: Account<'info, BasicState>,
    #[account(mut)]
    authority: Signer<'info>,
    /// CHECK: This is a Instructions Sysvar and proper checks has been made
    #[account(address = instructions_id @ StateInvalidAddress)]
    instruction_sysvar: UncheckedAccount<'info>
}

#[derive(Accounts)]
pub struct UpdateTimestamp<'info> {
    #[account(
        seeds = [b"basic", authority.key().as_ref()],
        bump = state_account.bump,
        has_one = authority
    )]
    state_account: Account<'info, BasicState>,
    #[account(mut)]
    authority: Signer<'info>
}

#[account]
#[derive(Debug)]
pub struct BasicState {
    timestamp: u64,
    value: u8,
    bump: u8,
    authority: Pubkey,
}
