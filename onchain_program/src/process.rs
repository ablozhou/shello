use solana_program::{
    account_info::next_account_info,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    msg,
    program_pack::{Pack},
    program_error::ProgramError,

};
use crate::{
    instruction::HelloWorldInstruction,
    state::HelloWorldState
};

// 从runtime过来要处理的instruction进行反序列化操作：
/// Processes an [Instruction](enum.Instruction.html).
/// Program state handler.
pub struct Processor {}
impl Processor {
     /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = HelloWorldInstruction::unpack(input)?;
        //然后根据前面介绍的反序列化后的instruction来进行分别的处理：
        match instruction {
            HelloWorldInstruction::Hello {
                message,
            } => {
                msg!(&format!("hello-world: HelloWorld msg:{}", message));
                Self::process_hello(accounts, message)
            },
            HelloWorldInstruction::Erase=>{
                msg!("hello-world: Erase");
                Self::process_erase(accounts)
            }
        }
    }

    //对于Hello，我们的处理就是将消息内容和谁发的信息，进行记录：
    /// Processes an [Hello](enum.HelloWorldInstruction.html) instruction.
    fn process_hello(
        accounts: &[AccountInfo],
        message: String,
    ) -> ProgramResult {
        // 采用sdk函数获取账号信息
        let account_info_iter = &mut accounts.iter();
        let client_info = next_account_info(account_info_iter)?;
        let message_info = next_account_info(account_info_iter)?;
        // check permission
        if !client_info.is_signer || !message_info.is_signer{
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        msg!("before unpack hello");
        let mut state = HelloWorldState::unpack_unchecked(&message_info.data.borrow())?;
        msg!("after unpack hello");
        state.account_key = *client_info.key;
        state.message = message;
        
        msg!("before pack hello");
        HelloWorldState::pack(state, &mut message_info.data.borrow_mut())?;
        msg!("after pack hello");
        Ok(())
    }
     /// Processes a [Erase](enum.HelloWorldInstruction.html) instruction.
    fn process_erase(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let client_info = next_account_info(account_info_iter)?;
        let message_info = next_account_info(account_info_iter)?;


        //check permission
        if !client_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let client_starting_lamports = client_info.lamports();
        **client_info.lamports.borrow_mut() = client_starting_lamports + message_info.lamports();
        **message_info.lamports.borrow_mut() = 0;
        Ok(())
    }
}