#!/usr/bin/env bash
# Bash script will eventually be replaced with a make file

TARGET=$1

# Check we need a library??
cd ../rust-sdl2
    SDL_MODE=dylib make
    RESULT=$?
    if [ $RESULT -ne 0 ] ; then
        echo "Failed to make dependancies"
        cd -
        exit $RESULT
    fi
cd -

# Make the bin directory
if [ ! -d "bin" ]; then
    mkdir bin
fi
# Make the libs directory
if [ ! -d "libs" ]; then
    mkdir libs
fi
# Scrape them
rm libs/*
cp ../rust-sdl2/*.dummy libs
cp ../rust-sdl2/*.so libs
cp ../rust-sdl2/*.dylib libs
cp ../rust-sdl2/*.dll libs

# Check what project we want to build, or all of them
# Hack it for now
if [ $TARGET ]; then
    cd src/$TARGET
        ./build.sh
    cd -
else
    DIRS=$(find src -maxdepth 1 -type d )
    for DIR in $DIRS; do
        if [ ! $DIR == 'src' ]; then
            cd $DIR
            echo "Building $DIR..."
            ./build.sh
            cd -
        fi
    done
fi
