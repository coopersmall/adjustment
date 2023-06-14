TARGET := target/

# Commands
BUILD_COMMAND = cargo build > /dev/null
TEST_COMMAND = cargo test > /dev/null
LINT_COMMAND = cargo clippy -- -D warnings > /dev/null
START_COMMAND = cargo run
FORMAT_COMMAND = cargo fmt > /dev/null
CLEAN_COMMAND = rm -r $(TARGET) > /dev/null

# Scripts 
PRE_COMMIT = scripts/pre-commit.sh
PRE_PUSH = scripts/pre-push.sh
RELEASE_MANAGER = scripts/bump.sh
TOOLING = scripts/tooling.sh

# Function definition
define clean_and_build
	if [ -e "$(TARGET)" ]; then \
		make clean > /dev/null; \
	fi; \
	$(BUILD_COMMAND);
endef

# Export function
export clean_and_build

# Build target
build:
	@echo "Building project..."
	@$(call clean_and_build)
	@echo "Project built successfully!"

bump:
	@echo "Bumping release version..."
	@chmod +x $(RELEASE_MANAGER)
	@bash $(RELEASE_MANAGER)
	@echo "Release version bumped successfully!"

deps:
	@echo "Installing dependencies..."
	@$(BUILD_COMMAND)
	@echo "Dependencies installed successfully!"
	@make tooling > /dev/null
	@make install-hooks > /dev/null

deps-dev:
	@echo "Installing dev dependencies..."
	@make tooling > /dev/null
	@$(BUILD_COMMAND) --dev
	@echo "Dev dependencies installed successfully!"
	@make install-hooks > /dev/null
	@make visual > /dev/null

clean:
	@echo "Removing target directory..."
	@$(CLEAN_COMMAND)
	@echo "Target directory removed successfully!"

format:
	@echo "Formatting code..."
	@$(FORMAT_COMMAND)
	@echo "Code formatted successfully!"

install-hooks:
	@echo "Installing pre-commit hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-commit
	@cat $(PRE_COMMIT) >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed successfully!"

	@echo "Installing pre-push hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-push
	@cat $(PRE_COMMIT) >> .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push
	@echo "Pre-push hook installed successfully!"

lint:
	@echo "Linting code..."
	@$(LINT_COMMAND)
	@echo "Code linted successfully!"

start:
	@echo "Starting project..."
	@$(START_COMMAND)

test:
	@echo "Running tests..."
	@$(TEST_COMMAND)
	@echo "Tests ran successfully!"

tooling:
	@echo "Installing tooling..."
	@chmod +x $(TOOLING)
	@bash $(TOOLING)
	@echo "Tooling installed successfully!"

version:
	@chmod +x $(RELEASE_MANAGER)
	@bash $(RELEASE_MANAGER)

visual:
	@echo "Installing fun visual tools :)"
	@chmod +x scripts/positivity.sh
	@chmod +x scripts/goodbye.sh
	@chmod +x scripts/visual.sh
	@bash scripts/visual.sh
	@echo "Visual tools installed successfully!"

.PHONY: clean format start test install-hooks all tooling deps lint bump visual

all: deps

