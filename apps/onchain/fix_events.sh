#!/bin/bash

# Fix deprecated Symbol::short usage
find contracts -name "*.rs" -exec sed -i 's/Symbol::short(/symbol_short!(/g' {} \;
find contracts -name "*.rs" -exec sed -i 's/Symbol::short("/symbol_short!("/g' {} \;

# Fix contractevent imports
find contracts -name "events.rs" -exec sed -i 's/use soroban_sdk::{contractevent,/use soroban_sdk::{/g' {} \;

# Remove contractevent attributes and topic attributes
find contracts -name "events.rs" -exec sed -i '/#\[contractevent\]/d' {} \;
find contracts -name "events.rs" -exec sed -i '/#\[topic\]/d' {} \;

# Convert event structs to simple functions (basic pattern)
find contracts -name "events.rs" -exec sed -i 's/pub struct \([A-Za-z]*Event\) {/pub fn \L\1(env: \&Env) {/g' {} \;

echo "Basic event fixes applied. Manual fixes still needed for complex events."
