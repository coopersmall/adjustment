#!/bin/bash

# Prompt for selection: workspace or crate
read -p "Choose an option: [1] Workspace [2] Crate: " option

if [ "$option" == "1" ]; then
    # Update release version for the entire workspace

    # Run the script to update the release version
    ./scripts/update-release-version.sh

elif [ "$option" == "2" ]; then
    # Prompt for the crate to update
    read -p "Enter the crate name (utils/macro/common): " crate

    # Validate the crate name
    if [ "$crate" != "utils" ] && [ "$crate" != "macro" ] && [ "$crate" != "common" ]; then
        echo "Invalid crate name. Please enter either 'utils', 'macro', or 'common'."
        exit 1
    fi

    # Navigate to the crate directory
    cd "$crate"

    # Run the script to update the release version
    ../../scripts/update-release-version.sh

    # Navigate back to the workspace root directory
    cd ..

else
    echo "Invalid option. Please choose either '1' or '2'."
    exit 1
fi

