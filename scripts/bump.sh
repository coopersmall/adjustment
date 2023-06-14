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
            echo
            bash ./scripts/goodbye.sh  
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
echo "${purple_dark}$(figlet "Release Version Manager")${reset}"

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
            bash ./scripts/positivity.sh | sed "s/.*/${light_green}&${reset}/"
            echo
            # Update release version for the entire workspace

            # Run the script to update the release version
            ./scripts/update-release-version.sh
            break
            ;;
        "crate")
            # Prompt for the crate to update
            bash ./scripts/positivity.sh | sed "s/.*/${light_green}&${reset}/"
            while true; do
                prompt_for_crate
                echo -n "Choice: "

                # Read user input and convert to lowercase
                read -r crate
                crate=$(echo "$crate" | tr '[:upper:]' '[:lower:]')

                # Remove trailing whitespace
                crate="${crate%"${crate##*[![:space:]]}"}"

                validate_crate_option "$crate"

                if [[ -n $crate_name ]]; then
                    bash ./scripts/positivity.sh | sed "s/.*/${light_green}&${reset}/"
                    echo
                    break
                else
                    echo "${bright_red}Invalid crate option. Please try again.${reset}"
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
            bash ./scripts/goodbye.sh  
            exit 0
            ;;
        *)
            echo "${bright_red}Invalid option. Please try again.${reset}"
            ;;
    esac
done
