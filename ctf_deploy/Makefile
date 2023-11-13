.SILENT:

deploy: 
	RUST_BACKTRACE=1 \
	YOURPK=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH \
	cargo run  -- \
    --path ./deployments/ \
    --query "http://37.27.5.0:3030" \
    --broadcast "http://37.27.5.0/testnet3/transaction/broadcast" \
    --fee 100000

	# cargo run  	--  --path ./deployments/