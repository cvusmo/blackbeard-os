# Maintainers: 
# 
# This file is a customized profiledef.sh example used by the mkarchiso script.
# See https://gitlab.archlinux.org/archlinux/archiso/-/wikis for more information.

iso_name="blackbeard-live"
iso_label="BLACKBEARD_$(date +%Y%m)"
iso_publisher="Nicholas Jordan <blacksheepcosmo@gmail.com>"
iso_application="Blackbeard Live"
iso_version="0.0.1-001"
install_dir="arch"
buildmodes=('iso')
bootmodes=('uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito' 'bios.syslinux.mbr' 'bios.syslinux.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/etc/gshadow"]="0:0:400"
  ["/root"]="0:0:750"
  ["/root/.automated_script.sh"]="0:0:755"
  ["/etc/skel"]="0:0:755"
)

# Add any packages you want included in the ISO here
profile_base_packages=(
    base
    linux-zen
    linux-zen-headers
    linux-firmware
)

# Additional packages can be specified here
profile_custom_packages=(
    neovim
    git
    fish
    alacritty
    btop
    cava
    clang
    cmake
    dunst
    efibootmgr
    flameshot
    gimp
    grim
    gst-plugin-pipewire
    hyprland
    iwd
    libpulse
    meson
    ninja
    nvidia-dkms
    openssh
    pipewire
    pipewire-alsa
    pipewire-jack
    pipewire-pulse
    plymouth
    polkit-kde-agent
    qemu-base
    qt5-wayland
    qt6-5compat
    qt6-wayland
    sddm
    slurp
    smartmontools
    sof-firmware
    spotify-launcher
    steam
    thunar
    ttf-fantasque-nerd
    ttf-fira-code
    ttf-font-awesome
    ttf-jetbrains-mono
    unzip
    vim
    virt-manager
    waterfox-bin
    waybar
    wget
    wireless_tools
    wireplumber
    wofi
    wpa_supplicant
    xclip
    xdg-desktop-portal-hyprland
    xorg-server
    xorg-xinit
    yay
    yay-debug
    zip
    zram-generator
    plymouth-theme-sweet-arch-git
    hyprwayland-scanner-git
    vesktop-bin
    memtest86+
    memtest86+-efi
    edk2-shell
)

# Set hostname
hostname="blackbeard"

# Other configurations
run_once_scripts=(
    "/root/setup.sh"
    "/root/install.sh"
    "/root/post_install.sh"
)

