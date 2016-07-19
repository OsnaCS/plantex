#!/bin/bash

# Checks
# - trailing whitespaces (not allowed)
# - single trailing newline (required)
# - bad windows/mac line endings
# - tab characters
# - lines longer than XX chars

# config
COLS=100
FOLDER="base client plantex plantex-server server"
FILES='.+\.\(rs\|vert\|tesc\|tese\|geom\|frag\|comp\)'


# Exit script on the first error
set -o errexit -o nounset

ERROR=0
### Trailing Whitespaces ===========================
echo ""
echo "=== Searching for lines with trailing whitespace... ==================="
FOUNDTW=0
for f in $(find $FOLDER -regex $FILES); do
    if egrep -q " +$" $f ; then
        echo "! Has trailing whitespace: $f"
        FOUNDTW=1
    fi
done

if [ $FOUNDTW -eq 0 ] ; then
    echo "=== None found! :-)"
else
    echo ""
    echo "!!! Some lines were found. Please remove the trailing whitespace!"
    ERROR=1
fi

### Trailing newlines ===============================
echo ""
echo "=== Searching for files without trailing newline... ==================="
FOUND=0
for f in $(find $FOLDER -regex $FILES); do
    lastline=$(tail -n 1 $f; echo x)
    lastline=${lastline%x}
    if [ "${lastline: -1}" != $'\n' ] ; then
        echo "! Has no single trailing newline: $f"
        FOUND=1
    fi
done

if [ $FOUND -eq 0 ] ; then
    echo "=== None found! :-)"
else
    echo ""
    echo "!!! Some files were found. Please add a single trailing newline!"
    ERROR=1
fi

### windows and mac OS line endings =======================
echo ""
echo "=== Searching for files with wrong line endings ==================="

FOUNDLE=0
for f in $(find $FOLDER -regex $FILES); do
    if grep -q $'\r' $f ; then
        echo "! Has windows/mac line ending: $f"
        FOUNDLE=1
    fi
done

if [ $FOUNDLE -eq 0 ] ; then
    echo "=== None found! :-)"
else
    echo ""
    echo "!!! Some lines were found. Please use unix line endings!"
    ERROR=1
fi

## windows and mac OS line endings =======================
echo ""
echo "=== Searching for files with tab chars ==================="

FOUNDTAB=0
for f in $(find $FOLDER -regex $FILES); do
    if grep -q $'\t' $f ; then
        echo "! Has tab character: $f"
        FOUNDTAB=1
    fi
done

if [ $FOUNDTAB -eq 0 ] ; then
    echo "=== None found! :-)"
else
    echo ""
    echo "!!! Some files were found. Please indent with spaces only!"
    ERROR=1
fi



### char limit ===================================
echo ""
echo "=== Searching for files with too long lines... ========================"
FOUND=0
for f in $(find $FOLDER -regex $FILES); do
    if [ $(wc -L $f | cut -d" " -f1) -gt $COLS ] ; then
        echo "! Line with more than $COLS chars in $f"
        FOUND=1
    fi
done

if [ $FOUND -eq 0 ] ; then
    echo "=== None found! :-)"
else
    echo ""
    echo "!!! Some files were found. Please shorten those lines!"
    ERROR=1
fi

test $ERROR == 0
