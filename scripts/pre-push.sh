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
    if [[ $1 == $2 ]]; then
        return 0
    fi

    IFS='.' read -ra v1 <<< "$1"
    IFS='.' read -ra v2 <<< "$2"

    for ((i=0; i<${#v1[@]}; i++)); do
        if ((10#${v1[i]} < 10#${v2[i]})); then
            # v1 is less than v2
            return 1
            # v1 is greater than v2
        elif ((10#${v1[i]} > 10#${v2[i]})); then
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
        if ! git log --oneline "origin/master..HEAD" -- "${crate}/" | grep -q "Bump"; then
            crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
            major="${crate_version%%.*}"
            minor="${crate_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$new_version\"/" "${crate}/Cargo.toml" 
            echo "Bumped ${crate} version to ${new_version}"
            git add "${crate}/Cargo.toml"
        fi
    done

    # Increase the patch version for the workspace if it has changes
    if ! git log --oneline --name-only --diff-filter=ACMRTUXB "$(git merge-base origin/master HEAD)..HEAD" -- . | grep -q "Bump"; then
        workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$new_version\"/" "${crate}/Cargo.toml" 
        echo "Bumped workspace version to ${new_version}"
        git add Cargo.toml
    fi

    # Commit the changes if there are modifications
    if git diff --cached --quiet; then
        echo "No version changes detected. Nothing to commit."
    else
        git commit -m "Bump versions"
    fi
else
    echo "Checking for version changes..."
    echo
    # Compare crate versions with master and update if necessary
    for crate in "utils" "common" "macros"; do

        echo "Checking ${crate} version..."
        crate_version=$(awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }' "${crate}/Cargo.toml")
        echo "Current version: ${crate_version}"

        echo "Checking for changes in ${crate}..."
        # Check if there are changes in the crate directory since the last commit
        if output=$(git diff --name-only --diff-filter=ACMRTUXB "$(git merge-base origin/master HEAD)" -- "${crate}/"); then

            echo "Changes detected. Checking if version bump is required..."
            master_version=$(git show "origin/master:${crate}/Cargo.toml" | awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }')
            echo "Master version: ${master_version}"

            # Compare the crate version with the master version and update if necessary
            echo "Checking if version bump is required..."
            if version_compare "$crate_version" "$master_version" && [[ $? -le 1 ]]; then
                # Extract major, minor, and patch versions using regex and validate them
                echo "Version bump required. Bumping version..."
                if [[ $crate_version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
                    major="${BASH_REMATCH[1]}"
                    minor="${BASH_REMATCH[2]}"
                    patch="${BASH_REMATCH[3]}"

                    # Validate if major, minor, and patch are valid integers
                    if ! [[ $major =~ ^[0-9]+$ && $minor =~ ^[0-9]+$ && $patch =~ ^[0-9]+$ ]]; then
                        echo "The version must only contain numbers. Double check the Cargo.toml file for ${crate}."
                        exit 1
                    fi

                    new_patch=$((patch + 1))
                    new_version="${major}.${minor}.${new_patch}"

                    # Update the version in Cargo.toml
                    sed -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$new_version\"/" "${crate}/Cargo.toml" > temp
                    mv temp "${crate}/Cargo.toml"

                    # Remove the backup file created by sed
                    rm "${crate}/Cargo.toml-e"

                    git add "${crate}/Cargo.toml"
                    echo "Bumped ${crate} version to ${new_version}"
                    echo
                else
                    echo "Invalid version number detected. Double check the Cargo.toml file for ${crate}."
                    exit 1
                fi
            else
                echo "No version bump required."
                echo
            fi
        else
            echo "No changes detected."
            echo
        fi
    done

    echo "Checking workspace version..."
    workspace_version=$(awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }' Cargo.toml)
    echo "Current version: ${workspace_version}"

    # Check if there are changes in the workspace directory since the last commit
    if output=$(git diff --name-only --diff-filter=ACMRTUXB "$(git merge-base origin/master HEAD)" -- "src/"); then
        echo "Changes detected. Checking if version bump is required..."
        master_version=$(git show "origin/master:Cargo.toml" | awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }')
        echo "Master version: ${master_version}"

        # Compare the workspace version with the master version and update if necessary
        echo "Checking if version bump is required..."
        if version_compare "$workspace_version" "$master_workspace_version" && [[ $? -le 1 ]]; then
            echo "Version bump required. Bumping version..."

            # Extract major, minor, and patch versions using regex and validate them
            if [[ $workspace_version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
                major="${BASH_REMATCH[1]}"
                minor="${BASH_REMATCH[2]}"
                patch="${BASH_REMATCH[3]}"

                # Validate if major, minor, and patch are valid integers
                if ! [[ $major =~ ^[0-9]+$ && $minor =~ ^[0-9]+$ && $patch =~ ^[0-9]+$ ]]; then
                    echo "The version must only contain numbers. Double check the Cargo.toml file for the workspace."
                    exit 1
                fi 

                # Increase the patch version
                new_patch=$((patch + 1))
                new_version="${major}.${minor}.${new_patch}"

                # Update the workspace version in Cargo.toml
                sed -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$new_version\"/" Cargo.toml > temp
                mv temp Cargo.toml

                # Remove the backup file created by sed
                rm Cargo.toml-e

                git add Cargo.toml
                echo "Bumped workspace version to ${new_version}"
                echo
            else
                echo "Invalid version number detected. Double check the Cargo.toml file for the workspace."
                exit 1
            fi
        else
            echo "No version bump required."
            echo
        fi
    else
        echo "No changes detected."
        echo
    fi

    # Commit the changes if there are modifications
    if git diff --cached --quiet; then
        echo "No version changes detected. Nothing to commit."
    else
        echo "Committing version changes..."
        git commit -m "bump versions"
        version_commit_sha=$(git rev-parse HEAD)
        echo "Version changes commited. Name: bump version SHA: ${version_commit_sha}"
    fi
fi

# Run cargo build
cargo build

# Run cargo test
cargo test

# Run cargo fmt
cargo fmt --all -- --check

# Run cargo clippy
# cargo clippy

echo "Pushing changes to remote..."
