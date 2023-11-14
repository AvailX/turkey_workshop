#!/bin/bash

ADDR_FILE="./deployments/addr.txt"
BASE_URL="http://37.27.5.0:3030/testnet3/program/credits.aleo/mapping/account"
EXPECTED_TEXT="00000000u64"

# Check if the text file exists
if [ ! -f "$ADDR_FILE" ]; then
    echo "Error: Text file '$ADDR_FILE' not found."
    exit 1
fi

# Read each address
while read -r address; do
    url="$BASE_URL/$address"
    echo "Checking URL: $url"

    # Use curl to make a GET request and check for "20000" in the response
    response=$(curl -s -o /dev/null -w "%{http_code}" "$url")

    if [ "$response" == "200" ]; then

        # If the response is 200, check if the HTML content includes the expected text
        html_content=$(curl -s "$url")
        if [[ $html_content == *"$EXPECTED_TEXT"* ]]; then
            echo "URL $url returned 200 and includes '$EXPECTED_TEXT'"
        else
            echo "URL $url returned 200, but does not include '$EXPECTED_TEXT'. Exiting..."
            exit 1
        fi

    else
        echo "URL $url did not return 200. Got $response"
        exit 1
    fi
done < "$ADDR_FILE"
