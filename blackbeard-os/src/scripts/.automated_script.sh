#!/usr/bin/env bash

# Function to install packages
install_packages() {
  sudo pacman -Sy --noconfirm acpi alacritty arch-audit archiso autoconf-archive base base-devel bc bless btrfs-progs cava cmake cpio dunst efibootmgr electron25-bin elfutils firefox fish fuse2 gimp git grim gst-libav gst-plugin-pipewire gtk-layer-shell gucharmap hypr-git hyprland-git hyprlang-git hyprpaper hyprpaper-git-debug intel-medi-driver intel-ucode iwd jq lazygit libpulse libva-intel-driver linux-firmware linux-zen linux-zen-headers linuxconsole lynis macbook-lighter materia-gtk-theme meson nasm neovim neovim-nvim-treesitter neovim-web-devicons-git net-tools network-manager-applet networkmanager noto-fonts-emoji nvidia nvim-treesitter-parsers-git obsidian p7zip pacman pacman-contrib papirus-icon-theme pipewire pipewire-alsa pipewire-jack pipewire-pulse plymouth polkit-gnome pommed-light pommed-light-debug power-profiles-daemon pulsemixer qemu-base qpwgraph qt5-graphicaleffects qt5-multimedia qt5-quickcontrols qt5-virtualkeyboard qt5-wayland qt5ct rustup scdoc sddm setconf slurp smartmontools sudo sway swaybg swayidle swaylock-git ttf-joypixels ttf-symbola ttf-twemoji ttf-twemoji-color unzip usbimager vesktop-bin vulkan-intel waybar wev wget wireless_tools wireplumber wofi xclip xdg-desktop-portal-gtk xdg-desktop-portal-hyprland xdg-utils xf86-video-intel xorg-server xorg-xinit yay yay-debug zip zram-generator
}

script_cmdline() {
    local param
    for param in $(</proc/cmdline); do
        case "${param}" in
            script=*)
                echo "${param#*=}"
                return 0
                ;;
        esac
    done
}

automated_script() {
    local script rt
    script="$(script_cmdline)"
    if [[ -n "${script}" && ! -x /tmp/startup_script ]]; then
        if [[ "${script}" =~ ^((http|https|ftp|tftp)://) ]]; then
            # there's no synchronization for network availability before executing this script
            printf '%s: waiting for network-online.target\n' "$0"
            until systemctl --quiet is-active network-online.target; do
                sleep 1
            done
            printf '%s: downloading %s\n' "$0" "${script}"
            curl "${script}" --location --retry-connrefused --retry 10 -s -o /tmp/startup_script
            rt=$?
        else
            cp "${script}" /tmp/startup_script
            rt=$?
        fi
        if [[ ${rt} -eq 0 ]]; then
            chmod +x /tmp/startup_script
            printf '%s: executing automated script\n' "$0"
            # note that script is executed when other services (like pacman-init) may be still in progress, please
            # synchronize to "systemctl is-system-running --wait" when your script depends on other services
            /tmp/startup_script
        fi
    fi
}

if [[ $(tty) == "/dev/tty1" ]]; then
    automated_script
fi
