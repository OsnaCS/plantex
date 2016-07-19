#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

# $PATH must be set correctly before invoking this

if ! type ghp-import > /dev/null; then
    echo ""
    echo "=== Installing ghp-import ================"
    pip install ghp-import --user
fi

if ! type rustfmt > /dev/null; then
    echo ""
    echo "=== Installing rustfmt ==============="
    cargo install rustfmt
fi

type ghp-import > /dev/null
type rustfmt > /dev/null
