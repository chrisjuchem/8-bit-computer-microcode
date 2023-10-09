SHELL=bash

programs = addition \
	empty

generated/microcode.txt $(foreach program,$(programs),generated/programs/$(program).txt):
	pushd microcode; \
	cargo run; \
	popd
