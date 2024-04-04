# Build custom Linux Kernel for Orange Pi 4 LTS
- All Commands use FISH shell
-'λ' is used as a prompt indicator

# Step 1: Linaro ARM Cross-Compiler Installation
To download the latest release visit: https://releases.linaro.org/components/toolchain/binaries/, choose a release and download the tarball gcc-linaro--x86_64_arm-linux-gnueabihf.tar.xz or simply follow commands below to install 7.5-2019.12. This project uses 7.5-2019.12.

## Download the Linaro ARM cross-compiler toolchain
λ wget -c https://releases.linaro.org/components/toolchain/binaries/7.5-2019.12/arm-linux-gnueabihf/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz

## Extract the downloaded tarball
λ tar -xJf gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz

## Check the path for CROSS_COMPILE
### Ensure the CROSS_COMPILE environment variable is correctly set. This variable specifies the prefix used for cross-compilation tools. You can verify its path by running:
λ echo $CROSS_COMPILE

## Save the path to the extracted compiler binaries
### Set the CROSS_COMPILE environment variable to point to the extracted compiler binaries: 
λ export CC=`pwd`/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-

## Set the CROSS_COMPILE Variable
λ set -x CROSS_COMPILE $PWD/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-

## Verify the installation
### To verify that the cross-compiler toolchain is correctly installed, execute following command in terminal with FISH shell:
λ $CROSS_COMPILE"gcc" --version

### The output should resemble:
```
arm-linux-gnueabihf-gcc (Linaro GCC 7.5-2019.12) 7.5.0
Copyright (C) 2017 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
```

# Step 2: Build the Bootloader
A bootloader is a program that loads the Operating System kernel from secondary memory to primary memory. U-Boot is an open source bootloader that this guide will be using to load Linux. Download U-boot from Github using the following commands:

## Clone u-boot
λ git clone https://github.com/u-boot/u-boot
λ cd u-boot/

## Check for current version
### Check for current version
λ git describe --tags

## If v2024.04 is outdated, replace it with the latest version obtained from the previous command output 
### Creates 'tmp' branch
λ git checkout v2024.04 -b tmp

## Configure and build U-boot
λ make ARCH=arm CROSS_COMPILE=$CC distclean

## Replace <orangepi_model> with appropriate model name of your Orange Pi Board
### Example: orangepi_4_lts_defconfig
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE <orangepi_model>_defconfig 

# Step 3: Build Linux Kernel
The process of building the Linux kernel involves compiling the kernel source code into executable binaries and modules that form the core of the operating system. By building the kernel, you customize its configuration, optimize performance, and incorporate necessary device drivers and features tailored to your system's requirements.

## Download the source code from Linux Kernel Archives
### 1. Visit https://www.kernel.org/ the official website for the Linux Kernel Archives.
### 2. Navigate to the "Longterm" section. 
### 3. Choose a version. This build guide uses 6.6.24 LTS (longterm)
### 4. Download the source code tarball (click [tarball])
### 5. Extract the tarball
λ tar -xvf linux-6.6.24.tar.xz

## Navigate to the Kernel Source Directory:
λ cd linux-6.6.24/

## Configure the Kernel
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE sunxi_defconfig

If the kernel configuration is correct, the following message should appear:
```
#
#
#configuration written to .config
```

## Customize Kernel Configuration (Optional)
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE menuconfig

## Build the Kernel Image and Modules
Example of <board>: 
λ make -j$(nproc) ARCH=arm CROSS_COMPILE=$CROSS_COMPILE zImage modules <board>.dtb

## Install Kernel Modules
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE modules_install
