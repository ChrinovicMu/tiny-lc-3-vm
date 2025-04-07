#!/bin/bash

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Build successful, proceeding with commit..."
else
    echo "Build failed, aborting commit."
    exit 1
fi

# Stage all changes
git add .

# Commit with a default message or use an argument
COMMIT_MESSAGE=${1:-"Automated commit: Update project files"}
git commit -m "$COMMIT_MESSAGE"

# Push to GitHub (main branch)
git push origin main
