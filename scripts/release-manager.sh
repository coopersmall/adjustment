#!/bin/bash

# Set color variables
purple_light=$(tput setaf 147)
purple_dark=$(tput setaf 99)
light_green=$(tput setaf 76)
blue=$(tput setaf 39)
yellow=$(tput setaf 228)
bright_red=$(tput setaf 196)
reset=$(tput sgr0)

# Function to display the menu
display_menu() {
    echo "${purple_light}Select an option:${reset}"
    echo "${blue}"
    echo "  ${blue}[1]${reset} Workspace"
    echo "  ${blue}[2]${reset} Crate"
    echo "  ${blue}[3]${reset} Exit"
    echo "${reset}"
}

# Function to prompt for the crate name
prompt_for_crate() {
    echo "${purple_light}Enter the target crate:${reset}"
    echo "${blue}"
    echo "  ${blue}[1]${reset} Utils"
    echo "  ${blue}[2]${reset} Macros"
    echo "  ${blue}[3]${reset} Common"
    echo "  ${blue}[4]${reset} Exit"
    echo "${reset}"
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
            exit 0
            ;;
        *)
            crate_name=""
            ;;
    esac
}

exit_display() {
    local exit_code=$1
    local message=$2

    clear

    if [[ $exit_code -eq 0 ]]; then
        echo "${light_green}${message}${reset}"
    else
        echo "${bright_red}${message}${reset}"
    fi
}

# Function to bump the version in Cargo.toml
bump_version() {
    local crate=$1
    local major=$2
    local minor=$3
    local file

    case $crate in
        "workspace")
            file="./Cargo.toml"
            ;;
        "utils")
            file="./utils/Cargo.toml"
            ;;
        "macros")
            file="./macros/Cargo.toml"
            ;;
        "common")
            file="./common/Cargo.toml"
            ;;
        *)
            clear
            echo "$(bash ./scripts/text/exit.sh)"
            echo "${bright_red}Invalid crate name: $crate${reset}"
            exit 1
            ;;
    esac

    if [ -f "$file" ]; then
        # Extract the existing version
        local current_version=$(grep -oE 'version\s*=\s*"[^"]+"' "$file" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')

        # Split the version into major, minor, and patch parts
        local current_major=$(echo "$current_version" | cut -d. -f1)
        local current_minor=$(echo "$current_version" | cut -d. -f2)
        local current_patch=$(echo "$current_version" | cut -d. -f3)

        # Prompt for version update type (major or minor)
        echo "${purple_light}Select version update type:${reset}"
        echo "${blue}"
        echo "  ${blue}[1]${reset} Major"
        echo "  ${blue}[2]${reset} Minor"
        echo "${reset}"
        echo -n "Version type?: "
        read -r version_type

        # Increment the version based on the selected type
        case $version_type in
            1)
                # Major version update
                echo "${bright_red}Are you sure you want to update to a new major version? (y/n):${reset}"
                read -r confirmation
                if [[ $confirmation == "y" || $confirmation == "Y" ]]; then
                    major=$((current_major + 1))
                    minor=0
                else
                    clear
                    echo "$(bash ./scripts/text/exit.sh)"
                    echo "${bright_red}Error: $file not found.${reset}"
                    exit 1
                fi
                ;;
            2)
                # Minor versuion update
                major=$current_major
                minor=$((current_minor + 1))
                ;;
            *)

                clear
                echo "$(bash ./scripts/text/exit.sh)"
                echo "${bright_red}Invalid version update type. Version update canceled.${reset}"
                exit 1
                ;;
        esac
        
        # Update the version line in the file
        sed -i -e "/^\[package\]$/,/^\[/ s/^version *=.*/version = \"$major.$minor.0\"/" "$file" 

        clear

        echo "${light_green}$(bash ./scripts/text/goodbye.sh)${reset}"
        echo "Version bumped to $major.$minor.0 for $crate"
        exit 0
    else
        clear
        echo "$(bash ./scripts/text/exit.sh)"
        echo "${bright_red}Error: $file not found.${reset}"
        exit 1
    fi
}

# Clear the screen
clear

# Print ASCII art
echo "${purple_dark}$(figlet "Release Version Manager")${reset}"

# Prompt for selection: workspace, crate, or exit
while true; do
    display_menu
    echo -n "Which one?: "

    # Read user input and convert to lowercase
    read -r choice
    choice=$(echo "$choice" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')

    validate_option "$choice"

    case $selected_option in
        "workspace")
            echo "${light_green}$(bash ./scripts/text/positivity.sh)${reset}"
            echo
            echo "Updating release version for the entire workspace"
            # Bump version for the workspace
            bump_version "workspace" 0 1
            break
            ;;
        "crate")
            # Prompt for the crate to update
            while true; do
                echo "${light_green}$(bash ./scripts/text/positivity.sh)${reset}"
                prompt_for_crate
                echo -n "Which Crate?: "

                # Read user input and convert to lowercase
                read -r crate
                crate=$(echo "$crate" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')

                validate_crate_option "$crate"

                if [[ -n $crate_name ]]; then
                    echo "${light_green}$(bash ./scripts/text/positivity.sh)${reset}"
                    echo "Updating release version for $crate_name crate"
                    # Bump version for the selected crate
                    bump_version "$crate_name" 0 1
                    break
                else
                    echo "${bright_red}Invalid crate option. Please try again.${reset}"
                fi
            done
            break
            ;;
        "exit")
            clear
            echo "${light_green}$(bash ./scripts/text/goodbye.sh)${reset}"
            exit 0
            ;;
        *)
            echo "${bright_red}Invalid option. Please try again.${reset}"
            ;;
    esac
done
