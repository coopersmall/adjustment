#!/bin/bash

# Function to check if a command is available
command_exists() {
    command -v "$1" >/dev/null 2>&1
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

# Function to compare two versions
version_compare() {
    local v1=$1
    local v2=$2
    local IFS=.
    v1=($v1)
    v2=($v2)
    for ((i=0; i<3; i++)); do
        if ((10#${v1[i]} > 10#${v2[i]})); then
            return 1
        elif ((10#${v1[i]} < 10#${v2[i]})); then
            return 2
        fi
    done
    return 0
}

# Check if it's the first push to origin
first_push=false
if ! git rev-parse --verify origin/master >/dev/null 2>&1; then
    first_push=true
fi

# Update crate versions if changes were made
if [ "$first_push" = true ]; then
    echo "First push to origin. Bumping versions for all crates."
    cargo update --workspace

    # Increase the patch version for each crate with changes
    for crate in "utils" "common" "macros"; do
        if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- "${crate}/"; then
            crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
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
        workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
        echo "Bumped workspace version to ${new_version}"
    fi
else
    # Compare crate versions with master and update if necessary
    for crate in "utils" "common" "macros"; do
        crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
        master_version=$(git show "origin/master:${crate}/Cargo.toml" | awk -F'"' '/version =/{print $2}')
        if version_compare "$crate_version" "$master_version" && [[ $? -eq 2 ]]; then
            major="${crate_version%%.*}"
            minor="${crate_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
            echo "Bumped ${crate} version to ${new_version}"
        fi
    done

    # Compare workspace version with master and update if necessary
    workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
    master_workspace_version=$(git show "origin/master:Cargo.toml" | awk -F'"' '/version =/{print $2}')
    if version_compare "$workspace_version" "$master_workspace_version" && [[ $? -eq 2 ]]; then
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
        echo "Bumped workspace version to ${new_version}"
    fi
fi

# Add and commit the version changes
git add -A
git commit -m "Bump versions"

# If any of the commands above fail, exit with a non-zero status
if [ $? -ne 0 ]; then
    echo "Pre-push checks failed. Please fix the errors before pushing."
    exit 1
fi
