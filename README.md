# SigmaOS Hyprland Config

> Hyprland Wayland compositor configuration for SigmaOS Omarchy persona

## Overview

This repository contains the Hyprland Wayland compositor configuration for the SigmaOS Omarchy persona, including keybindings, window rules, monitor configuration, and theme integration with Wallust.

## Features

- Dynamic tiling window management
- Keyboard-driven navigation
- Wallust wallpaper-driven theming
- Waybar/Quickshell integration
- Custom keybindings optimized for productivity
- Window rules for consistent application behavior
- Multi-monitor support

## Installation

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS-hyprland-config.git

# Copy configuration to Hyprland config directory
mkdir -p ~/.config/hypr
cp -r SigmaOS-hyprland-config/* ~/.config/hypr/
```

## Configuration Structure

```
~/.config/hypr/
├── hyprland.conf          # Main Hyprland configuration
├── keybindings.conf       # Keybinding definitions
├── windowrules.conf       # Window rules
├── monitors.conf          # Monitor configuration
└── themes/                # Theme configurations
    └── wallust.conf      # Wallust integration
```

## Keybindings

### Window Management
- `Super + Return`: Terminal
- `Super + D`: Application launcher
- `Super + Shift + Q`: Close window
- `Super + Shift + Space`: Toggle floating
- `Super + F`: Toggle fullscreen
- `Super + J/K/L/H`: Move focus (vim-style)
- `Super + Shift + J/K/L/H`: Move window

### Workspace Management
- `Super + 1-9`: Switch to workspace
- `Super + Shift + 1-9`: Move to workspace
- `Super + Mouse Scroll`: Cycle workspaces

### Media Keys
- `XF86AudioRaiseVolume`: Volume up
- `XF86AudioLowerVolume`: Volume down
- `XF86AudioMute`: Toggle mute
- `XF86MonBrightnessUp`: Brightness up
- `XF86MonBrightnessDown`: Brightness down

## Theming

The configuration integrates with Wallust for wallpaper-driven theming:

```bash
# Set wallpaper and generate theme
wallust run ~/.config/wallpaper.png

# Reload Hyprland to apply theme
hyprctl reload
```

## Requirements

- Hyprland Wayland compositor
- Waybar or Quickshell
- Wallust (for theming)
- Swaylock (for lock screen)
- Wofi or rofi (for application launcher)

## Development

See [SigmaOS Development Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development) for general guidelines.

## License

MIT OR GPL-2.0 — Free and Open Source Software.
© SigmaOS Contributors
