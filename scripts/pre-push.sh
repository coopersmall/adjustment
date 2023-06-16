#!/bin/bash

# Function to check if a command is available
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to compare versions
version_compare() {
    local latest_version="$1"
    local current_version="$2"
    local major_latest="${latest_version%%.*}"
    local major_current="${current_version%%.*}"
    local minor_latest="${latest_version#*.}"
    local minor_current="${current_version#*.}"
    local patch_latest="${minor_latest#*.}"
    local patch_current="${minor_current#*.}"

    if [[ $major_current -lt $major_latest ]]; then
        return 1
    elif [[ $major_current -gt $major_latest ]]; then
        return 2
    elif [[ $minor_current -lt $minor_latest ]]; then
        return 1
    elif [[ $minor_current -gt $minor_latest ]]; then
        return 2
    elif [[ $patch_current -lt $patch_latest ]]; then
        return 1
    elif [[ $patch_current -gt $patch_latest ]]; then
        return 2
    else
        return 0
    fi
}

# Check if cargo is installed
if ! command_exists cargo; then
    echo "cargo not found. Installing cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Check if rustc is installed
if ! command_exists rustc; then
    echo "rustc not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Check if cargo-fmt is installed
if ! command_exists cargo-fmt; then
    echo "cargo-fmt not found. Installing cargo-fmt..."
    cargo install --force cargo-fmt
fi

# Check if cargo-clippy is installed
if ! command_exists cargo-clippy; then
    echo "cargo-clippy not found. Installing cargo-clippy..."
    cargo install --force cargo-clippy
fi

# Get the branch name
branch=$(git symbolic-ref --short HEAD)

# Determine if this is the first push to origin
first_push=false
if ! git rev-parse --abbrev-ref "@{u}" >/dev/null 2>&1; then
    first_push=true
fi

# Update crate versions if changes were made
if [ "$first_push" = true ]; then
    echo "First push to origin. Bumping versions for all crates."
    cargo update --workspace

    # Increase the patch version for each crate with changes
    for crate in "utils" "common" "macros"; do
        if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- "${crate}/"; then
            crate_version=$(grep -Po '(?<=version = ")[^"]+' "${crate}/Cargo.toml")
            major="${crate_version%%.*}"
            minor="${crate_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
            echo "Bumped ${crate} version to ${new_version}"
        fi
    done

    # Increase the patch version for the workspace if it has changes
    if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- .; then
        workspace_version=$(grep -Po '(?<=version = ")[^"]+' Cargo.toml)
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
        echo "Bumped workspace version to ${new_version}"
    fi
else
    echo "Checking for version updates in changed crates..."
    master_version=$(git show "origin/master:./Cargo.toml" | grep -Po '(?<=version = ")[^"]+' | head -n1)

    # Compare and update versions for changed crates
    for crate in "utils" "common" "macros"; do
        if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- "${crate}/"; then
            crate_version=$(grep -Po '(?<=version = ")[^"]+' "${crate}/Cargo.toml")
            version_compare "$master_version" "$crate_version"
            case $? in
                1)
                    major="${crate_version%%.*}"
                    minor="${crate_version#*.}"
                    patch="${minor#*.}"
                    new_patch=$((patch + 1))
                    new_version="${major}.${minor}.${new_patch}"
                    sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
                    echo "Bumped ${crate} version to ${new_version}"
                    ;;
                2)
                    echo "Cannot decrease the version of ${crate}. Please fix the version in ${crate}/Cargo.toml."
                    exit 1
                    ;;
                *)
                    echo "No version change required for ${crate}"
                    ;;
            esac
        fi
    done

    # Compare and update version for the workspace if it has changes
    if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- .; then
        workspace_version=$(grep -Po '(?<=version = ")[^"]+' Cargo.toml)
        version_compare "$master_version" "$workspace_version"
        case $? in
            1)
                major="${workspace_version%%.*}"
                minor="${workspace_version#*.}"
                patch="${minor#*.}"
                new_patch=$((patch + 1))
                new_version="${major}.${minor}.${new_patch}"
                sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
                echo "Bumped workspace version to ${new_version}"
                ;;
            2)
                echo "Cannot decrease the version of the workspace. Please fix the version in Cargo.toml."
                exit 1
                ;;
            *)
                echo "No version change required for the workspace"
                ;;
        esac
    fi
fi

# Run cargo build
echo "Running cargo build..."
cargo build

# Run cargo test
echo "Running cargo test..."
cargo test

# Run cargo fmt
echo "Running cargo fmt..."
cargo fmt --all -- --check

# Run cargo clippy
# echo "Running cargo clippy..."
# cargo clippy --all-targets -- -D warnings

# If any of the commands above fail, exit with a non-zero status
if [ $? -ne 0 ]; then
    echo "Pre-push checks failed. Please fix the errors before pushing."
    exit 1
fi

# Check if any changes were made to crate versions
if git diff --quiet --exit-code; then
    echo "No crate version changes detected."
else
    echo "Committing crate version changes."
    git add . && git commit -m "Bump crate versions"
    exec git push origin "$branch"
fi
