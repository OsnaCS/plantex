#!/bin/bash
set -e

# check for validator folder (travis caching)
if [ ! -d "validator/" ]; then
    echo "need to install glslangValidator..."
    mkdir validator/
    cd validator/
    wget "https://cvs.khronos.org/svn/repos/ogl/trunk/ecosystem/public/sdk/tools/glslang/Install/Linux/glslangValidator"
    chmod +x glslangValidator
    cd ../
    echo "installed glslangValidator to $PWD/validator/glslangValidator"
else
    echo "using cached glslangValidator"
fi
