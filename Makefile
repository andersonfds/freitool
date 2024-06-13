PROJECT_NAME := $(shell basename $(CURDIR))
TARGET ?= default

build_macos:
	@echo "Building for macOS"
	cargo build --release --target $(TARGET)
	mkdir -p ./bin/macOS/$(TARGET)
	cp ./target/$(TARGET)/release/$(PROJECT_NAME) ./bin/macOS/$(TARGET)/$(PROJECT_NAME)
	tar -czvf ./bin/$(PROJECT_NAME)_macOS_$(TARGET).tar.gz -C ./bin/macOS/$(TARGET)/ $(PROJECT_NAME)
	rm -r ./bin/macOS
	@echo "Done!, checksum:"
	@shasum -a 256 ./bin/$(PROJECT_NAME)_macOS_$(TARGET).tar.gz

build_linux:
	@echo "Building for Linux"
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./bin/Linux/x86_64
	cp ./target/x86_64-unknown-linux-gnu/release/$(PROJECT_NAME) ./bin/Linux/x86_64/$(PROJECT_NAME)
	tar -czvf ./bin/$(PROJECT_NAME)_Linux_x86.tar.gz -C ./bin/Linux/x86_64/ $(PROJECT_NAME)
	rm -r ./bin/Linux

build_windows:
	@echo "Building for Windows"
	cargo build --release --target x86_64-pc-windows-gnu
	mkdir -p ./bin/Windows/x86_64
	cp ./target/x86_64-pc-windows-gnu/release/$(PROJECT_NAME).exe ./bin/Windows/x86_64/$(PROJECT_NAME).exe
	tar -czvf ./bin/$(PROJECT_NAME)_Windows_x86.tar.gz -C ./bin/Windows/x86_64/ $(PROJECT_NAME).exe
	rm -r ./bin/Windows