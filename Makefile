all:
	echo "Usage: make [offline | online]"

offline:
	cargo run offline

online:
	cd programs/online; cargo test-bpf --test loan_to_value
