use solana_client::rpc_client::RpcClient;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcSignatureSubscribeConfig;
use solana_sdk::signature::Signature;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;

use std::{thread, time};

use bincode::deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_big_array::big_array;
use log::*;
use env_logger;

use std::str::FromStr;

big_array! {
    BigArray;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OpenOrders {
    pub buf1: [u8; 5],
    pub account_flags: u64, // Initialized, OpenOrders
    pub market: Pubkey,
    pub owner: Pubkey,

    pub native_coin_free: u64,
    pub native_coin_total: u64,

    pub native_pc_free: u64,
    pub native_pc_total: u64,

    pub free_slot_bits: u128,
    pub is_bid_bits: u128,
    #[serde(with="BigArray")]
    pub orders: [u128; 128],
    // Using Option<NonZeroU64> in a pod type requires nightly
    #[serde(with="BigArray")]
    pub client_order_ids: [u64; 128],
    pub referrer_rebates_accrued: u64,
    pub buf2: [u8; 7],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Market {
    buf1: [u8; 5],
    account_flags: u64,
    owner: Pubkey,
    valutSignerNouce: u64,
    baseMint: Pubkey,
    quoteMint: Pubkey,
    baseValut: Pubkey,
    baseDepositsTotal: u64,
    baseFeesAccrued: u64,
    quoteValut: Pubkey,
    quoteDepositTotal: u64,
    quoteFeesAccrued: u64,
    quoteDustThreshold: u64,
    requestQueue: Pubkey,
    eventQueue: Pubkey,
    bids: Pubkey,
    asks: Pubkey,
    baseLotSize: u64,
    quoteLotSize: u64,
    feeRateBps: u64,
    referrerRebatesAccrued: u64,
    buf2: [u8; 7],
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct AMM {
    status: u64,
    nonce: u64,
    orderNum: u64,
    depth: u64,
    coinDecimals: u64,
    pcDecimals: u64,
    state: u64,
    resetFlag: u64,
    minSize: u64,
    volMaxCutRatio: u64,
    amountWaveRatio: u64,
    coinLotSize: u64,
    pcLotSize: u64,
    minPriceMultiplier: u64,
    maxPriceMultiplier: u64,
    systemDecimalsValue: u64,
    minSeparateNumerator: u64,
    minSeperateDenominator: u64,
    tradeFeeNumerator: u64,
    tradeFeeDenominator: u64,
    pnlNumerator: u64,
    pnlDenominator: u64,
    swapFeeNumerator: u64,
    swapFeeDenominator: u64,
    needTakePnlCoin: u64,
    needTakePnlPc: u64,
    totalPnlPc: u64,
    totalPnlCoin: u64,
    poolTotalDepositPc: u128,
    poolTotalDepositCoin: u128,
    swapCoinInAmount: u128,
    swapCoinOutAmount: u128,
    swapCoin2PcFee: u64,
    swapPcInAmount: u128,
    swapPcOutAmount: u128,
    swapPc2CoinFee: u64,

    poolcCoinTokenAccount: Pubkey,
    poolPcTokenAccount: Pubkey,
    coinMintAddress: Pubkey,
    pcMintAddress: Pubkey,
    lpMintAddress: Pubkey,
    ammOpenOrders: Pubkey,
    serumMarket: Pubkey,
    serumProgram: Pubkey,
    ammTargetOrders: Pubkey,
    poolWithdrawQueue: Pubkey,
    poolTempLpTokenAccount: Pubkey,
    ammOwner: Pubkey,
    pnlOwner: Pubkey,
}

impl AMM {
    pub fn deserialize(input: &[u8]) -> Self {
        deserialize(input).unwrap()
    }
}

impl OpenOrders {
    pub fn deserialize(input: &[u8]) -> Self {
        deserialize(input).unwrap()
    }
}

impl Market {
    pub fn deserialize(input: &[u8]) -> Self {
        deserialize(input).unwrap()
    }
}


fn main() {
    let api_url = String::from("http://api.rpcpool.com/");
    let mut c = RpcClient::new(api_url.clone());

    let key = Pubkey::from_str("8tzS7SkUZyHPQY7gLqsMCXZ5EDCgjESUHcB17tiR1h3Z").unwrap();

    let order_key = Pubkey::from_str("GJwrRrNeeQKY2eGzuXGc3KBrBftYbidCYhmA6AZj2Zur").unwrap();

    let market_key = Pubkey::from_str("ByRys5tuUWDgL73G8JBAEfkdFf8JWBzPBDHsBVQ5vbQA").unwrap();

    /*let a = c.get_account_data(&key).unwrap();
    let order_data = c.get_account_data(&order_key).unwrap();
    let market_data = c.get_account_data(&market_key).unwrap();


    let b = AMM::deserialize(&a);
    let order_deserial = OpenOrders::deserialize(&order_data);
//

    let c = Market::deserialize(&market_data);

    println!("{:?}", b);
    println!("{:?}", order_deserial);
    println!("{:?}", c)*/
    let sig = Signature::from_str("2BhRygLSAhkNu7e5sCA78B28Z1SREuiJzqJT7qNXMXF1sL8kasPgvEQkMBHujidDW9hbDpc3Qs3EW9ucgMmYWy8W").unwrap();

    loop {
        let hash = c.get_recent_blockhash_with_commitment(CommitmentConfig::confirmed()).unwrap().value.0;
        println!("hash: {}", hash);

        let mut t = 0;

        loop {
            t+=1;
            if t==10 {
                break;
            }
            println!("{}, {:?}",t,  c.get_fee_calculator_for_blockhash_with_commitment(&hash, CommitmentConfig::confirmed()));
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
