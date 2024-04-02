# blackbeard-os

* This file is part of Blackbeard OS, a distro for Arch Linux Arm, btw.

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


## Project Structure

blackbeard-os/
│
├── boot/
│   ├── configs/              # Configuration files for boot loaders, kernels, etc.
│   └── scripts/              # Scripts for boot process customization, if any
│
├── docs/                     # Documentation for the project, setup guides, etc.
│
├── os/                       # All OS level customizations and source code
│   ├── kernel/               # Custom kernel source and build tools
│   │   ├── patches/          # Kernel patches, if any
│   │   └── configs/          # Kernel configuration files
│   │
│   └── rootfs/               # Root filesystem modifications, overlays, etc.
│       ├── etc/              # System-wide configuration files
│       └── usr/              # Additional system binaries, libraries, etc.
│
├── src/                      # Source code for any custom utilities or applications
│
├── themes/                   # Themes and UI customizations
│   ├── sddm-blackbeard-theme/ # SDDM theme
│   └── icons/                # Icon themes
│
└── dist/                     # Distribution artifacts, pre-built images, packages
    ├── images/               # Pre-built OS images for direct download
    └── packages/             # Custom packages or binaries for installation


Each directory is equipped with a `README.md` providing more details on its contents and purpose.

