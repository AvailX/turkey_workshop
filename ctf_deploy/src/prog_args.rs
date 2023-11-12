use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgArgs {
    /// Path to destination folder
    #[arg(long)]
    pub path: String,

    /// PK for deployment fees (default: generate new)
    #[arg(long)]
    pub pk: Option<String>,

    /// Query URL (default: local node)
    #[arg(long)]
    pub query: Option<String>,

    /// Broadcast URL (default: local node)
    #[arg(long)]
    pub broadcast: Option<String>,

    /// Priority fee amount (default: 100000)
    #[arg(long)]
    pub fee: Option<u64>,

    /// Starting/Minimum index for deployment set
    #[arg(long)]
    pub start: Option<u16>,

    /// Number of deployments
    #[arg(long)]
    pub count: Option<u16>,

    /// Generate deployment indexes randomly
    #[arg(long)]
    pub rand: bool,

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

cargo run --release  -- \
    --path ./deployments/ \
    --pk ${{YOURPK}} \
    --query "http://localhost:3030" \
    --broadcast "http://localhost:3030/testnet3/transaction/broadcast" \
    --fee 100000 --rand

cargo run --release  --  --path ./deployments/
"#
    );
}
