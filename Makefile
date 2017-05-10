test: build test-incidents

build:
	cargo build --release

test-incidents:
	time ./target/release/sli2dli --has-header --manifest tmp/manifest.json tmp/incidents.csv -vvv

