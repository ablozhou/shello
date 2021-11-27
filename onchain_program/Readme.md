# Solana HellWorld Program
为了创建一个与单节点性能匹配的去中心化、无许可的网络，Solana团队成功开发8项关键技术：工作历史证明 PoH、基站拜占庭容错（Tower BFT）、涡轮机（区块传播协议）、海湾流（无内存交易转发协议）、海平面（并行智能合约）、流水线（验证交易）、云散（水平扩展账户数据库）以及存档器（分布式账本存储）。

Solana的合约编程，其实主要就是对Account的增删改查，或者说就是我们普通程序中的 对文件的增删改查。这其中需要使用Solana提供的SDK，按照其框架进行编程，一些主要 数据结构在SDK中都有提供，在编写逻辑的时候，可以直接使用。

## 网页钱包
（SolFlare、Sollet、Math Wallet等）网页钱包SolFlare（操作指南：https://docs.solana.com/zh/wallet-guide/solflare）、Sollet以及Math Wallet同样是第三方钱包，SolFlare、Sollet和Math Wallet都支持质押账户或抵押操作等行为。同时，三者皆可发送和接收SOL和SPL代币。
Sollet.io is a non-custodial browser-based web wallet created by the Project Serum (DEX) team.

## 命令行钱包
Solana CLI交易实现

```
zhh@nhsvr:~/git/shello/onchain_program$ solana-keygen new --outfile zhh.json
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none):

Wrote new keypair to zhh.json
=============================================================================
pubkey: A343EQXUakeNBaRULVcfim1MZRCpTaEQwmutVJM478kh
=============================================================================
# 这是我的第一个钱包地址
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
area glare limb crystal elegant sting draw enable loyal ostrich unfold indoor
=============================================================================
# 如果这是一个真的钱包，请不要把助记词分享出去！

zhh@nhsvr:~/git/shello/onchain_program$ solana balance -k zhh.json
0 SOL
# 空投 1 Sol到我的钱包地址
zhh@nhsvr:~/git/shello/onchain_program$ solana airdrop 1 -k zhh.json
Requesting airdrop of 1 SOL

Signature: k3yKDfdGAJr5Q6LxMWzReG8G5ngEt6Dsf6UJprpuwauxs86xhh2LKKB4B8DSD5NSokLM2afR8xyhpmKV1ba5FrA

1 SOL
zhh@nhsvr:~/git/shello/onchain_program$ solana balance -k zhh.json
1 SOL # 检查地址的余额

zhh@nhsvr:~/git/shello/onchain_program/$  solana balance 7FKkJEFdyEYR6YWnV7PR4okE9gJgkctejS4qyeBnTRPY
499999999.11456877 SOL


$ solana-keygen new -o # --no-outfile创建第二个钱包，纸钱包
zhh@nhsvr:~/git/shello/onchain_program$ solana-keygen new -o zhh1.json
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none):

Wrote new keypair to zhh1.json
=========================================================================
pubkey: ANScsPxqXsmqsBH5jkSBvtaYt35gjq1j4rFjcRFcmWd
=========================================================================
# 这是第二个钱包的地址
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
iron tackle cruel cement climb annual jeans hope present hammer code road
=========================================================================
# 如果这是一个真的钱包，请不要把助记词分享出去！
# 余额
zhh@nhsvr:~/git/shello/onchain_program$ solana balance -k zhh1.json
0 SOL

# 转账
USAGE:
    solana transfer <RECIPIENT_ADDRESS> <AMOUNT> --allow-unfunded-recipient --config <FILEPATH> --fee-payer <KEYPAIR> --from <FROM_ADDRESS> --keypair <KEYPAIR>

zhh@nhsvr:~/git/shello/onchain_program$ solana transfer ANScsPxqXsmqsBH5jkSBvtaYt35gjq1j4rFjcRFcmWd 1000 --from ~/.config/solana/id.json  --allow-unfunded-recipient  --fee-payer ~/.config/solana/id.json
msg BdkEAGXx3xyj1vkdjLjYxtuYo5K49P5zFqVKge8WyGfz

# 这是交易的签名
Signature: 5QRsKPytKY62hGVrNeQdgUQ2Vf6FVJ1uQSz54dXSt64h2v5PX44WG1k9kFAfQKkFx9Nd2vt24fKpDFPGEoUbgWys

# 接受账号余额
zhh@nhsvr:~/git/shello/onchain_program$ solana balance ANScsPxqXsmqsBH5jkSBvtaYt35gjq1j4rFjcRFcmWd
1000 SOL
zhh@nhsvr:~/git/shello/onchain_program$  solana balance 7FKkJEFdyEYR6YWnV7PR4okE9gJgkctejS4qyeBnTRPY
499998999.114563763 SOL
# 此前有499999999.11456877 SOL
# 发送账号减少的代币多于1000 SOL，因为减去了0.000005 SOL 的交易手续费

# 转账给程序账号？
zhh@nhsvr:~/git/shello/onchain_program$ solana address -k target/deploy/helloworld-keypair.json
BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU

solana transfer BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU 1000 --from ~/.config/solana/id.json  --allow-unfunded-recipient  --fee-payer ~/.config/solana/id.json
Error: RPC response error -32002: Transaction simulation failed: Transaction loads a writable account that cannot be written
zhh@nhsvr:~/git/shello/onchain_program$ solana balance BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU
0.00114144 SOL
```
## HelloWorld功能如下

1. DApp调用一个名为`printhello`的接口，该接口发送一个Transaction 到链上
2. 该Transaction包含两条Instruction：一条创建一个Account文件，一条将 helloworld的参数内容记录在这个Account文件中
3. 执行成功后，返回这个Account文件的地址。 DApp可以通过这个地址，查询里面存放的内容

## 合约流程
Solana的合约程序主要干三个事情
1. 解析由runtime传过来的instruction
2. 执行instruction对应的逻辑
3. 将执行结果中需要落地的部分，pack打包输出到指定的Account文

## 文件结构
根据上面的逻辑结构，我们依次创建如下几个文件：

* instruction.rs ： 解析由runtime传过来的instruction
* processor.rs : 针对instruction的合约逻辑
* state.rs : 将需要存储的内容进行打包存储
* error.rs: 出错处理，定义各种错误
* entrypoint.rs : 结合“entrypoint”特性，封装合约入口

### 文件
```
    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── entrypoint.rs
    │   ├── error.rs
    │   ├── instruction.rs
    │   ├── lib.rs
    │   ├── processsor.rs
    │   └── state.rs
    └── Xargo.tom
```

## Account 定义：
Account
Solana链上的资源包括了内存、文件、CPU(Compute Budge)等，不同于EOS的内存和CPU，Solana上只是对合约 运行的的栈大小（4KB),CPU执行时间（200,000 BPF），函数栈深度（64）做了最大数量的约定，所以不会出现 EOS上的抢资源的情况。Solana链上的信息，不同于EOS上的记录在内存，而是记录在文件中，这个文件在Solana上 表现为Account(PS:个人认为这个概念不是很好，容易和账户冲突，但是这个设计思想是OK的，类似Unix世界里面 的：一切皆是文件)，所以用户所需要支付的就是一个文件存储所需要的花费，是以SOL计价的。这里衍生出一个概念， 如果想要关闭文件的话，那么只要把这个Account的SOL都转走，那么这个Account对应的地址，在链上就没有钱 来买位置了，也就会被删除掉了。
```rust
/// Account information
#[derive(Clone)]
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
    /// Was the transaction signed by this account's public key?
    pub is_signer: bool,
    /// Is the account writable?
    pub is_writable: bool,
    /// The lamports in the account.  Modifiable by programs.
    pub lamports: Rc<RefCell<&'a mut u64>>,
    /// The data held in this account.  Modifiable by programs.
    pub data: Rc<RefCell<&'a mut [u8]>>,
    /// Program that owns this account
    pub owner: &'a Pubkey,
    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// The epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
}
```
AccountInfo就是一个Account在链上的表达形式，可以认为是一个文件的属性，想象一些state函数列出 的文件属性。其中，key表示文件名，也就是base58的地址。而文件大小可以认为是lamports，这里区别 与我们操作系统里面的文件，操作系统里面的文件的大小是可以为0的，且文件存在，而Solana链上的Account 如果其大小，也就是lamports为0的话，就认为这个文件被删除了（PS:这里将lamporsts类比作文件大小 是不完全准确的，因为文件大小是data字段内容的大小，但是从花费硬盘资源的角度，确实比较类似）。 这里的”is_writable”表示文件是否可执行，如果是可执行的，那么就是一个智能合约账号。 而data里面则是文件的内容，类似电脑上的ls 列出的文件属性，和cat列出来的文件内容，这里是二进制的
```
Rc<RefCell<&'a mut [u8]>>
```
buffer来表示。每个文件都要由一个程序来创建，这个程序称之为这个文件的拥有者，也就是这里的owner。
## runtime
Solana的Runtime前面说了，是执行BPF字节码的，为什么选择了这个runtime而不是WebAssembly或者Lua、Python 之类呢？其实主要还是因为性能的考量，Solana引以为傲的就是TPS，而BPF的执行效率更快。为了限制一个合约不至于 占光所有资源，runtime对合约的运行做了一些限制，当前的限制可以在SDK中查询：
```
pub struct BpfComputeBudget {

}
```
当执行超过限制时，该条合约执行就会失败。
## SPL Account 定义：
SPL即是Solana网络的代币标准，也被称为Solana程序库，是由Solana团队维护的链上程序的集合。

SPL代币专为DeFi应用程序设计，是一系列针对Sealevel（并行智能合约）并行运行时的链上程序。这些程序针对Solana的Sealevel实现、solana-runtime进行了测试，并部署到了其主网上。

### SPL代币与ERC20 相同点
其类似于以太坊区块链上的ERC20代币和币安智能链上的BEP20代币，以表现一种通用的和可预测的方式，帮助价值在链上的协议和去中心化应用程序中快速流动。
Solana的SPL同样如此，支持任何用户使用Solana区块链上的令牌程序创建自己的自定义SPL令牌。

用户可以通过链接钱包，在几分钟内创建自定义Solana代币（SPL代币）。创建后，即可通过将其配置在像Serum和Bonfida这样的DEX上来发行代币。随后用户可以将他们的钱包连接到这些DEX并开始交易。

### SPL代币与ERC20 不同点
不同点：在大多数的以太坊钱包上，用户都可以向钱包中的ETH接收地址发送或接收基于ERC-20标准铸造的代币。而SPL代币则不能直接流转在Solana原生代币SOL地址。

这也是两者之间最大的区别。

造成这种区别的原因在于Solana一直贯彻的“异步并行”观念，通过自动验证的各种“异步”处理来最大限度避免交易和验证过程的拥堵，保证最高的处理性能。而这一观念也被应用到了制定SPL代币标准和代币铸造流程中。

SPL是Solona链上唯一一个链上程序库，因此只需要通过⼀个账⼾来存储执⾏逻辑（Token_Program）即可。通过SPL发行的每一个新Token，都有⼀个对应的Account(Token Mint)来存储持有代币基本信息（供应量、铸币权限等），同时每个Token Holder还被分配⼀个对应的Token Account来记录持有⼈的持有数量的信息。Solana 区块链上的每个 SPL 代币都有自己的链上地址。

在这样的Token模型中数据的存储都是分开的，而每种Token，每个Token持有⼈都对应着⼀个独⽴的账⼾。这就提供了交易并行处理的可能，即使有⼀万笔交易需要处理，只要交易涉及的账号不同就不会造成拥堵。

还有一个值得注意的不同点是，在Solana链上，用户还可以运用SPL创建和发行非同质化代币（NFT）。而在以太坊上，ERC-20只能创建同质化代币，非同质化代币需要使用ERC-721标准。

### 和ERC20转换
2021年2月，Solana与以太坊间的双向跨链桥Wormhole（虫洞）正式启动，允许用户将ERC-20代币转换成Solana的SPL标准代币，以用于DeFi应用中。

## 预言机
“预言机”一词起源于希腊神话故事，指那些能直接与神沟通并预测未来的人。然而，与希腊神话中不同的是，区块链领域的预言机并不能预测未来，它们被用于数据的获取。
Chainlink是区块链预言机领域的先行者之一。通过Chainlink，为现实世界提供支持的传统平台可以无缝连接到新兴的区块链行业。Chainlink Price Feeds在市场遇到极端波动和网络不稳定的情况下，包括在中心化基础设施崩溃和数据操纵、黑客攻击期间，都证明了其高效、稳健的基本面。这使得Solana生态系统的开发者和 Serum、Raydium 、Oxygen 等协议能够支持高吞吐量DeFi应用，最终推动整个DeFi行业的进一步发展。
除了Chainlink之外，Solana还集成了其它一些优秀的预言机协议，包括Pyth Network、Band Protocol、Flux等
## Mint 定义：
## PublicKey 定义：
```rust
#[repr(transparent)]
#[derive(
    Serialize, Deserialize, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash, AbiExample,
)]
pub struct Pubkey([u8; 32]);]
```
Pubkey实际是就是32个字符表示的额base58的Account地址，在上面的Instruction中，我们看到的ProgramId 就是这样的类型，因为Program本身其实一个文件，也就是Account，只是是可执行的文件。
## 指令 instruction
Instruction在上面已经有介绍了，一个处理指令，包含了要处理他的程序的地址program_id,以及这个程序处理 时需要用到的Account表示的账号信息，还有这个指令对应的具体数据payload部分的data。
这里真实的用户协议数据是序列化后，存放在data里面的，所以整体流程是DApp客户端将自定义的指令数据序列化 到data里面，然后将账号信息和data发到链上，Solana节点为其找到要执行的程序，并将账号信息和数据data 传递给合约程序，合约程序里面将这个data数据在反序列化，得到客户端传过来的具体参数。
# 运行链上程序

## 启动本地程序
```
zhh@nhsvr:~$ solana-test-validator
Identity: EQhTggtNuPKLxBpXDdiwZzrM2UeBcKHeFMvbK2YDFg5a
Genesis Hash: 66kBQhHuk77ivZJkj4fAbvuDwQF9nGHuYLWcQS8CLXo7
Version: 1.9.0
Shred Version: 14330
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899

```

## 启动日志监控
```
zhh@nhsvr:~$ solana logs
```
## build
```
zhh@nhsvr:~/git/shello/onchain_program$ Cargo build
```
## deploy
https://docs.solana.com/zh/cli/deploy-a-program

- Initialize a program account
- Upload the program's shared object to the program account's data buffer
- Verify the uploaded program
- Finalize the program by marking the program account executable.

```
solana program deploy <PROGRAM_FILEPATH>
```
If the program id is not specified on the command line the tools will first look for a keypair file matching the <PROGRAM_FILEPATH>, or internally generate a new keypair.

```
solana program deploy --program-id <KEYPAIR_FILEPATH> <PROGRAM_FILEPATH>
```

```

zhh@nhsvr:~/git/shello/onchain_program$ cargo build-bpf
$ solana program deploy /home/zhh/git/shello/onchain_program/target/deploy/helloworld.so
msg HEF3FmbofWmhy5kLzw8TaMaAzNu5t9wX2Xan3wdQAZrv
Program Id: BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU

zhh@nhsvr:~/git/shello/onchain_program/target/deploy$ solana address
7FKkJEFdyEYR6YWnV7PR4okE9gJgkctejS4qyeBnTRPY
zhh@nhsvr:~/git/shello/onchain_program/target/deploy$  solana balance 7FKkJEFdyEYR6YWnV7PR4okE9gJgkctejS4qyeBnTRPY
499999999.11456877 SOL

# 部署
USAGE:
    solana deploy <PROGRAM_FILEPATH> --config <FILEPATH>

zhh@nhsvr:~/git/shello/onchain_program/$ solana deploy target/deploy/helloworld.so

zhh@nhsvr:~/git/shello/onchain_program/target/deploy$ solana balance BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU
0.00114144 SOL

zhh@nhsvr:~/git/shello/onchain_program$ solana program show BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU

Program Id: BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: Hr7EyJKZPiKwupC8itEC51r4d8jWHX9e15H7PWnyZPxe
Authority: 7FKkJEFdyEYR6YWnV7PR4okE9gJgkctejS4qyeBnTRPY
Last Deployed In Slot: 177657
Data Length: 126832 (0x1ef70) bytes
Balance: 0.8839548 SOL

```
- Program Id is the address that can be referenced in an instruction's program_id field when invoking - a program.
- Owner: The loader this program was deployed with.
- ProgramData Address is the account associated with the program account that holds the program's data - (shared object).
- Authority is the program's upgrade authority.
- Last Deployed In Slot is the slot in which the program was last deployed.
- Data Length is the size of the space reserved for deployments. The actual space used by the - currently deployed program may be less.

# 运行前端
需要修改App.js的区块链连接。
本地网络 http://47.242.205.48:8899
```
zhh@nhsvr:~/git/shello/onchain_program$ solana address -k target/deploy/helloworld-keypair.json
BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU
zhh@nhsvr:~/git/shello/onchain_program$ cat target/deploy/helloworld-keypair.json
[158,188...]
```
```
    let url =  'http://47.242.205.48:8899';//为本地网络
    this.programID = new PublicKey("BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU");
    this.playerPrivKey = [...]
```
```
zhh@nhsvr:~/git/shello/dapp$ yarn build

zhh@nhsvr:~/git/shello/dapp$ yarn start

```
访问 http://47.242.205.48:3000
# 问题：
## 费用不足,或者配错账号
If you do not have enough SOL in your wallet then you’ll not be able to add any tokens. You’ll get the following error.
```
failed to send transaction: Transaction simulation failed: Attempt to debit an account but found no record of a prior credit.
```
## 
```
Transaction failed to sanitize accounts offsets correctly
```
# Solana中的陷阱

1. 不要用ETH来类比Solana
2. 多参考Token合约
3. 多参考SDK代码

## Account长度固定
```rust
pub fn allocate(pubkey: &Pubkey, space: u64) -> Instruction {
    let account_metas = vec![AccountMeta::new(*pubkey, true)];
    Instruction::new(
        system_program::id(),
        &SystemInstruction::Allocate { space },
        account_metas,
    )
}
```
根据Solana开发的计划，未来会提供修改Account长度的功能。 参见[Discrod 讨论](https://discordapp.com/channels/428295358100013066/517163444747894795/786290020465115147)

对于一般程序的理解，文件对象都会有增删改查的功能，比如最开始可能不知道要用多少来存储，但是随着
用户的增加，要存储的内容越来越多，所以这个功能我们还是等待官方的开发吧。
## 如何得到AccountInfo

```rust
// rpc 调用
invoke(
    &system_instruction::transfer(
        src_info.key,
        dst_info.key,
        amount,
    ),
    &[
        src_info.clone(),
        dst_info.clone(),
        system_program_info.clone(),
    ],
)?;
```

## 合约和普通记录Account 不具有SOL交易能力：
```rust
// The balance of read-only and executable accounts may not change
if self.lamports != post.lamports {
    if !self.is_writable {
        return Err(InstructionError::ReadonlyLamportChange);
    }
    if self.is_executable {
        return Err(InstructionError::ExecutableLamportChange);
    }
}
// Only the owner of the account may change owner and
//   only if the account is writable and
//   only if the account is not executable and
//   only if the data is zero-initialized or empty
if self.owner != post.owner
    && (!self.is_writable // line coverage used to get branch coverage
        || self.is_executable
        || *program_id != self.owner
    || !Self::is_zeroed(&post.data))
{
    return Err(InstructionError::ModifiedProgramId);
}
```
## runtime资源：
```
Program GJqD99MTrSmQLN753x5ynkHdVGPrRGp35WqNnkXL3j1C consumed 200000 of 200000 compute units
Program GJqD99MTrSmQLN753x5ynkHdVGPrRGp35WqNnkXL3j1C BPF VM error: exceeded maximum number of instructions allowed (193200)
Program GJqD99MTrSmQLN753x5ynkHdVGPrRGp35WqNnkXL3j1C failed: custom program error: 0xb9f0002

---
Transaction simulation failed: Error processing Instruction 0: Program failed to complete 
Program N42Qjxtrb1KMwCshrpbSJxj3khrZwN51VVv5Zdv2AFL invoke [1]
Program log: [solong-lottery]:Instruction: SignIn
Program log: [solong-lottery]:process_signin lottery:award[0] fund[1000000000] price[1000000000] billboard[Epj4jWrXq4JsEAhvDKMAdR47GqZve8dyKp29KdGfR4X] pool[255]
Program log: Error: memory allocation failed, out of memory
Program N42Qjxtrb1KMwCshrpbSJxj3khrZwN51VVv5Zdv2AFL consumed 109954 of 200000 compute units
Program N42Qjxtrb1KMwCshrpbSJxj3khrZwN51VVv5Zdv2AFL BPF VM error: BPF program Panicked in  at 0:0
Program failed to complete: UserError(SyscallError(Panic("", 0, 0)))
Program N42Qjxtrb1KMwCshrpbSJxj3khrZwN51VVv5Zdv2AFL failed: Program failed to complete	
```
## stack和heap限制：
```
BpfComputeBudget {
    max_units: 100_000,
    log_units: 0,
    log_64_units: 0,
    create_program_address_units: 0,
    invoke_units: 0,
    max_invoke_depth: 1,
    sha256_base_cost: 85,
    sha256_byte_cost: 1,
    max_call_depth: 20,
    stack_frame_size: 4_096,
    log_pubkey_units: 0,
};
```
在上面的配置中，我们看到了栈大小被限制在4KB，而上面没有列出来的堆大小，其实也有个限制，是32KB。
所以我们再使用栈空间的时候，需要注意，这个大小限制。另外存储的Vec以及Box对象的时候，也要注意不
能超过32KB.

## ELF错误：
```
Error: dynamic program error: ELF error: ELF error: .bss section not supported

Failed to parse ELF file: read-write: base offet 207896
```
报在ELF上的错误，一般是我们用了Solana目前不支持的rust的特性，比如用了HashMap。或者不支持
SDK的接口如：find_program_address


No access to
- `rand` or any crates that depend on it
- `std::fs`
- `std::net`
- `std::os`
- `std::future`
- `std::process`
- `std::sync`
- `std::task`
- `std::thread`
- `std::time`
- Limited access to:
- `std::hash`
- `std::os`
- Bincode is extremely computationally expensive in both cycles and call depth and should be avoided
- String formatting should be avoided since it is also computationally expensive
- No support for `println!`, `print!`, the Solana SDK helpers in `src/log.rs` should be used instead
- The runtime enforces a limit on the number of instructions a program can execute during the processing of one instruction
