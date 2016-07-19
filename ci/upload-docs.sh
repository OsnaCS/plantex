#!/bin/bash

# Exit script on the first error
set -e

echo ""
echo "=== Generating documentation ================="
cargo doc

if [ "$TRAVIS_BRANCH" == "master" ] && [ "$TRAVIS_PULL_REQUEST" == "false" ]; then
    echo ""
    echo "=== Uploading docs ==============="
    ghp-import -n target/doc
    git push -qf https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
fi
