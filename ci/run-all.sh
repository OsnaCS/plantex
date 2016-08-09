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
VALIDATOR_PATH=`find $PWD -name glslangValidator`
export PATH=$PATH:`dirname $VALIDATOR_PATH`

# validate shaders
$MY_PATH/validate-shaders.sh

# check that everything compiles and all tests pass
$MY_PATH/test-all.sh

echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
echo "+              Everything is fine!                 +"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
