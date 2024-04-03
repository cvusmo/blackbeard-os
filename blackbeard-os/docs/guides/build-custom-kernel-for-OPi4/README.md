# Build custom Linux Kernel for Orange Pi 4 LTS
## All Commands use FISH shell
## The 'λ' is used as a prompt indicator

# Step 1: Linaro ARM Cross-Compiler Installation
### To download the latest release visit: https://releases.linaro.org/components/toolchain/binaries/, choose a release and download the tarball gcc-linaro--x86_64_arm-linux-gnueabihf.tar.xz or simply follow commands below to install 7.5-2019.12. This project uses 7.5-2019.12.

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
λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- sunxi_defconfig

## Customize Kernel Configuration (Optional)
λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- menuconfig

## Build the Kernel Image and Modules
λ make -j$(nproc) ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- zImage modules <board>.dtb

## Install Kernel Modules
λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- modules_install

For Orange Pi PC Plus the dtb file to compile is sun8i-h3-orangepi-pc-plus.dtb. With the tool menuconfig you can configure the Linux source with a menu-driven user interface.
At the end of the building you can find the zImage file in the directory <kernel_path>/arch/arm/boot  and the dtb file in the directory <kernel_path>/arch/arm/boot/dts.

Download a root file system
The next step it's the building of the Linux kernel. First of all download the source code by the Linux Kernel Archives. It's suggested to download a stable or longterm version of Linux kernel. We try the version 4.14. When you finish to download the kernel source code extract it. Then you to have to configure and build the kernel. Open a Linux terminal and type the following commands:

λ cd <kernel_path>

λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- sunxi_defconfig

λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- menuconfig

λ make -j$(nproc) ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- zImage modules <board>.dtb

λ make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- modules_install

For Orange Pi PC Plus the dtb file to compile is sun8i-h3-orangepi-pc-plus.dtb. With the tool menuconfig you can configure the Linux source with a menu-driven user interface.
At the end of the building you can find the zImage file in the directory <kernel_path>/arch/arm/boot  and the dtb file in the directory <kernel_path>/arch/arm/boot/dts.

Setup MicroSD card
This section explains how to setup a microSD card in order to boot your Orange Pi board. You need:

    the file u-boot-sunxi-with-spl.bin;
    the Linux kernel image (uImage or zImage);
    the specific dtb file for the board;
    a root file system.

The first step is to insert the microSD card in your computer and identify it. Usually it should be /dev/mmcblk0. The following steps expect that your SD card is /dev/mmcblk0; if your computer identify the device differently use its name instead of /dev/mmcblk0.
Then create two partitions in your microSD card: the first one is the bootable partition that contains the kernel image, the dtb file and the eventual uEnv.txt file, the second partition contains the root file system. To create the partitions you can use the Linux command fdisk:

$ sudo fidsk /dev/mmcblk0

In order to create the two partitions:

Command (m for help): n                                 # Type n
Partition type:
   p   primary (0 primary, 0 extended, 4 free)
   e   extended
Select (default p):                                     # Press Enter Key      
Using default response p
Partition number (1-4, default 1):                      # Press Enter Key
Using default value 1
First sector (2048-15523839, default 2048):             # Press Enter Key
Using default value 2048
Last sector, +sectors or +size{K,M,G} (2048-15523839, default 15523839): +20M      # Type +20M

Command (m for help): n                                   # Type n    
Partition type:                                           
   p   primary (1 primary, 0 extended, 3 free)
   e   extended
Select (default p):                                       # Press Enter Key
Using default response p
Partition number (1-4, default 2):                        # Press Enter Key
Using default value 2
First sector (43008-15523839, default 43008):
Using default value 43008
Last sector, +sectors or +size{K,M,G} (43008-15523839, default 15523839): # Press Enter Key
Using default value 15523839

Then you have to format the two partitions, the first with a FAT file system, the second with an EXT4 file system. You can use Linux mkfs command:

$ sudo mkfs.vfat /dev/mmcblk0p1

$ sudo mkfs.ext /dev/mmcblk0p2

Now you have to clean the partition table of your microSD card  and write the u-boot-sunxi-with-spl.bin typing these commands:

$ dd if=/dev/zero of=/dev/mmcblk0 bs=1k count=1023 seek=1

$ dd if=<u-boot_path>/u-boot-sunxi-with-spl.bin of=/dev/mmcblk0 bs=1024 seek=8

Now you have to copy in the first partition the kernel image and the specific dtb file for the board (for Orange Pi PC Plus it's sun8i-h3-orangepi-pc-plus.dtb). In order to do that type:

$ sudo mount /dev/mmcblk0p1 /mnt

$ sudo cp -v <kernel_path>/arch/arm/boot/zImage /mnt

$ sudo cp -v <kernel_path>/arch/arm/dts/<board>.dtb /mnt

$ sudo umount /mnt

Since Orange Pi boards don't use by default uEnv.txt file it's suggested to not create it. 
Then you have to copy the root file system and the kernel modules in the second partition with the following commands:

$ sudo mount /dev/mmcblk0p2

$ sudo tar xfvp <root_file_system_path>/<root_file_system>.tar.gz -C /mnt

$ sync

$ sudo chown root:root /mnt

$ sudo chmod 755 /mnt

$ sudo cp -rv <kernel_modules_path>/* /mnt/lib

$ sync

$ sudo umount /mnt

Now you have a bootable microSD card and you are ready to use your Orange Pi board.
There is only a last thing to do: since you don't use the uEnv.txt file, at the first boot  you have to set an U-boot environment variable that contains the instrunctions to execute at the boot. So, at the first boot, type any key to stop autoboot and enter in U-boot command line. Then you have to set the environment variable bootargs and modify the variable bootcmd, executed by default at boot. At the end save the modified environment and boot the board.

=> setenv bootargs console=${console} root=/dev/mmcblk0p2 rw rootwait

=> setenv bootcmd `mmc rescan; fatload mmc 0 0x46000000 zImage; fatload mmc 0 0x49000000 ${fdtfile}; bootz 0x46000000 - 0x49000000`

=> saveenv

=> boot

At the next boots the board will boot automatically. Now you can enjoy your Orange Pi board.
