#!/bin/bash

## First compile the feature flag upgrade script
## We need to compile it for the CLI to run it on version 6, atleast, I had to locally.
movement move compile --package-dir movement-migration/framework-upgrades --bytecode-version 6

movement move run-script --compiled-script-path movement-migration/framework-upgrades/build/feature-flag-reconfig/bytecode_scripts/main.mv
