mod aleo_code;
mod deployment;
mod files;
mod prog_args;

use aleo_code::*;

use clap::Parser;
use prog_args::*;

use dyn_fmt::AsStrFormatExt;
use snarkvm::prelude::*;
use std::path::Path;

use crate::{
    deployment::deploy,
    files::{create_file, create_full_project, write_file_line},
};

type CurrentAleo = snarkvm::circuit::AleoV0;
type CurrentNetwork = snarkvm::prelude::Testnet3;

fn load_account(pk_str: &str) -> Result<(PrivateKey<CurrentNetwork>, Address<CurrentNetwork>)> {
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

fn new_account<R: Rng + CryptoRng>(
    rng: &mut R,
) -> Result<(PrivateKey<CurrentNetwork>, Address<CurrentNetwork>)> {
    println!();
    println!("New Account created:");
    let pk = PrivateKey::<CurrentNetwork>::new(rng)?;
    load_account(&pk.to_string())
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

fn get_secret_code(pk: &PrivateKey<CurrentNetwork>) -> Result<String> {
    let last_5_bytes = pk.to_bytes_le()?.to_vec()[..5].to_vec();

    Ok(u32::from_le_bytes([
        last_5_bytes[2],
        last_5_bytes[1],
        last_5_bytes[3],
        last_5_bytes[0],
    ])
    .to_string()[..5]
        .to_string())
}

fn get_unique_player<R: Rng + CryptoRng>(work_path: &Path, rng: &mut R) -> Result<String> {
    let (pk_player, addr_player) = new_account(rng)?;
    let program_secret = get_secret_code(&pk_player)?;
    write_file_line(work_path, "pk.txt", &pk_player.to_string())?;
    write_file_line(work_path, "addr.txt", &addr_player.to_string())?;
    write_file_line(work_path, "secrets.txt", &program_secret)?;
    Ok(program_secret)
}

fn load_unique_player(pk: &str) -> Result<String> {
    let (pk_player, _) = load_account(pk)?;
    get_secret_code(&pk_player)
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

    if params.count.is_some() && params.pk_player.is_some() {
        println!("pk-player and count cannot be specified together");
        return;
    }

    if params.goose && params.countryman {
        println!("goose and countryman cannot be specified together");
        return;
    }

    if !params.goose && !params.countryman && params.pk_fees.is_none() {
        println!("specify pk-fees");
        return;
    }

    if (params.goose || params.countryman) && params.pk_player.is_none() {
        println!("goose and countryman require pk-player parameter");
        return;
    }

    let index_count = params.count.unwrap_or(1);
    if index_count < 1 {
        println!("Deployment count must be at least 1.");
        return;
    }

    let work_path = Path::new(&params.path);
    if !work_path.exists() || !work_path.is_dir() {
        print!("Directory not found at path {}!", &params.path);
        return;
    }

    let mut rng = rand::thread_rng();

    let (pk_fees, _) = if params.pk_fees.is_some() {
        load_account(&params.pk_fees.unwrap()).unwrap()
    } else {
        new_account(&mut rng).unwrap()
    };

    let query_url = params.query.unwrap_or("http://localhost:3030".to_string());
    let broadcast_url = params
        .broadcast
        .unwrap_or("http://localhost:3030/testnet3/transaction/broadcast".to_string());
    let priority_fee = params.fee.unwrap_or(100000);

    // Note parameter validation ensures that if
    // goose or countryman are specified
    // index_count = 1 and pk_player is set
    for _index_pos in 0..index_count {
        let program_secret = match &params.pk_player {
            Some(pk_player_str) => load_unique_player(pk_player_str).unwrap(),
            None => get_unique_player(work_path, &mut rng).unwrap(),
        };

        if params.goose {
            create_goose(work_path, &program_secret).unwrap();
        } else if params.countryman {
            create_countryman(work_path, &program_secret).unwrap();
        } else {
            create_and_deploy(
                work_path,
                &program_secret,
                &pk_fees,
                &query_url,
                &broadcast_url,
                priority_fee,
            )
            .unwrap();
        }
    }
}
