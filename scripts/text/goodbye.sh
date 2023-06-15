#!/bin/bash

# Function to display a friendly goodbye message
display_goodbye_message() {
    # Array of goodbye messages
    goodbye_messages=("See ya next time!"
                      "Goodbye! Take care!"
                      "Farewell! Have a great day!"
                      "Until we meet again!"
                      "Take care and goodbye!"
                      "Wishing you all the best!"
                      "Goodbye! Stay awesome!"
                      "See you soon! Farewell!"
                      "Have a wonderful day!"
                      "Bye bye! Stay safe!"
                      "Goodbye! Keep smiling!"
                      "Take care and see you later!"
                      "Wishing you a fantastic day!"
                      "Goodbye! Keep up the great work!"
                      "See you next time! Stay positive!"
                      "Have a fantastic journey!"
                      "Farewell! May success be with you!"
                      "Take care of yourself! Goodbye!"
                      "Wishing you happiness and success!"
                      "Goodbye! Remember to stay awesome!")

    # Generate a random index
    local random_index=$(( RANDOM % ${#goodbye_messages[@]} ))

    # Display the random goodbye message
    echo "${goodbye_messages[$random_index]}"
}

# Clear the screen
clear

# Print the goodbye message
display_goodbye_message
