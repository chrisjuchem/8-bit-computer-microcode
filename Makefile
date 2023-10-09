SHELL=bash

programs = addition \
	empty

generated/microcode.txt $(foreach program,$(programs),generated/programs/$(program).txt) &: microcode/*
	pushd microcode; \
	cargo run; \
	popd


# TODO: add programs as dependencies
eeprom-programmer/eeprom-programmer.ino: \
		eeprom-programmer/eeprom-programmer.ino.j2 \
		generated/microcode.txt \
		generated/*.chipconf
	# TODO customize programs
	high_chip=$(high_chip) \
	rom_chip=$(rom_chip) \
	microcode=$$(cat generated/microcode.txt) \
	prog0=$$(cat generated/programs/addition.txt) \
	prog1=$$(cat generated/programs/empty.txt) \
	prog2=$$(cat generated/programs/empty.txt) \
	prog3=$$(cat generated/programs/empty.txt) \
	prog4=$$(cat generated/programs/empty.txt) \
	prog5=$$(cat generated/programs/empty.txt) \
	prog6=$$(cat generated/programs/empty.txt) \
	prog7=$$(cat generated/programs/empty.txt) \
	j2 $@.j2 -o $@


generated/%.chipconf:
	rm -f generated/*.chipconf
	touch $@

microcode-high microcode-low: rom_chip=false
microcode-high: high_chip=true
microcode-low: high_chip=false
rom: rom_chip=true
rom: high_chip=false

microcode-high microcode-low rom:  %: generated/%.chipconf eeprom-programmer/eeprom-programmer.ino
	./program

clean:
	rm eeprom-programmer/eeprom-programmer.ino
	rm generated/*.chipconf
	rm generated/microcode.txt
	rm generated/programs/*.txt
