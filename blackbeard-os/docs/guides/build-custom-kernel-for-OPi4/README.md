# Build custom Linux Kernel for Orange Pi 4 LTS
- All Commands use FISH shell
- 'λ' is used as a prompt indicator

## Step 1: TF-A ARM Cross-Compiler Installation

#### Download the TF-A ARM cross-compiler toolchain
```
λ git clone --depth 1 https://github.com/ARM-software/arm-trusted-firmware.git
```

#### Install aarch64-linux-gnu-gcc 
```
λ sudo pacman -S aarch64-linux-gnu-gcc
```

#### Build TF-A
Specify the PLAT= with desired Rockchip platform to build TF-A
```
cd arm-trusted-firmware
make realclean
make CROSS_COMPILE=aarch64-linux-gnu- PLAT=rk3399
cd ..
```

#### (OPTIONAL) Check the path for CROSS_COMPILE
- Ensure the CROSS_COMPILE environment variable is correctly set. This variable specifies the prefix used for cross-compilation tools.
- You can verify its path by running:
```
λ echo $CROSS_COMPILE
```

#### Verify the installation
- To verify that the cross-compiler toolchain is correctly installed, execute following command:

```
λ $CROSS_COMPILE"gcc" --version
```

- The output should resemble:
```
aarch64-linux-gnu-gcc (GCC) 13.2.0
Copyright (C) 2023 Free Software Foundation, Inc.
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
Navigate to ~/u-boot/configs to search for <orangepi_model>_defconfig as of 20240404
- orangepi-5-plus-rk3588_defconfig
- orangepi-5-rk3588s_defconfig
- orangepi_zero_plus_defconfig
- orangepi_zero_plus2_h3_defconfig
- orangepi_zero_plus2_defconfig
- orangepi_zero_defconfig
- orangepi_zero3_defconfig
- orangepi_zero2w_defconfig
- orangepi_zero2_defconfig
- orangepi_win_defconfig
- orangepi-r1-plus-lts-rk3328_defconfig
- orangepi-r1-plus-rk3328_defconfig
- orangepi-rk3399_defconfig
- orangepi_2_defconfig
- orangepi_3_defconfig
- Orangepi_defconfig
- orangepi_lite2_defconfig
- orangepi_lite_defconfig
- Orangepi_mini_defconfig
- orangepi_one_defconfig
- orangepi_one_plus_defconfig
- orangepi_pc2_defconfig
- orangepi_pc_defconfig
- orangepi_pc_plus_defconfig
- orangepi_plus2e_defconfig
- orangepi_plus_defconfig
- orangepi_prime_defconfig
- orangepi_r1_defconfig
```
λ make ARCH=arm CROSS_COMPILE=$CC distclean
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE <orangepi_model>_defconfig
λ make ARCH=arm CROSS_COMPILE=$CROSS_COMPILE
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
    - Press w to save and exit.

3. Format the partitions
```
λ sudo mkfs.vfat /dev/loop0p1  # Format boot partition 
```
4. Mount the partitions
```
λ mkdir boot
λ sudo mount /dev/loop0p1 boot
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

