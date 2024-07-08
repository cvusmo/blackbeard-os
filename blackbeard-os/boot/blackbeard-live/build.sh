#!/bin/bash

# Ensure script is run as root
if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

# Define the build directory
BUILD_DIR=$(pwd)

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf ${BUILD_DIR}/work
rm -rf ${BUILD_DIR}/out

# Exclude unnecessary directories
EXCLUDES="--exclude=./airootfs/home/echo/projects --exclude=./airootfs/root/Downloads"

# Build the ISO
echo "Building the ISO..."
mkarchiso -v ${EXCLUDES} ${BUILD_DIR}

echo "ISO build process completed."
