use std::str::FromStr;
use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use pyth_sdk_solana::{load_price_feed_from_account};

fn main() {
    // The ETH/USD price key can be found at https://pyth.network/price-feeds/crypto-eth-usd
    let key = Pubkey::from_str("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw").unwrap();
    let client = RpcClient::new("http://api.devnet.solana.com");

    let mut account = client.get_account(&key).unwrap();
    let feed = load_price_feed_from_account(&key, &mut account).unwrap();
    let result = feed.get_current_price().unwrap();

    println!("ETH/USD price");
    println!("status: \t{:?}", feed.status);
    println!("#publishers: \t{}", feed.num_publishers);
    println!("========================");

    println!("exponent: \t{}", result.expo);
    println!("conf: \t\t{}", result.conf);
    println!("price: \t\t{}", result.price);
    let actual_conf = (result.conf as f64) * (10 as f64).powf(result.expo as f64);
    let actual_price = (result.price as f64) * (10 as f64).powf(result.expo as f64);
    println!("combined: \t{} * 10^({}) = ${} +- ${}", result.price, result.expo, actual_price, actual_conf);
}
