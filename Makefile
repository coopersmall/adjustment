build:
	@echo "Removing target directory..."
	@make clean > /dev/null
	@echo "Building project..."
	@cargo build > /dev/null
	@echo "Project built successfully!"

bump:
	@echo "Bumping release version..."
	@chmod +x scripts/update-release.sh
	@bash scripts/bump.sh
	@echo "Release version bumped successfully!"


make deps:
	@echo "Installing dependencies..."
	@cargo build > /dev/null
	@echo "Dependencies installed successfully!"
	@make tooling > /dev/null
	@make install-hooks > /dev/null

clean:
	@echo "Removing target directory..."
	@rm -r target
	@echo "Target directory removed successfully!"

format:
	@echo "Formatting code..."
	@cargo fmt
	@echo "Code formatted successfully!"

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

lint:
	@echo "Linting code..."
	@cargo clippy -- -D warnings
	@echo "Code linted successfully!"

start:
	@echo "Starting project..."
	@cargo run

test:
	@echo "Running tests..."
	@cargo test
	@echo "Tests ran successfully!"

tooling:
	@echo "Installing tooling..."
	@chmod +x scripts/tooling.sh
	@bash scripts/tooling.sh
	@echo "Tooling installed successfully!"

.PHONY: clean format start test install-hooks all tooling deps lint

all: deps 
