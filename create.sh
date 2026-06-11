#!/bin/bash

# remove the existing database
if [ -f "apps.db" ]; then
  printf "removing existing database... "
  rm -rf apps.db
  printf "done.\n"
else
  printf "no database exists to remove.\n"
fi

# create the new/replacement database
printf "running db creator.\n";
cargo run --bin dashboard-data

# verify success
if [ -f "apps.db" ]; then
  printf "database creation successful.\n"
else
  printf "error: database doesn't exist.\n"
fi
