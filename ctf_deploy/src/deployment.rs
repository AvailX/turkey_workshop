use std::path::Path;

use colored::Colorize;
use snarkvm::{
    package::Package,
    prelude::{
        query::Query,
        store::{
            helpers::memory::{BlockMemory, ConsensusMemory},
            ConsensusStore,
        },
        transaction::Transaction,
        PrivateKey, ProgramOwner, Result,
    },
    synthesizer::{deployment_cost, VM},
};
use snarkvm_console_network::prelude::{bail, ensure};

use crate::{CurrentAleo, CurrentNetwork};

pub fn deploy(
    pk: &PrivateKey<CurrentNetwork>,
    query_url: &str,
    broadcast_url: &str,
    program_id: &str,
    aleo_path: &Path,
    priority_fee: u64,
) -> Result<String> {
    let query: Query<CurrentNetwork, BlockMemory<CurrentNetwork>> = Query::from(query_url);
    let package = Package::<CurrentNetwork>::open(aleo_path)?;

    let deployment = package.deploy::<CurrentAleo>(None)?;
    let deployment_id = deployment.to_deployment_id()?;

    // Generate the deployment transaction.
    let transaction = {
        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Initialize the VM.
        let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)?;
        let vm = VM::from(store)?;

        // Compute the minimum deployment cost.
        let (minimum_deployment_cost, (_, _)) = deployment_cost(&deployment)?;

        // Prepare the fees.
        let fee_authorization = vm.authorize_fee_public(
            pk,
            minimum_deployment_cost,
            priority_fee,
            deployment_id,
            rng,
        )?;
        let fee = vm.execute_fee_authorization(fee_authorization, Some(query), rng)?;

        // Construct the owner.
        let owner = ProgramOwner::new(pk, deployment_id, rng)?;

        // Create a new transaction.
        Transaction::from_deployment(owner, deployment, fee)?
    };

    handle_transaction(broadcast_url, transaction, program_id)
}

pub fn handle_transaction(
    broadcast_url: &str,
    transaction: Transaction<CurrentNetwork>,
    program_id: &str,
) -> Result<String> {
    let transaction_id = transaction.id();

    // Ensure the transaction is not a fee transaction.
    ensure!(
        !transaction.is_fee(),
        "The transaction is a fee transaction and cannot be broadcast"
    );

    // Send the deployment request to the local development node.
    match ureq::post(broadcast_url).send_json(&transaction) {
        Ok(id) => {
            // Remove the quotes from the response.
            let response_string = id.into_string()?.trim_matches('\"').to_string();
            ensure!(
                            response_string == transaction_id.to_string(),
                            "The response does not match the transaction id. ({response_string} != {transaction_id})"
                        );

            match transaction {
                Transaction::Deploy(..) => {
                    println!(
                        "✅ Successfully broadcast deployment {transaction_id} ('{}') to {}.",
                        program_id.bold(),
                        broadcast_url
                    )
                }
                Transaction::Execute(..) => {
                    println!(
                        "✅ Successfully broadcast execution {transaction_id} ('{}') to {}.",
                        program_id.bold(),
                        broadcast_url
                    )
                }
                Transaction::Fee(..) => {
                    println!(
                        "❌ Failed to broadcast fee '{}' to the {}.",
                        program_id.bold(),
                        broadcast_url
                    )
                }
            }
        }
        Err(error) => {
            let error_message = match error {
                ureq::Error::Status(code, response) => {
                    format!("(status code {code}: {:?})", response.into_string()?)
                }
                ureq::Error::Transport(err) => format!("({err})"),
            };

            match transaction {
                Transaction::Deploy(..) => {
                    bail!(
                        "❌ Failed to deploy '{}' to {}: {}",
                        program_id.bold(),
                        &broadcast_url,
                        error_message
                    )
                }
                Transaction::Execute(..) => {
                    bail!(
                        "❌ Failed to broadcast execution '{}' to {}: {}",
                        program_id.bold(),
                        &broadcast_url,
                        error_message
                    )
                }
                Transaction::Fee(..) => {
                    bail!(
                        "❌ Failed to broadcast fee '{}' to {}: {}",
                        program_id.bold(),
                        &broadcast_url,
                        error_message
                    )
                }
            }
        }
    };

    // Output the transaction id.
    Ok(transaction_id.to_string())
}
