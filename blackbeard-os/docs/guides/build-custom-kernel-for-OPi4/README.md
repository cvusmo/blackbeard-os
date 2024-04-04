# Build custom Linux Kernel for Orange Pi 4 LTS
- All Commands use FISH shell
- 'λ' is used as a prompt indicator

## Step 1: Linaro ARM Cross-Compiler Installation
- Option 1: download the latest release visit: https://releases.linaro.org/components/toolchain/binaries/, choose a release and download the tarball gcc-linaro--x86_64_arm-linux-gnueabihf.tar.xz.
- Option 2: simply follow commands below to install 7.5-2019.12. 
##### This project uses 7.5-2019.12.

#### Download the Linaro ARM cross-compiler toolchain
```
λ wget -c https://releases.linaro.org/components/toolchain/binaries/7.5-2019.12/arm-linux-gnueabihf/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz
```

#### Extract the downloaded tarball
```
λ tar -xJf gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz
```

#### Check the path for CROSS_COMPILE
- Ensure the CROSS_COMPILE environment variable is correctly set. This variable specifies the prefix used for cross-compilation tools.
- You can verify its path by running:
```
λ echo $CROSS_COMPILE
```

#### Save the path to the extracted compiler binaries
- Set the CROSS_COMPILE environment variable to point to the extracted compiler binaries: 
```
λ export CC=`pwd`/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-
```

#### Set the CROSS_COMPILE Variable
```
λ set -x CROSS_COMPILE $PWD/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-
```

#### Verify the installation
- To verify that the cross-compiler toolchain is correctly installed, execute following command:

```
λ $CROSS_COMPILE"gcc" --version
```

- The output should resemble:
```
arm-linux-gnueabihf-gcc (Linaro GCC 7.5-2019.12) 7.5.0
Copyright (C) 2017 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
```

# Step 2: Build the Bootloader
A bootloader is a program that loads the Operating System kernel from secondary memory to primary memory. U-Boot is an open source bootloader that this guide will be using to load Linux. Download U-boot from Github using the following commands:

#### Clone u-boot
```
λ git clone https://github.com/u-boot/u-boot
λ cd u-boot/
``` 

#### Check for current version
```
λ git describe --tags
```
- If v2024.04 is outdated, replace it with the latest version obtained from the previous command output

#### Create 'tmp' branch
```
λ git checkout v2024.04 -b tmp
```

#### Configure and build U-boot
```
λ make ARCH=arm CROSS_COMPILE=$CC distclean
```

#### Replace <orangepi_model> with appropriate model name of your Orange Pi Board
- Example: ofile:///home/blackbeard/Projects/Blackbeard-OS/blackbeard-os/blackbeard-os/docs/guides/build-custom-kernel-for-OPi4/ArchLinuxARMrangepi_4_lts_defconfig
```
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE <orangepi_model>_defconfig
```

# Step 3: Build Linux Kernel
- The process of building the Linux kernel involves compiling the kernel source code into executable binaries and modules that form the core of the operating system. By building the kernel, you customize its configuration, optimize performance, and incorporate necessary device drivers and features tailored to your system's requirements.

#### Download the source code from Linux Kernel Archives
1. Visit https://www.kernel.org/ the official website for the Linux Kernel Archives.
2. Navigate to the "Longterm" section. 
3. Choose a version. This build guide uses 6.6.24 LTS (longterm)
4. Download the source code tarball (click [tarball])
5. Extract the tarball

```
λ tar -xvf linux-6.6.24.tar.xz
```

#### Navigate to the Kernel Source Directory:
```
λ cd linux-6.6.24/
```

#### Configure the Kernel
```
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE sunxi_defconfig
```

- If the kernel configuration is correct, the following message should appear:
```
#
# configuration written to .config
#
```
#### Customize Kernel Configuration (Optional)
```
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE menuconfig
```

#### Save the required DTB file
Option 1: SSH into Orange Pi from main PC to run SCP
``` 
λ scp orangepi@orangepi /boot/dtb-5.10.43/rockchip/<board>.dtb username@hostname:/path/to/destination/directory
Now go to location of <build>.dtb and save the <build>.dtb at ~/linux-x.x.x/arch/arm/boot/dts/<build>.dtb
```
Option 2: Locate <build>.dtb on OrangePi and save to USB.
```
λ find / -name "*.dtb"
Now go to location of <build>.dtb, copy and save onto USB
Save the <build>.dtb at ~/linux-x.x.x/arch/arm/boot/dts/<build>.dtb
```

#### Build the Kernel Image and Modules 
```
λ make -j$(nproc) ARCH=arm CROSS_COMPILE=$CROSS_COMPILE zImage modules <board>.dtb
```

#### Install Kernel Modules
```
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE modules_install
```

# Step 4: Root File System (I use ArchLinuxARM, BTW!)
Download a root file system and extract using the following commands:

```
λ wget -c http://os.archlinuxarm.org/os/ArchLinuxARM-aarch64-latest.tar.gz
λ tar -xvf ArchLinuxARM-aarch64-latest.tar.gz
```

# Step 5: Setup MicroSD card
Setup microSD card in to boot Orange Pi
```
Requirements:
    - file u-boot-sunxi-with-spl.bin
    - Linux kernel image 
    - <board>.dtb
    - root file system.
```

1. Create an empty image file
```
λ dd if=/dev/zero of=blackbeard-os-0.0.1-opi4-lts.img bs=1M count=2048
```
2. Partition the image file (e.g., using parted or fdisk)
```
λ sudo losetup -fP blackbeard-os-0.0.1-opi4-lts.img
λ sudo fdisk /dev/loop0
```
Follow these steps to create a partition:

    - Press n to create a new partition.
    - Choose the partition type (p for primary).
    - Specify the partition number (e.g., 1).
    - Press Enter to accept the default start sector.
    - Press Enter to accept the default end sector (to use the entire disk).
    - Press t to change the partition type.
    - Enter 83 as the partition type for Linux filesystem (ext4). 

3. Format the partitions
```
λ sudo mkfs.vfat /dev/loop0p1  # Format boot partition 
```
4. Mount the partitions
```
λ mkdir boot rootfs
λ sudo mount /dev/loopXp1 boot
λ sudo mount /dev/loopXp2 rootfs
```
5. Copy files to the image
```
λ cp u-boot-sunxi-with-sp1.bin boot
λ cp zImage boot
λ cp <board>.dtb boot
λ cp -a ArchLinuxARM/* rootfs  # Copy root file system contents
```
6. Unmount the partitions
```
λ sudo umount boot
λ sudo umount rootfs
```
7. Compress the image file
```
λ xz -z -k -9 blackbeard-os-0.0.1-opi4-lts.img
```

