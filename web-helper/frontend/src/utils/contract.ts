import axios from "axios"

type apiResponse = {
	code: string,
	pk: string,
}

const test = `program avail_ctf_goose_3.aleo {

		record golden_egg {
			owner: address,
			eggs: u64,
		}
	
		function magic_key(magic_phrase: scalar, addr: address) -> field {
			return BHP256::commit_to_field(addr, magic_phrase);
		}
	
		transition show_caller() -> public address {
			return self.caller;
		}
	
		transition lay_egg(magic_phrase: scalar) -> (public address, golden_egg) {
	
			// door_key address is the address for contract "avail_ctf_countryman_xx.aleo"
			let door_key: field = magic_key(123321scalar, aleo1t69ffz0ltprnys87ysheyd8pmltflh2wftz4gysra7yt7qgs8syqklfhq8);
			let caller_key: field = magic_key(magic_phrase, self.caller);
	
			assert(self.caller != self.signer);
			assert(caller_key == door_key);
	
			return (self.signer, golden_egg {
				owner: self.signer,
				eggs: 1u64
			});
		}
	}
`

const getGooseContract = async (): Promise<apiResponse> => {

	let res = await axios.get<apiResponse>("http://localhost:8080");

	return res.data;
}



export default getGooseContract
