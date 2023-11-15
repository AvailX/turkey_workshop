// The 'avail_ctf_countryman_12345' program.
program avail_ctf_countryman_12345.aleo {
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
