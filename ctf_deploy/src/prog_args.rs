use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgArgs {
    /// Path to destination folder
    #[arg(long)]
    pub path: String,

    /// PK for deployment fees
    #[arg(long)]
    pub pk_fees: Option<String>,

    /// PK for player
    #[arg(long)]
    pub pk_player: Option<String>,

    /// Query URL (default: local node)
    #[arg(long)]
    pub query: Option<String>,

    /// Broadcast URL (default: local node)
    #[arg(long)]
    pub broadcast: Option<String>,

    /// Priority fee amount (default: 100000)
    #[arg(long)]
    pub fee: Option<u64>,

    /// Number of deployments
    #[arg(long)]
    pub count: Option<u16>,

    /// Produce only the leo goose contract
    #[arg(long)]
    pub goose: bool,

    /// Produce only the leo countryman contract
    #[arg(long)]
    pub countryman: bool,
}

pub fn cmd_usage() {
    println!(
        r#"

Example:

export YOURPK=APrivateKey1zkpA9GoBwmeGxHutfB87aYZRzXxs1G8HrnNhgtFn96wcTT9

# generate a player pk and deploy its goose contract
cargo run --release  -- \
    --path ./deployments/ \
    --pk-fees ${{YOURPK}} \
    --query "http://localhost:3030" \
    --broadcast "http://localhost:3030/testnet3/transaction/broadcast" \
    --fee 100000

# generate a player pk and deploy its goose contract
cargo run --release  --  --path ./deployments/  --pk-fees ${{YOURPK}}

# get the goose program
cargo run --release  --  --goose --pk-player  ${{YOURPK}}

# get the countryman program
cargo run --release  --  --countryman --pk-player  ${{YOURPK}}
"#
    );
}
