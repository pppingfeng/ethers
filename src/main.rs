use ethers::prelude::*;
use ethers::utils::parse_ether;
use std::sync::Arc;
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到 Sepolia 测试网络的 HTTP 提供者
    let provider = Provider::<Http>::try_from("https://sepolia.infura.io/v3/cd35b901304d47d0b4976e1fa2cb8bad")?;
    let provider = Arc::new(provider);

    
    let private_key_hex = ""; // 这里填写私钥
    let wallet: LocalWallet = private_key_hex.parse()?;
    let wallet = wallet.with_chain_id(11155111u64); // Sepolia 网络的链 ID

    // 使用 Wallet 连接到 provider
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    // 获取账户地址
    let address = wallet.address();

    // 获取账户余额
    let balance = provider.get_balance(address, None).await?;
    println!("余额: {}", balance);

    // 获取下一个交易的 nonce
    let nonce = provider.get_transaction_count(address, None).await?;

    // 构造交易
    let tx = TransactionRequest::new()
        .to(H160::from_str("0x19F47e792660Da7D4aE59585eD0Dd88F0097C1fa").unwrap()) // 替换为目标地址
        .value(parse_ether("0.1")?) // 0.1 ETH
        .gas(21_000)
        .gas_price(20_000_000_000u64)
        .nonce(nonce)
        .chain_id(11155111u64); // Sepolia 网络的链 ID

    // 签名并发送交易
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    println!("交易哈希: {:?}", tx_hash);

    Ok(())
}

