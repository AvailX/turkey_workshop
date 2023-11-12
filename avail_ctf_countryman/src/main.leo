import avail_ctf_goose_3.leo;


program avail_ctf_countryman_3.aleo {

    transition my_address() -> public address {
        return avail_ctf_goose_3.leo/show_caller();
    }

    transition main(magic_phrase: scalar) {
        avail_ctf_goose_3.leo/lay_egg(magic_phrase);
    }
}
