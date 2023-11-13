# CTF - The Goose that Laid the Golden Eggs

[Book by Aesop](https://read.gov/aesop/091.html)


## CTF Description

1. `avail_ctf_goose.aleo` is a program that generates `golden_egg` tokens.


1. It contains a transition with the signature: <br />
    `transition lay_egg(magic_phrase: scalar) -> golden_egg`


1. `lay_egg` uses the `magic_phrase` to compute a commitment.


1. If the `magic_phrase` commitment matches a special `door_key` comitment, a `golden_egg` is transferred to the player.


1. The `magic_phrase` commitment is computed as follows: <BR />
    `commitment(self.caller + magic_phrase)`


1. The player will need to discover how the `door_key` and `magic_phrase` comitments are being computed, such that to make them match.


1. Ultimately the solution is for the player to understand that `self.caller` is the address of a second program, the program he needs to code. This program must be named `avail_ctf_countryman.aleo`

    _The program name must be shown as a comment within the code since contracts don't have the ability to compute address from program ids. Users will be provided with the code for avail_ctf_goose.aleo_

1. CTF is completed when the player recieves a `golden_egg`.



## Solution

Write a program called `avail_ctf_countryman.aleo` that sends the golden_egg to the player's address.

1. Make sure you are running leo version v1.10.0:
    ```BASH
    leo -V
    ```

1. Create leo project and open in VS code:
    ```BASH
    leo new avail_ctf_countryman
    cd avail_ctf_countryman
    code .
    ```

1. Create `imports` directory
    ```BASH
    mkdir imports
    ```

1. Copy the code for `avail_ctf_goose.leo` to the import directory.
    ```BASH
    # Adjust the path as necessary.
    cp ../avail_ctf_goose/src/main.leo  ./imports/avail_ctf_goose.leo
    ```

1. Code the avail_ctf_countryman/src/main.leo file as [in this solution](./avail_ctf_countryman/src/main.leo).

1. Build solution
    ```BASH
    leo build
    ```

1. Deloy Countryman program:
    ```BASH
    export YOURPK=
    export YOURVK=
    export YOURADDR=

    snarkos  developer  deploy  avail_ctf_countryman_58102.aleo  \
        --private-key "${YOURPK}" \
        --path "./build/" \
        --query "http://37.27.5.0:3030" \
        --broadcast "http://37.27.5.0:3030/testnet3/transaction/broadcast" \
        --priority-fee 100000
    ```

1. Confirm deployment: <BR />
    http://37.27.5.0:3030/testnet3/transaction/transaction_id  <BR />
    http://37.27.5.0:3030/testnet3/program/avail_ctf_countryman.aleo


1. Execute `avail_ctf_countryman.aleo` | `main` to generate golden egg:
    ```BASH
    snarkos  developer  execute  avail_ctf_countryman.aleo  main  \
        123321scalar  \
        --private-key "${YOURPK}"  \
        --query "http://37.27.5.0:3030" \
        --broadcast "http://37.27.5.0:3030/testnet3/transaction/broadcast" \
        --priority-fee 100000
    ```

1. Confirm solution: <BR />
    http://37.27.5.0:3030/testnet3/transaction/transaction_id


## CTF Setup
1. Generate pk.

1. Deposit 100 credits to the address corresponding to the pk.

1. Provide player:
    * pk
    * unique CTF five digit code.
    * avail_ctf_goose_xxxxx.leo


## Notes

1. To compute program address from its ID:

    ```BASH
    git checkout AVL-80-CTF-GoldenEgg
    cd ./proof_of_concepts/credit_custody/prog_addr
    cargo run -- "avail_ctf_countryman_xxxxx.aleo"
    ```

1. To decrypt records you can use:

    ```BASH
    git checkout AVL-80-CTF-GoldenEgg
    cd ./proof_of_concepts/kaxxa123_t1_msigv21/aleo_decrypt

    cargo run record  \
        --vk ${YOURVK} \
        --cipher record1...
    ```
