build:
	cargo build

clean:
	rm -r target

format:
	cargo fmt

start:
	cargo run

test:
	cargo test

install-hooks:
	@echo "Installing pre-commit hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-commit
	@cat scripts/pre-commit.sh >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed successfully!"


	@echo "Installing pre-push hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-push
	@cat scripts/pre-push.sh >> .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push
	@echo "Pre-push hook installed successfully!"


.PHONY: clean format start test install-hooks all

all: install-hooks clean build

