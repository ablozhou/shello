use crate::{error::HelloWorldError, process::Processor};
use solana_program::{account_info::AccountInfo,pubkey::Pubkey,entrypoint,
entrypoint::ProgramResult,program_error::PrintProgramError};

// entrypoint is  all programs entry:
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,  // address of the program
    accounts: &[AccountInfo],  // runtime resolve keys and got the accounts.
    instruction_data: &[u8],   // params of the program
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<HelloWorldError>();
        return Err(error);
    }
    Ok(())
}