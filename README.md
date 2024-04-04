
<p align="center">
  <img src="https://cdn.discordapp.com/attachments/712174300291137638/1225122047890297023/blackbeard-os-icon.png?ex=661ffad1&is=660d85d1&hm=eec3f1f4dd43c71d1d759519a9cbc7ce3af3d31ece4adffc4e2a6da135cfc878&" alt="Blackbeard OS Icon" width="256" height="256">
</p>

<p align="center">
This file is part of Blackbeard-OS, a distro for ARM Arch Linux btw.
  
```bash
  Copyright (c) 2024 Nicholas Jordan <blacksheepcosmo@gmail.com>

* Permission is hereby granted, free of charge, to any person
  obtaining a copy of this software and associated documentation
  files (the "Software"), to deal in the Software without restriction,
  including without limitation the rights to use, copy, modify, merge,
  publish, distribute, sublicense, and/or sell copies of the Software,
  and to permit persons to whom the Software is furnished to do so,
  subject to the following conditions:

* The above copyright notice and this permission notice shall be included
  in all copies or substantial portions of the Software.

* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
  OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
  OR OTHER DEALINGS IN THE SOFTWARE. 
```
</p>
  
## Project Structure

```
  
blackbeard-os/
│
├── boot/
│   ├── configs/            # Configuration files for boot loaders, kernels, etc.
│   └── scripts/            # Scripts for boot process customization, if any
│
├── docs/                   # Documentation for the project, setup guides, etc.
│   ├── guides/
│   │   └── build-custom-kernel-for-OPi4/
│   └── ascii/
│
├── os/                     # All OS level customizations and source code
│   ├── kernel/             # Custom kernel source and build tools
│   │   ├── patches/        # Kernel patches, if any
│   │   └── configs/        # Kernel configuration files
│   └── rootfs/             # Root filesystem modifications, overlays, etc.
│       ├── etc/            # System-wide configuration files
│       └── usr/            # Additional system binaries, libraries, etc.
│
├── src/                    # Source code for any custom utilities or applications
│
├── themes/                 # Themes and UI customizations
│   ├── sddm-blackbeard-theme/   # SDDM theme
│   └── icons/              # Icon themes
│
└── dist/                   # Distribution artifacts, pre-built images, packages
    ├── images/             # Pre-built OS images for direct download
    └── packages/           # Custom packages or binaries for installation
```

```Each directory is equipped with a `README.md` ```


