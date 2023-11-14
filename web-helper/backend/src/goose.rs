type CurrentAleo = snarkvm::circuit::AleoV0;
type CurrentNetwork = snarkvm::prelude::Testnet3;

use anyhow::Result;
use dyn_fmt::AsStrFormatExt;
use snarkvm::console::program::ProgramID;
use snarkvm::prelude::*;

pub const PROG_GOOSE_LEO: &str = "avail_ctf_goose_{}.leo";
pub const PROG_COUNTRYMAN_ID: &str = "avail_ctf_countryman_{}.aleo";

//Arguments: index, address(avail_ctf_countryman.aleo), index, address(avail_ctf_countryman.aleo)
pub const AVAIL_CTF_GOOSE_LEO: &str = r#"
program avail_ctf_goose_{}.aleo {{

    record golden_egg {{
        owner: address,
        eggs: u64,
    }}

    function magic_key(magic_phrase: scalar, addr: address) -> field {{
        return BHP256::commit_to_field(addr, magic_phrase);
    }}

    transition show_caller() -> public address {{
        return self.caller;
    }}

    transition lay_egg(magic_phrase: scalar) -> (public address, golden_egg) {{

        //  {} = address("avail_ctf_countryman_{}.aleo")
        let door_key: field = magic_key(123321scalar, {});
        let caller_key: field = magic_key(magic_phrase, self.caller);

        assert(self.caller != self.signer);
        assert(caller_key == door_key);

        return (self.signer, golden_egg {{
            owner: self.signer,
            eggs: 1u64
        }});
    }}
}}
"#;

//Arguments: index, index, index, index
pub const AVAIL_CTF_COUNTYMAN_LEO: &str = r#"
import avail_ctf_goose_{}.leo;


program avail_ctf_countryman_{}.aleo {{

    transition my_address() -> public address {{
        return avail_ctf_goose_{}.leo/show_caller();
    }}

    transition main(magic_phrase: scalar) {{
        avail_ctf_goose_{}.leo/lay_egg(magic_phrase);
    }}
}}
"#;

fn prog_addr(prog_id: &str) -> Result<String> {
    let program_id = ProgramID::<CurrentNetwork>::from_str(prog_id)?;
    let addr = program_id.to_address()?;

    Ok(addr.to_string())
}

pub fn create_goose(index: &str) -> Result<String> {
    let cman_id = PROG_COUNTRYMAN_ID.format(&[index]);
    let cman_addr = prog_addr(&cman_id)?;

    let content_leo = AVAIL_CTF_GOOSE_LEO.format(&[index, &cman_addr, index, &cman_addr]);

    let ret = content_leo;

    Ok(ret)
}

pub fn get_secret_code(pk: &PrivateKey<CurrentNetwork>) -> Result<String> {
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
