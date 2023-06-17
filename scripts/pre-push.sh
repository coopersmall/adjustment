#!/bin/bash

crate_names=("utils" "common" "macros" "workspace")

# Set color variables
purple_light=$(tput setaf 147)
purple_dark=$(tput setaf 99)
light_green=$(tput setaf 76)
blue=$(tput setaf 39)
yellow=$(tput setaf 228)
bright_red=$(tput setaf 196)
reset=$(tput sgr0)

compare_versions() {
    if [[ $1 == $2 ]]; then
        return 0
    fi

    version_type=$3

    IFS='.' read -ra v1 <<< "$1"
    IFS='.' read -ra v2 <<< "$2"

    if [[ $version_type = "major" ]]; then
        position=0
    elif [[ $version_type = "minor" ]]; then
        position=1
    elif [[ $version_type = "patch" ]]; then
        position=2
    else
        clear
        echo "Invalid version type: ${version_type}"
        exit 1
    fi

    if ((10#${v1[$position]} == 10#${v2[$position]})); then
        return 0
    elif ((10#${v1[$position]} < 10#${v2[$position]})); then
        return 1
    elif ((10#${v1[$position]} > 10#${v2[$position]})); then
        return 2
    else
        echo "Something went wrong comparing the $version_type versions"
        exit 1
    fi
}

is_bumped_version() {
    current_version=$1
    master_version=$2
    version_type=$3

    IFS='.' read -ra current <<< "$current_version"
    IFS='.' read -ra master <<< "$master_version"

    if [[ $version_type == "major" ]]; then
        position=0
    elif [[ $version_type == "minor" ]]; then
        position=1
    elif [[ $version_type == "patch" ]]; then
        position=2
    else
        clear
        echo "Invalid version type: ${version_type}"
        exit 1
    fi

    if ((10#${current[$position]} == 10#${master[$position]} + 1)); then
        return 0
    else
        return 1
    fi
}

is_new_version() {
    current_version=$1
    master_version=$2
    version_type=$3

    IFS='.' read -ra current <<< "$current_version"
    IFS='.' read -ra master <<< "$master_version"

    if [[ $version_type == "major" ]]; then
        # Compare the major version with master to see if it was increased by one and the minor and patch versions are 0 (i.e. 1.0.0 -> 2.0.0)
        if ((10#${current[1]} == 0)) && ((10#${current[2]} == 0)); then
            return 0
        else
            return 1
        fi
    fi

    if [[ $version_type == "minor" ]]; then
        # Compare the minor version with master to see if it was increased by one and the patch version is 0 (i.e. 1.1.0 -> 1.2.0)
        if ((10#${current[2]} == 0)); then
            return 0
        else
            return 1
        fi
    fi

    if [[ $version_type == "patch" ]]; then
        # Compare the patch version with master to see if it was increased by one (i.e. 1.1.1 -> 1.1.2)
        # Always return true for patch versions
        return 0
    fi

    clear
    echo "Invalid version type: ${version_type}"
    exit 1
}

bump_version() {
    local version="$1"
    local version_type="$2"

    # Validate if the version is in the format of major.minor.patch
    if [[ $version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
        major="${BASH_REMATCH[1]}"
        minor="${BASH_REMATCH[2]}"
        patch="${BASH_REMATCH[3]}"
    else
        echo "${bright_red}The version must be in the format of major.minor.patch. Double check the Cargo.toml.${reset}"
        exit 1
    fi

    # Validate if major, minor, and patch are valid integers
    if ! [[ $major =~ ^[0-9]+$ && $minor =~ ^[0-9]+$ && $patch =~ ^[0-9]+$ ]]; then
        echo "${bright_red}The version must only contain numbers. Double check the Cargo.toml file.${reset}"
        exit 1
    fi

    # Increase the version based on the version type
    if [[ "${version_type}" == "major" ]]; then
        new_major=$((major + 1))
        new_version="${new_major}.0.0"
    elif [[ "${version_type}" == "minor" ]]; then
        new_minor=$((minor + 1))
        new_version="${major}.${new_minor}.0"
    elif [[ "${version_type}" == "patch" ]]; then
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
    else
        clear
        echo "Invalid version type: ${version_type}"
        exit 1
    fi

    echo "${new_version}"
}

# Function to validate if the only change is to the Cargo.toml file
check_toml_file() {
    local file="$1"

    # Get the commit history for the branch
    commit_history=$(git log --name-only --oneline "${file}")

    # Split the commit history by newlines
    IFS=$'\n' read -rd '' -a commits <<< "$commit_history"

    # Check if there is only one commit in the history
    if [[ ${#commits[@]} -eq 1 ]]; then
        # Get the file changes in the commit
        changed_files=$(git diff --name-only "${commits[0]}")

       # Check if the only changed file is the Cargo.toml for the given crate,
       if [[ $changed_files == "${file}" ]] && [[ $(awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }' "${file}") ]]; then
           return 0
       fi 
    fi

    return 1
}

is_first_push() {
    # Check if the master branch exists on origin (i.e. the remote)
    if ! git rev-parse --verify origin/master >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

echo "${yellow}Checking for version changes...${reset}"
echo

# Compare crate versions with master and update if necessary
for crate in "${crate_names[@]}"; do
    echo "Checking ${crate} version..."

    if [ "${crate}" = "workspace" ]; then
        src_path="src/"
        toml_path="Cargo.toml"
    else
        src_path="${crate}/src/"
        toml_path="${crate}/Cargo.toml"
    fi

    # Get the current version of the crate under [package]
    crate_version=$(awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }' "${toml_path}")
    echo "Current version: ${yellow}${crate_version}${reset}"

   # Get the master version of the crate under [package]
    master_version=$(git show "origin/master:${toml_path}" | awk -F'"' '/^\[package\]/ { package = 1 } package && /^version *=/ { gsub(/^[[:space:]]+|"[[:space:]]+$/, "", $2); print $2; exit }')
    echo "Master version: ${yellow}${master_version}${reset}"

    # Check if there are changes in the crate directory since the last commit
    echo "Checking for changes in ${crate}..."

    # If there are no changes in the crate directory compared to master, skip to the next crate
    if ! output=$(git diff --name-only --diff-filter=ACMRTUXB "$(git merge-base origin/master HEAD)" -- "${src_path}") && [ -n "$output" ]; then
        echo "${yellow}No changes detected in ${crate}.${reset}"
        continue
    fi

    echo "${yellow}Changes detected in ${crate}.${reset}"
    echo "Checking branch history for version changes..."

    was_major_version_changed=false
    was_minor_version_changed=false

    compare_versions "$crate_version" "$master_version" "major"
    if [[ $? -eq 2 ]]; then
        was_major_version_changed=true
    fi

    compare_versions "$crate_version" "$master_version" "minor"
    if [[ $? -eq 2 ]]; then
        was_minor_version_changed=true
    fi

    # Compare the crate major version with the master version and update if necessary
    echo "Checking if major version was changed..."

    if $was_major_version_changed; then
        echo "${yellow}Major version change detected in commit history! Validating version change...${reset}"

        # Validate that the major version was increased only by 1
        if ! is_bumped_version "$crate_version" "$master_version" "major"; then
            echo "${bright_red}Unable to validate major version change. Please ensure that the only change in the commit for your branch is to the version variable in the ${toml_path} file.${reset}"
            echo
            exit 1
        fi

        if ! is_new_version "$crate_version" "$master_version" "major"; then
            echo "${bright_red}Major version updates can only be made to a new version (for example 1.2.0 -> 2.0.0)${reset}"
            echo
            exit 1
        fi

        # Validate if the only change in the commit is to the version variable in the Cargo.toml file
        if ! check_toml_file "${toml_path}"; then
            echo "${bright_red}Major version updates can only be done in increments of 1. Double check the Cargo.toml file for ${crate}.${reset}"
            echo
            exit 1
        fi

        echo "Successfully validated major version change!"
    else
        echo "Major version change not detected in commit history."
    fi

    # Compare the crate minor version with the master version and update if necessary
    echo "Checking if minor version was changed..."

    if $was_minor_version_changed; then
        echo "${yellow}Minor version change detected in commit history! Validating version change...${reset}"

        # Validate that the minor version was increased only by 1
        if ! is_bumped_version "$crate_version" "$master_version" "minor"; then
            echo "${bright_red}Minor version updates can only be done in increments of 1. Double check the Cargo.toml file for ${crate}.${reset}"
            echo
            exit 1
        fi

        if ! is_new_version "$crate_version" "$master_version" "minor"; then
            echo "${bright_red}Minor version updates can only be made to a new version (for example 1.2.3 -> 1.3.0)${reset}"
            echo
            exit 1
        fi

        # Validate if the only change in the commit is to the version variable in the Cargo.toml file
        if ! check_toml_file "${toml_path}"; then
            echo "${bright_red}Unable to validate minor version change. Please ensure that the only change in the commit for your branch is to the version variable in the ${crate}/Cargo.toml file.${reset}"
            echo
            exit 1
        fi

        echo "${light_green}Successfully validated minor version change!${reset}"
    else
        echo "Minor version change not detected in commit history."
    fi

    # Compare the crate version with the master version and update if necessary
    # If the major or minor version was changed, then the patch version is not checked
    echo "Checking if patch bump is required..."
    if compare_versions "$crate_version" "$master_version" "patch" && [[ $? -le 1 ]] && ! $was_major_version_changed && ! $was_minor_version_changed; then
        # Extract major, minor, and patch versions using regex and validate them
        echo "${yellow}Patch bump required! Bumping version...${reset}"

        # Bump the version
        new_version=$(bump_version "$crate_version" "patch")

        # Update the version in Cargo.toml file
        sed -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$new_version\"/" "${toml_path}" > temp
        mv temp "${toml_path}"

        # Commit the changes
        git add "${toml_path}"
        git commit -m "bump ${crate} version to ${new_version}"

        echo "${light_green}Bumped ${crate} version to ${new_version}${reset}"
        echo
    else
        echo "${light_green}No version bump required.${reset}"
        echo
    fi
done

# Run cargo build
cargo build

# Run cargo test
cargo test

# Run cargo fmt
cargo fmt --all -- --check

# Run cargo clippy
# cargo clippy
#

