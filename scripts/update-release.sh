#!/bin/bash

# Function to validate the version format
validate_version() {
    local version=$1

    # Check if the version matches the expected format
    if ! [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version format. Please use 'major.minor.patch' format."
        exit 1
    fi
}

# Prompt user to select the version type (major or minor)
echo "Which version type do you want to update? (major/minor)"
read -r version_type

if [[ $version_type != "major" && $version_type != "minor" ]]; then
    echo "Invalid version type. Please choose 'major' or 'minor'."
    exit 1
fi

# Read the current version from Cargo.toml
current_version=$(awk -F '"' '/^version/ {print $2}' Cargo.toml)
echo "Current version: $current_version"

# Prompt user to enter the new version
echo "Enter the new $version_type version:"
read -r new_version

# Validate the new version format
validate_version "$new_version"

# Update the version based on the selected type
if [[ $version_type == "major" ]]; then
    # Increment the major version
    IFS='.' read -ra version_parts <<< "$current_version"
    new_version="$((version_parts[0] + 1)).0.0"
elif [[ $version_type == "minor" ]]; then
    # Increment the minor version
    IFS='.' read -ra version_parts <<< "$current_version"
    new_version="${version_parts[0]}.$((version_parts[1] + 1)).0"
fi

echo "Updating to version: $new_version"

# Update Cargo.toml with the new version
sed -i "s/^version = \".*\"$/version = \"$new_version\"/" Cargo.toml

echo "Version updated successfully!"
