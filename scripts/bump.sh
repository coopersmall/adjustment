#!/bin/bash

# Function to display the menu
display_menu() {
    echo "Select an option:"
    echo "  [1] Workspace"
    echo "  [2] Crate"
    echo "  [3] Exit"
    echo
}

# Function to prompt for the crate name
prompt_for_crate() {
    echo "Enter the target crate:"
    echo "  [1] Utils"
    echo "  [2] Macros"
    echo "  [3] Common"
    echo "  [4] Exit"
    echo
}

# Function to validate the option
validate_option() {
    local option=$1

    case $option in
        1 | "workspace")
            selected_option="workspace"
            ;;
        2 | "crate")
            selected_option="crate"
            ;;
        3 | "exit")
            selected_option="exit"
            ;;
        *)
            selected_option=""
            ;;
    esac
}

# Function to validate the crate option
validate_crate_option() {
    local crate_option=$1

    case $crate_option in
        1 | "utils")
            crate_name="utils"
            ;;
        2 | "macros")
            crate_name="macros"
            ;;
        3 | "common")
            crate_name="common"
            ;;
        4 | "exit")
            echo
            echo "Exiting..."
            exit 0
            ;;
        *)
            crate_name=""
            ;;
    esac
}

# Function to check if a command is available
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Clear the screen
clear

# Print ASCII art
figlet "Release Version Manager"

echo

# Prompt for selection: workspace, crate, or exit
while true; do
    display_menu
    echo -n "Choice: "

    # Read user input and convert to lowercase
    read -r choice
    choice=$(echo "$choice" | tr '[:upper:]' '[:lower:]')

    # Remove trailing whitespace
    choice="${choice%"${choice##*[![:space:]]}"}"

    validate_option "$choice"

    case $selected_option in
        "workspace")
            # Update release version for the entire workspace

            # Run the script to update the release version
            ./scripts/update-release-version.sh
            break
            ;;
        "crate")
            # Prompt for the crate to update
            while true; do
                echo
                prompt_for_crate
                echo -n "Choice: "

                # Read user input and convert to lowercase
                read -r crate
                crate=$(echo "$crate" | tr '[:upper:]' '[:lower:]')

                # Remove trailing whitespace
                crate="${crate%"${crate##*[![:space:]]}"}"

                validate_crate_option "$crate"

                if [[ -n $crate_name ]]; then
                    break
                else
                    echo
                    echo "Invalid crate option. Please try again."
                fi
            done

            # Navigate to the crate directory
            cd "$crate_name" || exit 1

            # Run the script to update the release version
            ../../scripts/update-release-version.sh

            # Navigate back to the workspace root directory
            cd ..
            break
            ;;
        "exit")
            echo
            echo "Exiting..."
            exit 0
            ;;
        *)
            echo
            echo "Invalid option. Please try again."
            ;;
    esac
done
