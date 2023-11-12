.SILENT:

deploy: 
	RUST_BACKTRACE=full \
	YOURPK=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH \
	cargo run  -- \
    --path ./deployments/ \
    --pk ${YOURPK} \
    --query "http://node.avail.global" \
    --broadcast "http://node.avail.global/testnet3/transaction/broadcast" \
    --fee 100000

	# cargo run  	--  --path ./deployments/