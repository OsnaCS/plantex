#!/bin/bash

set -o errexit -o nounset

echo ""
echo "=== Validating Shaders =============="

SHADER_DIR=`find $PWD -name shader`

ERROR_TMP=".shader_errors"
trap "rm $ERROR_TMP; exit" SIGHUP SIGINT SIGTERM

# Make sure only shaders are in client/shader/ for this to work
for shader in $SHADER_DIR/*; do
    echo "Validating: `basename $shader` ..."
    if ! glslangValidator -s $shader > /dev/null;
        then
        echo "=== ERRORS in `basename $shader` =============" >> $ERROR_TMP
        glslangValidator $shader | grep "ERROR" >> $ERROR_TMP
        echo "" >> $ERROR_TMP
    fi
done

echo ""
echo "=== Validating Shaders Done! ============"

if [ -s $ERROR_TMP ]
    then
    echo ""
    echo "=== FOUND ERRORS IN SOME SHADERS! =============="
    echo ""
    ERROR=1
    # Print Errors & Cleanup
    cat $ERROR_TMP
    rm $ERROR_TMP
else
    echo ""
    echo "=== Shaders seem fine! =============="
    ERROR=0
fi

test $ERROR == 0
