default:
	# This is my 2024 Advent of Code project

init:
	cargo new day$(day)
	sed 's/#DAY#/$(day)/g' main.rs.template | sed 's/#TITLE#/$(title)/g' > day$(day)/src/main.rs
	touch day$(day)/data.txt
	cd day$(day) && cargo add regex

build:
	cd ./day$(day) &&	cargo build

run:
	cd ./day$(day) &&	cargo run

test:
	cd ./day$(day) &&	cargo test