all:
	echo "Usage: make [offline | online]"

offline:
	cargo run offline

online:
#cd programs/online; cargo test-bpf --test loan_to_value
	cd programs/online; cargo build-bpf
invoke:
	cd programs/online/scripts; npm run build; echo invoking transaction...; node invoke.js
deploy:
	solana program deploy --program-id target/deploy/pyth_best_practice_online-keypair.json target/deploy/pyth_best_practice_online.so
