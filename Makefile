PACKAGE_MANAGER := cargo
TARGET := target/

# Commands
BUILD_COMMAND = $(PACKAGE_MANAGER) build > /dev/null
TEST_COMMAND = $(PACKAGE_MANAGER) test > /dev/null
LINT_COMMAND = $(PACKAGE_MANAGER) clippy -- -D warnings > /dev/null
START_COMMAND = $(PACKAGE_MANAGER) run
FORMAT_COMMAND = $(PACKAGE_MANAGER) fmt > /dev/null
CLEAN_COMMAND = rm -r $(TARGET) > /dev/null

# Scripts 
PRE_COMMIT = scripts/pre-commit.sh
PRE_PUSH = scripts/pre-push.sh
RELEASE_MANAGER = scripts/release-manager.sh
TOOLING = scripts/tooling.sh

# Functions definition
define clean_and_build
	if [ -e "$(TARGET)" ]; then \
		make clean > /dev/null; \
	fi; \
	$(BUILD_COMMAND);
endef

define make_scripts_executable
    find scripts -type f -name "*.sh" -exec chmod +x {} +
endef

# Export function
export clean_and_build
export make_scripts_executable

# Build target
build:
	@echo "Building project..."
	@$(call clean_and_build)
	@echo "Project built successfully!"

deps:
	@echo "Making scripts executable..."
	@$(call make_scripts_executable)
	@echo "Installing crate dependencies..."
	@$(BUILD_COMMAND)
	@echo "Installing tooling..."
	@make tooling > /dev/null
	@echo "Installing git hooks..."
	@make install-hooks > /dev/null
	@echo "Dependencies installed successfully!"

deps-dev:
	@echo "Making scripts executable..."
	@$(call make_scripts_executable)
	@echo "Installing tooling..."
	@make tooling > /dev/null
	@echo "Installing crate dependencies..."
	@$(BUILD_COMMAND) --dev
	@echo "Installing git hooks..."
	@make install-hooks > /dev/null
	@echo "Installing CLI dependencies..."
	@make deps-cli > /dev/null
	@echo "Dev dependencies installed successfully!"

deps-cli:
	@echo "Making scripts executable..."
	@$(call make_scripts_executable)
	@echo "Installing CLI dependencies..."
	@bash scripts/visual.sh
	@echo "CLI dependencies installed successfully!"

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

	@echo "Installing pre-push hook..."
	@echo "#!/bin/bash" > .git/hooks/pre-push
	@cat $(PRE_PUSH) >> .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push

	@echo "Hooks installed successfully!"

lint:
	@echo "Linting code..."
	@$(LINT_COMMAND)
	@echo "Code linted successfully!"

release:
	@bash $(RELEASE_MANAGER)

start:
	@echo "Starting project..."
	@$(START_COMMAND)

test:
	@echo "Running tests..."
	@$(TEST_COMMAND)
	@echo "Tests ran successfully!"

tooling:
	@echo "Installing tooling..."
	@bash $(TOOLING)
	@echo "Tooling installed successfully!"

.PHONY: build clean deps deps-dev deps-cli format install-hooks lint release start test tooling

all: deps

