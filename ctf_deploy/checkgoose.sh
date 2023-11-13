#!/bin/bash

SECRET_FILE="./deployments/secrets.txt"
BASE_URL="http://37.27.5.0:3030/testnet3/program"
EXPECTED_TEXT="200000000u64"

# Check if the text file exists
if [ ! -f "$SECRET_FILE" ]; then
    echo "Error: Text file '$SECRET_FILE' not found."
    exit 1
fi

# Read each secret
while read -r secret; do
    url="$BASE_URL/avail_ctf_goose_$secret.aleo"
    echo "Checking URL: $url"

    # Use curl to make a GET request and check for "20000" in the response
    response=$(curl -s -o /dev/null -w "%{http_code}" "$url")

    if [ "$response" == "200" ]; then
        # If the response is 200
        echo "URL $url returned 200"
    else
        echo "URL $url did not return 200. Got $response"
        exit 1
    fi
done < "$SECRET_FILE"
