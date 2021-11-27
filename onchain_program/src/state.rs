use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::str::from_utf8;
/// HelloWorld data.
//传递结构体类型的参数时，可以使用 repr 属性 # [repr (C)] 确保有一致的内存布局。
#[repr(C)]
//PartialEq 和 Eq这两个 Traits 的名称实际上来自于抽象代数中的等价关系和局部等价关系，
//实际上两者的区别仅有一点，即是否在相等比较中是否满足反身性（Reflexivity）
//并非所有的二元关系都是等价关系， Eq 和 PartialEq 的区别在于是否在相等比较中是否满足自反性，即 x == x
//对于浮点类型，Rust 只实现了 PartialEq 而不是 Eq，原因就是 NaN != NaN。
//当一个类型同时实现了 Eq 和 Hash 时，该类型满足下列特性：

//k1 == k2 -> hash(k1) == hash(k2)
//即，当两个 key 相等时，它们的哈希值必然相等。Rust 里的 HashMap 和 HashSet 都依赖该特性。

//Rust 在很多地方使用了 traits, 从非常浅显的操作符重载, 到 Send, Sync 这种非常微妙的特性。
//一些 traits 是可以被自动派生的(你只需要写
//#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Hash, ...)] 就能得到一个神奇的实现, 它通常是对的。
//如果我们想比较某个类型的两个值 x 和 y 是否相等（不等），例如：x == y (x != y)，
//那么我们就必须为类型实现 PartialEq Trait。
//自己实现 PartialEq，实现时只需要实现判断是否相等的函数 fn eq(&self, other: &Self) -> bool
/*
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}
*/
//Clone trait 自动复制
//Debug 在打印时可以用{:?}直接打印内容
//Default  construct a new instance with default values
//Rust中的许多类型都有一个构造函数，但是，这是特定于类型的；Rust不能抽象为“任何有new()方法的东西”，
//为了实现这一点，构思了Default trait ，他可以于容器和其他通用类型一起使用
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HelloWorldState {
    /// account
    pub account_key: Pubkey,
    /// message
    pub message: String,
}

impl Sealed for HelloWorldState {}
impl IsInitialized for HelloWorldState {
    fn is_initialized(&self) -> bool {
        return true;
    }
}
/*
pack：
+-------------------------------------------+
|  key   |l|message                         |
+-------------------------------------------+
*/
//实现Pack Trait
impl Pack for HelloWorldState {
    const LEN: usize = 32 + 1 + 256; // max hello message's length is 256

    //首先通过array_ref得到一个array，然后通过array_refs指定三个成员 的内容，这里我们在序列化文件内容 时，
    //采用和Instruction一样的二进制序列化方法，对于Pubkey其固定为32个字节。对于Message，其长度
    //我们约定小于256，这样用一个字节表示长度，后面256个字节表示内容（256不一定全部用完，但是任然分配空间).
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 289];
        //多数组解码
        let (account_key_buf, message_len_buf, message_buf) = array_refs![src, 32, 1, 256];
        let account_key = Pubkey::new_from_array(*account_key_buf);
        let message_len = message_len_buf[0] as u8;
        let (msg_buf, _rest) = message_buf.split_at(message_len.into());
        let message = String::from(from_utf8(msg_buf).unwrap());
        Ok(HelloWorldState {
            account_key,
            message,
        })
    }
    //这里采用mut_array_refs预先给几个要存储的元素分配好地址，
    //然后使用copy_from_slice复制32字节 的key，用as u8转换长度，copy_from_slice copy字符串内容。
    fn pack_into_slice(&self, dst: &mut [u8]) {
        msg!("pack into slice");
        let dst = array_mut_ref![dst, 0, 289];
        let (account_key_buf, message_len_buf, message_buf) = mut_array_refs![dst, 32, 1, 256];
        msg!("pack into slice key");
        account_key_buf.copy_from_slice(self.account_key.as_ref());
        msg!("pack into slice len");
        message_len_buf[0] = self.message.len() as u8;
        msg!(&format!("pack into slice msg:{}", self.message));
        let (msg_buf, _rest) = message_buf.split_at_mut(self.message.len());
        msg_buf.copy_from_slice(self.message.as_bytes());
        msg!("pack into slice out");
    }

    // /// Unpack from slice without checking if initialized
    // fn unpack_unchecked(input: &[u8]) -> Result<Self, ProgramError> {
    //     if input.len() != Self::LEN {
    //         return Err(ProgramError::InvalidAccountData);
    //     }
    //     Self::unpack_from_slice(input)
    // }

    // /// Pack into slice
    // fn pack(src: Self, dst: &mut [u8]) -> Result<(), ProgramError> {
    //     if dst.len() != Self::LEN {
    //         return Err(ProgramError::InvalidAccountData);
    //     }
    //     src.pack_into_slice(dst);
    //     Ok(())
    // }
}
