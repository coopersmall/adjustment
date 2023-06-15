#!/bin/bash

# Array of positive comments
positive_comments=(
    "Nice response, hold tight."
    "Great answer! Just a moment..."
    "You're doing great! Loading the next step."
    "Well done! The program is working on it."
    "Awesome! Let me handle the next step."
    "Excellent choice! Sit tight while I proceed."
    "Fantastic response! Loading the next task."
    "Impressive! I'll take care of the rest."
    "Superb answer! Just a little more to go."
    "Brilliant! The program is working its magic."
    "Terrific! Give me a moment to process."
    "Outstanding! Next step coming right up."
    "Wonderful response! Loading the next action."
    "You're on fire! Hold on while I proceed."
    "Very well! Sit back and relax for a moment."
    "Perfect! Let me handle the next operation."
    "Excellent job! I'm working on the next step."
    "Marvelous! The program is processing now."
    "Splendid answer! Loading the next task."
    "Great work! Give me a moment to proceed."
)

# Get the number of positive comments
num_comments=${#positive_comments[@]}

# Generate a random index
random_index=$((RANDOM % num_comments))

# Select a random positive comment
positive_comment=${positive_comments[random_index]}

# Echo the positive comment
echo "$positive_comment"
