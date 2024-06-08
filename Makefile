PROJECT_NAME := $(shell basename $(CURDIR))

build_arm_macos:
	@echo "Building for ARM"
	cargo build --release --target aarch64-apple-darwin
	mkdir -p ./bin/macOS/arm64
	cp ./target/aarch64-apple-darwin/release/$(PROJECT_NAME) ./bin/macOS/arm64/$(PROJECT_NAME)
	tar -czvf ./bin/$(PROJECT_NAME)_macOS_arm.tar.gz -C ./bin/macOS/arm64/ $(PROJECT_NAME)
	rm -r ./bin/macOS

build_intel_macos:
	@echo "Building for x86"
	cargo build --release --target x86_64-apple-darwin
	mkdir -p ./bin/macOS/x86_64
	cp ./target/x86_64-apple-darwin/release/$(PROJECT_NAME) ./bin/macOS/x86_64/$(PROJECT_NAME)
	tar -czvf ./bin/$(PROJECT_NAME)_macOS_x86.tar.gz -C ./bin/macOS/x86_64/ $(PROJECT_NAME)
	rm -r ./bin/macOS

build_macos: build_arm_macos build_intel_macos

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