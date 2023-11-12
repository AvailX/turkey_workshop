mod aleo_code;
mod prog_args;

use aleo_code::*;

use clap::Parser;
use prog_args::*;

use std::fs::*;
use std::path::Path;

use dyn_fmt::AsStrFormatExt;

use colored::Colorize;

// use snarkvm::prelude::query::Query;
use snarkvm::package::Package;
use snarkvm::prelude::block::Transaction;
use snarkvm::prelude::query::*;
use snarkvm::prelude::store::helpers::memory::*;
use snarkvm::prelude::store::*;
use snarkvm::prelude::*;

type CurrentAleo = snarkvm::circuit::AleoV0;
type CurrentNetwork = snarkvm::prelude::Testnet3;

fn generate_account(pk_str: &str) -> Result<(PrivateKey<CurrentNetwork>, Address<CurrentNetwork>)> {
    let pk = PrivateKey::from_str(pk_str)?;
    let ck = ComputeKey::try_from(&pk)?;
    let vk = ViewKey::try_from(&pk)?;
    let addr = Address::try_from(&ck)?;

    println!(
        r#"
    Private key: {pk}
    View key:    {vk}
    Address:     {addr}
    "#,
    );

    Ok((pk, addr))
}

fn prog_addr(prog_id: &str) -> Result<String> {
    let program_id = ProgramID::<CurrentNetwork>::from_str(prog_id)?;
    let addr = program_id.to_address()?;

    println!("Address for {prog_id}: {addr}");
    Ok(addr.to_string())
}

fn deploy(
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

fn handle_transaction(
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

fn create_file(base: &Path, filename: &str, content: &str) -> Result<()> {
    let file_path = base.join(filename);

    // Attempt to create the file and write the text to it
    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error writing '{}': {}", file_path.display(), e);
            } else {
                println!("'{}' created.", file_path.display());
            }
        }
        Err(e) => {
            eprintln!("Error creating '{}': {}", file_path.display(), e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn create_project_dir(base: &Path, index: &str) -> Result<String> {
    let project_name = PROG_GOOSE.format(&[index]);
    let project_path = base.join(project_name);

    // Attempt to create the directory
    match create_dir(&project_path) {
        Ok(_) => println!("Directory '{}' created.", project_path.display()),
        Err(e) => {
            eprintln!("Error creating '{}': {}", project_path.display(), e);
            return Err(e.into());
        }
    }

    Ok(project_path.to_string_lossy().to_string())
}

fn create_full_project(base: &Path, index: &str) -> Result<(String, String)> {
    let project = create_project_dir(base, index)?;
    let project = Path::new(&project);
    println!();

    let cman_id = PROG_COUNTRYMAN_ID.format(&[index]);
    let cman_addr = prog_addr(&cman_id)?;
    println!();

    let content_aleo = AVAIL_CTF_GOOSE_ALEO.format(&[index, &cman_addr]);
    create_file(project, MAIN_ALEO, &content_aleo)?;

    let content_json = AVAIL_CTF_GOOSE_JSON.format(&[index]);
    create_file(project, PROGRAM_JSON, &content_json)?;

    let goose_leo = PROG_GOOSE_LEO.format(&[index]);
    let content_leo = AVAIL_CTF_GOOSE_LEO.format(&[index, &cman_addr, index, &cman_addr]);
    create_file(project, &goose_leo, &content_leo)?;
    println!();

    let ret = (
        PROG_GOOSE_ID.format(&[index]),
        project.to_string_lossy().to_string(),
    );
    Ok(ret)
}

fn create_goose(base: &Path, index: &str) -> Result<String> {
    let cman_id = PROG_COUNTRYMAN_ID.format(&[index]);
    let cman_addr = prog_addr(&cman_id)?;
    println!();

    let goose_leo = PROG_GOOSE_LEO.format(&[index]);
    let content_leo = AVAIL_CTF_GOOSE_LEO.format(&[index, &cman_addr, index, &cman_addr]);
    create_file(base, &goose_leo, &content_leo)?;
    println!();

    let ret = PROG_GOOSE_ID.format(&[index]);
    Ok(ret)
}

fn create_countryman(base: &Path, index: &str) -> Result<String> {
    let countryman_leo = PROG_COUNTRYMAN_LEO.format(&[index]);
    let content_leo = AVAIL_CTF_COUNTYMAN_LEO.format(&[index, index, index, index]);
    create_file(base, &countryman_leo, &content_leo)?;
    println!();

    let ret = PROG_COUNTRYMAN_ID.format(&[index]);
    Ok(ret)
}

fn create_and_deploy(
    work_path: &Path,
    index: &str,
    pk: &PrivateKey<CurrentNetwork>,
    query_url: &str,
    broadcast_url: &str,
    priority_fee: u64,
) -> Result<()> {
    let (program_id, project_path) = create_full_project(work_path, index)?;
    let project_path = Path::new(&project_path);
    let tran_id = deploy(
        pk,
        query_url,
        broadcast_url,
        &program_id,
        project_path,
        priority_fee,
    )?;

    println!("Transaction id: {tran_id}");
    Ok(())
}

fn main() {
    let params = ProgArgs::try_parse();
    let params: ProgArgs = match params {
        Ok(res) => res,
        Err(err) => {
            let _ = err.print();
            cmd_usage();
            return;
        }
    };

    let do_rand = params.rand;
    let index_start = params.start.unwrap_or(0);
    let index_count = params.count.unwrap_or(1);
    if index_count < 1 {
        println!("Deployment count must be at least 1.");
        return;
    }

    if index_start.checked_add(index_count).is_none() {
        println!("Start/Count combination would overflow.");
        return;
    }

    let work_path = Path::new(&params.path);
    if !work_path.exists() || !work_path.is_dir() {
        print!("Directory not found at path {}!", &params.path);
        return;
    }

    let mut rng = rand::thread_rng();
    let pk;
    if params.pk.is_none() {
        pk = PrivateKey::new(&mut rng).unwrap();
    } else {
        (pk, _) = generate_account(&params.pk.unwrap()).unwrap();
    }

    let query_url = params.query.unwrap_or("http://localhost:3030".to_string());
    let broadcast_url = params
        .broadcast
        .unwrap_or("http://localhost:3030/testnet3/transaction/broadcast".to_string());
    let priority_fee = params.fee.unwrap_or(100000);

    for _index_pos in index_start..(index_start + index_count) {
        // Max u16 = 65535
        let program_secret = {
            let last_5_bytes = pk.to_bytes_le().unwrap().to_vec()[..5].to_vec();
            let last_5_digits = u32::from_le_bytes([
                last_5_bytes[2],
                last_5_bytes[1],
                last_5_bytes[3],
                last_5_bytes[0],
            ])
            .to_string()[..5]
                .to_string();

            format!("{}", last_5_digits)
        };

        if params.goose {
            create_goose(work_path, &program_secret).unwrap();
        } else if params.countryman {
            create_countryman(work_path, &program_secret).unwrap();
        } else {
            create_and_deploy(
                work_path,
                &program_secret,
                &pk,
                &query_url,
                &broadcast_url,
                priority_fee,
            )
            .unwrap();
        }
    }
}
