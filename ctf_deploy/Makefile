.SILENT:


deploy:
	YOURPK=APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs
	RUST_BACKTRACE=1

	cargo run --release  -- \
	--path ./deployments/ \
	--pk-fees $(YOURPK) \
	--query "http://37.27.5.0:3030" \
	--broadcast "http://37.27.5.0:3030/testnet3/transaction/broadcast"


deploy10:
	YOURPK=APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs
	RUST_BACKTRACE=1

	cargo run  --release  -- \
	--path ./deployments/ \
	--pk-fees $(YOURPK) \
	--query "http://37.27.5.0:3030" \
	--broadcast "http://37.27.5.0:3030/testnet3/transaction/broadcast" \
	--count 10


deploy50:
	YOURPK=APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs
	RUST_BACKTRACE=1

	cargo run  --release  -- \
	--path ./deployments/ \
	--pk-fees $(YOURPK) \
	--query "http://37.27.5.0:3030" \
	--broadcast "http://37.27.5.0:3030/testnet3/transaction/broadcast" \
	--count 50

checkgoose:
	@./checkgoose.sh


credits:
	@./credits.sh


checkcredits:
	@./checkcredits.sh


clean:
	@./clean.sh


cleanall:
	@./clean.sh
	-rm ./deployments/pk.txt
	-rm ./deployments/addr.txt
	-rm ./deployments/secrets.txt
