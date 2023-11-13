mod aleo_code;
mod deployment;
mod files;
mod prog_args;

use aleo_code::*;

use clap::Parser;
use prog_args::*;

use std::fs::*;
use std::path::Path;

use dyn_fmt::AsStrFormatExt;

use snarkvm::prelude::*;

use crate::{
    deployment::deploy,
    files::{create_file, create_full_project},
};

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
        // write pk to file
        let pk_file = work_path.join("pk.txt");
        match File::create(&pk_file) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(pk.to_string().as_bytes()) {
                    eprintln!("Error writing '{}': {}", pk_file.display(), e);
                } else {
                    println!("'{}' created.", pk_file.display());
                }
            }
            Err(e) => {
                eprintln!("Error creating '{}': {}", pk_file.display(), e);
                return;
            }
        }
    } else {
        (pk, _) = generate_account(&params.pk.unwrap()).unwrap();
    }

    let query_url = params.query.unwrap_or("http://localhost:3030".to_string());
    let broadcast_url = params
        .broadcast
        .unwrap_or("http://localhst:3030/testnet3/transaction/broadcast".to_string());
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
