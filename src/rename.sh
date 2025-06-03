#!/bin/bash

# Rename Script for PDx Project
# Author: kartik4091
# Created: 2025-06-03 18:15:52 UTC

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting file rename operations...${NC}"

# Array of renames: [source]=destination
declare -A files=(
    ["analyzer/structure_handler.rs"]="analyzer/handler.rs"
    ["analyzer/structure_analyzer.rs"]="analyzer/analyzer.rs"
    ["structure/structure_handler.rs"]="structure/handler.rs"
)

# Function to handle file operations
rename_file() {
    local src=$1
    local dst=$2
    
    # Check if source exists
    if [ ! -f "$src" ]; then
        echo -e "${RED}Error: Source file $src does not exist${NC}"
        return 1
    }
    
    # Create destination directory if it doesn't exist
    mkdir -p $(dirname "$dst")
    
    # Perform the move
    mv "$src" "$dst"
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Successfully renamed: $src -> $dst${NC}"
    else
        echo -e "${RED}Failed to rename: $src -> $dst${NC}"
        return 1
    fi
}

# Perform renames
for src in "${!files[@]}"; do
    dst=${files[$src]}
    rename_file "$src" "$dst"
done

echo -e "${GREEN}File rename operations completed.${NC}"

# Git operations if in a git repository
if [ -d ".git" ]; then
    echo -e "${GREEN}Updating git repository...${NC}"
    git add .
    git status
    echo -e "${GREEN}Please review changes and commit if satisfied${NC}"
fi
