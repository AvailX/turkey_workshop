pub const MAIN_ALEO: &str = "main.aleo";
pub const PROGRAM_JSON: &str = "program.json";
pub const PROG_GOOSE: &str = "avail_ctf_goose_{}";
pub const PROG_GOOSE_LEO: &str = "avail_ctf_goose_{}.leo";
pub const PROG_GOOSE_ID: &str = "avail_ctf_goose_{}.aleo";
pub const PROG_COUNTRYMAN_LEO: &str = "avail_ctf_countryman_{}.leo";
pub const PROG_COUNTRYMAN_ID: &str = "avail_ctf_countryman_{}.aleo";

//Arguments: index
pub const AVAIL_CTF_GOOSE_JSON: &str = r#"
{{
    "program": "avail_ctf_goose_{}.aleo",
    "version": "0.0.0",
    "description": "",
    "license": "MIT"
}}
"#;

//Arguments: index, address(avail_ctf_countryman.aleo)
pub const AVAIL_CTF_GOOSE_ALEO: &str = r#"
program avail_ctf_goose_{}.aleo;

record golden_egg:
    owner as address.private;
    eggs as u64.private;


closure magic_key:
    input r0 as scalar;
    input r1 as address;
    commit.bhp256 r1 r0 into r2 as field;
    output r2 as field;


function show_caller:
    output self.caller as address.public;


function lay_egg:
    input r0 as scalar.private;
    call magic_key 123321scalar {} into r1;
    call magic_key r0 self.caller into r2;
    is.neq self.caller self.signer into r3;
    assert.eq r3 true;
    is.eq r2 r1 into r4;
    assert.eq r4 true;
    cast self.signer 1u64 into r5 as golden_egg.record;
    output self.signer as address.public;
    output r5 as golden_egg.record;

"#;

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
