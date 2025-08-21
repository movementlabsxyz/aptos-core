#!/bin/bash

# Check if exactly 2 arguments are provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <folder_to_clean> <reference_folder>"
    echo "Recursively deletes files from folder_to_clean that don't exist in reference_folder,"
    echo "then syncs reference_folder to folder_to_clean using rsync"
    exit 1
fi

folder_to_clean="$1"
reference_folder="$2"

# Check if both directories exist
if [ ! -d "$folder_to_clean" ]; then
    echo "Error: Directory '$folder_to_clean' does not exist"
    exit 1
fi

if [ ! -d "$reference_folder" ]; then
    echo "Error: Directory '$reference_folder' does not exist"
    exit 1
fi

# Convert to absolute paths to avoid issues
folder_to_clean=$(realpath "$folder_to_clean")
reference_folder=$(realpath "$reference_folder")

echo "Cleaning folder: $folder_to_clean"
echo "Reference folder: $reference_folder"
echo

# Ask for confirmation before proceeding
echo "This will:"
echo "1. Delete all files in '$folder_to_clean' that don't exist in '$reference_folder'"
echo "2. Remove empty directories"
echo "3. Sync '$reference_folder' to '$folder_to_clean' using rsync"
echo
read -p "Do you want to continue? (y/N): " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 0
fi

echo

# Phase 1: Delete files that don't exist in reference folder
echo "=== Phase 1: Cleaning up files not in reference folder ==="
deleted_count=0

# Use find to get all files in folder_to_clean, preserving the relative path structure
while IFS= read -r -d '' file; do
    # Get the relative path from folder_to_clean
    relative_path="${file#"$folder_to_clean"/}"

    # Check if this file exists in the same relative location in reference folder
    reference_file="$reference_folder/$relative_path"

    if [ ! -f "$reference_file" ]; then
        echo "Deleting: $file"
        rm "$file"
        ((deleted_count++))
    fi
done < <(find "$folder_to_clean" -type f -print0)

echo "Files deleted: $deleted_count"
echo

# Phase 2: Remove empty directories
echo "=== Phase 2: Removing empty directories ==="
# Find and remove empty directories (bottom-up)
find "$folder_to_clean" -type d -empty -delete 2>/dev/null
echo "Empty directories removed"
echo

# Phase 3: Sync with rsync
echo "=== Phase 3: Syncing reference folder to cleaned folder ==="
echo "Running: rsync -av --progress \"$reference_folder/\" \"$folder_to_clean/\""
echo

# Use rsync to sync the reference folder to the cleaned folder
# -a: archive mode (preserves permissions, timestamps, etc.)
# -v: verbose
# --progress: show progress
# Note the trailing slashes are important for rsync behavior
rsync -av --progress "$reference_folder/" "$folder_to_clean/"

sync_exit_code=$?

if [ $sync_exit_code -eq 0 ]; then
    echo
    echo "=== Cleanup and sync completed successfully ==="
    echo "The folder '$folder_to_clean' now matches '$reference_folder'"
else
    echo
    echo "=== WARNING: rsync failed with exit code $sync_exit_code ==="
    echo "Please check the rsync output above for errors"
    exit $sync_exit_code
fi
