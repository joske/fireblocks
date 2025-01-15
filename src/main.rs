use bigdecimal::BigDecimal;
use fireblocks_sdk::{types::CreateAccount, Client, ClientBuilder, PagingVaultRequestBuilder};
use std::{fs::File, io::Read, str::FromStr, time::Duration};

pub async fn create_whitelisted_address(client: &Client) -> color_eyre::Result<()> {
    // create a whitelisted address
    let wallet = client.internal_wallet_create("blah").await?;
    println!("{:?}", wallet);
    Ok(())
}

pub async fn create_address(client: &Client) -> color_eyre::Result<()> {
    let params = CreateAccount {
        name: "omnibus".to_string(),
        hidden_on_ui: false,
        customer_ref_id: Some("omnibus".to_string()),
        auto_fuel: false,
    };
    let omnibus = client.create_vault(&params).await?;
    println!("{:?}", omnibus);
    let wallet = client.create_address(3, "MATIC_POLYGON").await?;
    println!("{:?}", wallet);
    Ok(())
}

pub async fn assets(client: &Client) -> color_eyre::Result<()> {
    let assets = client.supported_assets().await?;
    for asset in assets.0.iter() {
        println!("{:?}", asset);
    }
    let regex = regex::Regex::new(r"Matic.*").unwrap();
    let matic = assets
        .0
        .iter()
        .find(|asset| regex.is_match(asset.name.as_str()))
        .unwrap();
    println!("{:?}", matic);
    Ok(())
}

pub async fn create_wallet(client: &Client) -> color_eyre::Result<()> {
    // wallet == address
    let wallet = client.create_address(2, "BTC_TEST").await?;
    println!("{:?}", wallet);

    Ok(())
}

pub async fn transfer(client: &Client) -> color_eyre::Result<()> {
    // transfer from vault 0 to vault 2
    let amount = BigDecimal::from_str("0.00001").unwrap();
    let resp = client
        .create_transaction_vault(0, 2, "ETH_TEST5", amount, Some("test"))
        .await?;
    println!("{:?}", resp);
    Ok(())
}

pub async fn vaults(client: &Client) -> color_eyre::Result<()> {
    let min = BigDecimal::from_str("0.00000001").unwrap();
    let params = PagingVaultRequestBuilder::new()
        .min_threshold(&min)
        .limit(10)
        .build()?;
    let (vaults, _id) = client.vaults(params).await?;
    for vault in vaults.accounts.iter() {
        println!("{:?}", vault);
    }
    let vault = client.vault(2).await?;
    println!("{:?}", vault);

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
    let secret_file = std::env::var("FIREBLOCKS_SECRET")?;
    let mut file = File::open(secret_file).expect("file not found");
    let mut secret: String = String::new();
    file.read_to_string(&mut secret)
        .expect("something went wrong reading the file");
    let client = ClientBuilder::new(&api_key, &secret.into_bytes())
        .with_sandbox()
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(5))
        .build()?;

    vaults(&client).await?;

    Ok(())
}
