#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

echo ""
echo "=== Checking Rust style with rustfmt... =============="

FOLDER="base client plantex plantex-server server"
FILES='.+\.rs'

ERROR=0
for f in $(find $FOLDER -regex $FILES); do
    # jesus I'm sorry, but I couldn't find a better way to use rustfmt :/
    if [ $(rustfmt --write-mode=checkstyle --skip-children $f | grep 'error' | wc -l) != 0 ]; then
        echo "! incorrectly formatted: $f"
        ERROR=1
    fi
done

test $ERROR == 0
