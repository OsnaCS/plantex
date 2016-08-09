#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

MY_PATH="`dirname \"$0\"`"

# basic style check
$MY_PATH/check-basic-style.sh

# check that everything is formatted with rustfmt
# EDIT: we don't want to check it anymore...
# $MY_PATH/check-rustfmt.sh

# add validator to PATH
# validate shaders only if glslangValidator was successfully installed
# the tool only works on linux so I make this check to run run_all.sh on osx
VALIDATOR_PATH=`find $PWD -name glslangValidator`
if [ ! -z $VALIDATOR_PATH ]; then
    export PATH=$PATH:`dirname $VALIDATOR_PATH`
    $MY_PATH/validate-shaders.sh
else
    echo ""
    echo "======= glslangValidator was not found! ======"
    echo "...skipping shader validation..."
fi

# check that everything compiles and all tests pass
$MY_PATH/test-all.sh

echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
echo "+              Everything is fine!                 +"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
