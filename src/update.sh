#!/bin/bash

# Update References Script for PDx Project
# Author: kartik4091
# Created: 2025-06-03 18:32:35 UTC

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting reference updates...${NC}"

# Function to update file contents
update_file() {
    local file=$1
    local old_name=$2
    local new_name=$3
    
    if [ ! -f "$file" ]; then
        echo -e "${RED}Error: File $file does not exist${NC}"
        return 1
    }
    
    # Create backup
    cp "$file" "${file}.bak"
    
    # Update references
    sed -i "s/${old_name}/${new_name}/g" "$file"
    
    echo -e "${GREEN}Updated references in: $file${NC}"
}

# Update module declarations
echo "Updating module declarations..."
sed -i 's/mod structure_handler;/mod handler;/' structure/mod.rs

# Update import statements
for file in $(find . -name "*.rs"); do
    # Update structure_handler references
    update_file "$file" "structure_handler" "handler"
    # Update structure_analyzer references
    update_file "$file" "structure_analyzer" "analyzer"
done

# Update documentation
echo "Updating documentation..."
for file in $(find . -name "*.rs"); do
    # Update comments and documentation
    update_file "$file" "Structure Handler" "Handler"
    update_file "$file" "Structure Analyzer" "Analyzer"
done

# Show git diff for review
if [ -d ".git" ]; then
    echo -e "\n${GREEN}Changes made:${NC}"
    git diff

    echo -e "\n${GREEN}Committing changes...${NC}"
    git add -A
    git commit -m "docs: update references after file renames

- Update module declarations
- Fix import statements
- Update documentation references
- Fix test module paths

Timestamp: 2025-06-03 18:32:35 UTC
Author: kartik4091"
fi

echo -e "${GREEN}Reference updates completed.${NC}"
