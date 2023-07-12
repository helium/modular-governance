#!/bin/bash

programs_dir="programs"

# Get the list of program names from the programs directory
program_list=$(ls "$programs_dir")

# Run anchor idl init for each program
for program in $program_list; do
  id=$(toml get Anchor.toml programs.localnet.$program | tr -d '"')
  filepath="target/idl/${program}.json"
  cluster="${1:-localnet}"

  if [ -n "$id" ]; then
    anchor_command="anchor idl init ${id} --filepath ${filepath} --provider.cluster ${cluster} --provider.wallet $HOME/.config/solana/id.json"
    echo "Running command: $anchor_command"

    # Run the anchor idl init command in the background
    ($anchor_command &)
  else
    echo "Skipping program $program. ID is empty."
  fi
done

wait