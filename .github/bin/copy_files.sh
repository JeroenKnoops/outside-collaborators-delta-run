#!/bin/bash

# Set the destination directory where the files should be copied
dest_dir="delta"

mkdir -p $dest_dir/repos
mkdir -p $dest_dir/groups

# Set the file containing the list of filenames to copy
file_list="file_list.txt"

# Loop through each filename in the file list and copy it to the destination directory
# filename is in form of repos/name.yml or groups/name.yml
while read -r filename; do
    cp "$filename" "$dest_dir/$filename"
done < "$file_list"

# Remove old configurations
mv repos repos-all
mv groups groups-all

# Insert delta configurations
mv $dest_dir/repos repos
mv $dest_dir/groups groups
