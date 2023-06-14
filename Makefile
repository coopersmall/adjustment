clean:
	rm -r target

format:
	cargo fmt

start:
	cargo run

test:
	cargo test

install-hooks:
	@echo "Installing pre-push hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-push
	@cat scripts/pre-push.sh >> .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push
	@echo "Pre-push hook installed successfully!"

.PHONY: clean format start test install-hooks all

all: install-hooks

