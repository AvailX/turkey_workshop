use std::str::FromStr;

use aleo_tools::{api::AleoAPIClient, program_manager::ProgramManager};
use snarkvm::{
    console::program::{anyhow, Identifier, Value},
    prelude::{PrivateKey, Testnet3},
};

#[tauri::command(rename_all = "snake_case")]
pub fn execute_program(private_key: &str, program_id: &str, function_name: &str, input: &str) {
    let api_client = match AleoAPIClient::<Testnet3>::new("http://37.27.5.0:3030", "testnet3") {
        Ok(api_client) => api_client,
        Err(e) => panic!("Error creating API client: {:?}", e),
    };

    let private_key = match PrivateKey::<Testnet3>::from_str(private_key) {
        Ok(private_key) => private_key,
        Err(e) => panic!("Error creating private key: {:?}", e),
    };

    let input = match Value::<Testnet3>::from_str(input) {
        Ok(input) => input,
        Err(e) => panic!("Error creating input: {:?}", e),
    };

    let function = match Identifier::<Testnet3>::from_str(function_name) {
        Ok(function) => function,
        Err(e) => panic!("Error creating function: {:?}", e),
    };

    // Get the program from chain, error if it doesn't exist
    let program = api_client
     .get_program(program_id)
     .map_err(|_| anyhow!("Program {program_id:?} does not exist on the Aleo Network. Try deploying the program first before executing.")).unwrap();

    let rng = &mut rand::thread_rng();

    let vm = ProgramManager::<Testnet3>::initialize_vm(&api_client, &program, true).unwrap();

    let transaction = vm
        .execute(
            &private_key,
            (program_id, function),
            vec![input].iter(),
            None,
            1u64,
            None,
            rng,
        )
        .unwrap();

    println!("Transaction: {:?}", transaction.id().to_string());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        execute_program(
            "APrivateKey1zkpGowLYHT1mLL8atgSTdvzL1EwfB65CqD93zMvNT5aVDVS",
            "avail_ctf_countryman_37381.aleo",
            "main",
            "123321scalar",
        )
    }
}
