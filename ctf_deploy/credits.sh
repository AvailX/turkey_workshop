#!/bin/bash

ADDR_FILE="./deployments/addr.txt"
YOURPK=APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs
NODE_IP="37.27.5.0"
# NODE_IP="localhost"

# Check if the text file exists
if [ ! -f "$ADDR_FILE" ]; then
    echo "Error: Text file '$ADDR_FILE' not found."
    exit 1
fi

# Read each address run snarkos
while IFS= read -r address; do
    echo "Running snarkos with parameter: $address"

    snarkos developer execute credits.aleo  transfer_public \
        "$address" "200000000u64" \
        --private-key "${YOURPK}" \
        --query "http://${NODE_IP}:3030"  \
        --broadcast "http://${NODE_IP}:3030/testnet3/transaction/broadcast"  \
        --priority-fee 100000

done < "$ADDR_FILE"
