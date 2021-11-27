/// Instructions supported by the hello-world program.
use core::str::from_utf8;
use crate::error::HelloWorldError;
use solana_program::program_error::ProgramError;

/// Instructions supported by the hello-world program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum HelloWorldInstruction {
    /// Hello print hello to an Account file
    Hello{
        /// message for hello
        message: String,
    },
    /// Erase free the hello account 
    Erase ,
}
/*
```
            +-----------------------------------+
Hello:      |   0    |       message            |
            +-----------------------------------+
            +--------+
Erase:      |   1    |
            +--------+
```
*/
impl HelloWorldInstruction {
    /// Unpacks a byte buffer into a [HelloWorldInstruction](enum.HelloWorldInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use HelloWorldError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag { //HelloWorld
            0 => {
                let message= String::from(from_utf8(rest).unwrap());
                Self::Hello{
                    message,
                }
            },
            1 => Self::Erase,
            _ => return Err(HelloWorldError::InvalidInstruction.into()),
        })
    }
}