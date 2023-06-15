#!/bin/bash

# Function to display a random error exit message
display_error_exit_message() {
    # Array of error exit messages
    error_exit_messages=("Oops! Something went wrong."
                         "Looks like something didn't go as planned."
                         "Uh-oh! We encountered an unexpected error."
                         "Well, this is awkward."
                         "Oops! We hit a little snag."
                         "Oh no! An error occurred."
                         "Hmmm... Something went wrong."
                         "Yikes! An error has occurred."
                         "Oh dear! We ran into an issue."
                         "Oopsie-daisy! An error popped up."
                         "Well, that didn't go as expected."
                         "Looks like we have a problem here."
                         "Oops! It seems we have a hiccup."
                         "Uh-oh! Something went haywire."
                         "Oops-a-daisy! An error crept in."
                         "Oh boy! An error has sneaked in."
                         "Whoopsie! An error is in the house."
                         "Oh snap! We encountered an error."
                         "Houston, we have a problem."
                         "Well, that didn't quite work out.")

    # Generate a random index
    local random_index=$(( RANDOM % ${#error_exit_messages[@]} ))

    # Display the random error exit message
    echo "${error_exit_messages[$random_index]}"
}

# Clear the screen
clear

# Print the error exit message
display_error_exit_message
