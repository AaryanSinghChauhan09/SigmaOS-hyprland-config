// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hyprland Wayland Compositor Configuration Engine for SigmaOS Omarchy Persona.
//!
//! Provides parsing, validation, and theme generation for:
//! - Keybindings (modifiers, dispatchers, commands)
//! - Window rules (floating, opacity, workspace pinning)
//! - Monitor layouts (resolutions, positions, scaling)
//! - Sources, autostart exec-once, and environment variables
//! - Wallust & Catppuccin Mocha color variable interpolation

#![forbid(unsafe_code)]

use std::collections::HashMap;

/// Modifier keys for Hyprland bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub super_key: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl Modifiers {
    pub fn parse(s: &str) -> Self {
        let mut mods = Self::default();
        for part in s.split('_').flat_map(|p| p.split('+')) {
            let p = part.trim().to_uppercase();
            match p.as_str() {
                "SUPER" | "$MAINMOD" => mods.super_key = true,
                "CTRL" | "CONTROL" => mods.ctrl = true,
                "SHIFT" => mods.shift = true,
                "ALT" | "MOD1" => mods.alt = true,
                _ => {}
            }
        }
        mods
    }

    pub fn is_empty(&self) -> bool {
        !self.super_key && !self.ctrl && !self.shift && !self.alt
    }
}

/// Action dispatcher types supported by Hyprland.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dispatcher {
    Exec,
    KillActive,
    Workspace,
    MoveToWorkspace,
    ToggleFloating,
    Fullscreen,
    Custom(String),
}

impl Dispatcher {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "exec" => Self::Exec,
            "killactive" => Self::KillActive,
            "workspace" => Self::Workspace,
            "movetoworkspace" => Self::MoveToWorkspace,
            "togglefloating" => Self::ToggleFloating,
            "fullscreen" => Self::Fullscreen,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// A parsed keybinding rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    pub modifiers: Modifiers,
    pub key: String,
    pub dispatcher: Dispatcher,
    pub arg: String,
}

/// A parsed window rule for tiling/floating and styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRule {
    pub rule_type: String,
    pub target_regex: String,
}

/// Monitor arrangement configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorConfig {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub scale: f32,
}

impl MonitorConfig {
    pub fn parse_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.is_empty() {
            return None;
        }
        let name = parts[0].to_string();
        let res_part = parts.get(1).copied().unwrap_or("preferred");
        let pos_part = parts.get(2).copied().unwrap_or("auto");
        let scale_part = parts.get(3).copied().unwrap_or("1");

        let (width, height, refresh_rate) = if res_part.contains('x') {
            let mut dims = res_part.split('@');
            let wh = dims.next().unwrap_or("1920x1080");
            let hz = dims.next().and_then(|h| h.parse::<u32>().ok()).unwrap_or(60);
            let mut wh_parts = wh.split('x');
            let w = wh_parts.next().and_then(|w| w.parse::<u32>().ok()).unwrap_or(1920);
            let h = wh_parts.next().and_then(|h| h.parse::<u32>().ok()).unwrap_or(1080);
            (w, h, hz)
        } else {
            (1920, 1080, 60)
        };

        let (pos_x, pos_y) = if pos_part.contains('x') {
            let mut p = pos_part.split('x');
            let x = p.next().and_then(|x| x.parse::<i32>().ok()).unwrap_or(0);
            let y = p.next().and_then(|y| y.parse::<i32>().ok()).unwrap_or(0);
            (x, y)
        } else {
            (0, 0)
        };

        let scale = scale_part.parse::<f32>().unwrap_or(1.0);

        Some(Self {
            name,
            width,
            height,
            refresh_rate,
            pos_x,
            pos_y,
            scale,
        })
    }
}

/// Full parsed Hyprland configuration structure.
#[derive(Debug, Clone, Default)]
pub struct HyprlandConfig {
    pub variables: HashMap<String, String>,
    pub sources: Vec<String>,
    pub exec_once: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub binds: Vec<KeyBinding>,
    pub window_rules: Vec<WindowRule>,
    pub monitors: Vec<MonitorConfig>,
}

impl HyprlandConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(content: &str) -> Self {
        let mut config = Self::new();
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Variable assignment: $var = value
            if line.starts_with('$') {
                if let Some((k, v)) = line.split_once('=') {
                    config.variables.insert(k.trim().to_string(), v.trim().to_string());
                }
                continue;
            }

            // Source imports
            if let Some(rest) = line.strip_prefix("source = ").or_else(|| line.strip_prefix("source=")) {
                config.sources.push(rest.trim().to_string());
                continue;
            }

            // Exec-once autostart
            if let Some(rest) = line.strip_prefix("exec-once = ").or_else(|| line.strip_prefix("exec-once=")) {
                config.exec_once.push(rest.trim().to_string());
                continue;
            }

            // Environment variables: env = KEY, VAL
            if let Some(rest) = line.strip_prefix("env = ").or_else(|| line.strip_prefix("env=")) {
                if let Some((k, v)) = rest.split_once(',') {
                    config.env_vars.insert(k.trim().to_string(), v.trim().to_string());
                }
                continue;
            }

            // Monitor rule: monitor = name, res, pos, scale
            if let Some(rest) = line.strip_prefix("monitor = ").or_else(|| line.strip_prefix("monitor=")) {
                if let Some(m) = MonitorConfig::parse_line(rest) {
                    config.monitors.push(m);
                }
                continue;
            }

            // Bind rule: bind = MODS, KEY, DISPATCHER, ARG
            if let Some(rest) = line.strip_prefix("bind = ").or_else(|| line.strip_prefix("bind=")) {
                let parts: Vec<&str> = rest.splitn(4, ',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let mods = Modifiers::parse(parts[0]);
                    let key = parts[1].to_string();
                    let dispatcher = Dispatcher::parse(parts[2]);
                    let arg = parts.get(3).copied().unwrap_or("").to_string();
                    config.binds.push(KeyBinding {
                        modifiers: mods,
                        key,
                        dispatcher,
                        arg,
                    });
                }
                continue;
            }

            // Window rules: windowrule = RULE, REGEX or windowrulev2 = RULE, REGEX
            if let Some(rest) = line.strip_prefix("windowrule = ")
                .or_else(|| line.strip_prefix("windowrule="))
                .or_else(|| line.strip_prefix("windowrulev2 = "))
                .or_else(|| line.strip_prefix("windowrulev2="))
            {
                if let Some((rule, regex)) = rest.split_once(',') {
                    config.window_rules.push(WindowRule {
                        rule_type: rule.trim().to_string(),
                        target_regex: regex.trim().to_string(),
                    });
                }
                continue;
            }
        }
        config
    }

    /// Render Catppuccin Mocha / Wallust color variables in Hyprland config format.
    pub fn render_catppuccin_colors() -> String {
        r#"# Catppuccin Mocha Color Palette for Omarchy Hyprland
$rosewater = rgb(f5e0dc)
$flamingo  = rgb(f2cdcd)
$pink      = rgb(f5c2e7)
$mauve     = rgb(cba6f7)
$red       = rgb(f38ba8)
$maroon    = rgb(eba0ac)
$peach     = rgb(fab387)
$yellow    = rgb(f9e2af)
$green     = rgb(a6e3a1)
$teal      = rgb(94e2d5)
$sky       = rgb(89dceb)
$sapphire  = rgb(74c7ec)
$blue      = rgb(89b4fa)
$lavender  = rgb(b4befe)
$text      = rgb(cdd6f4)
$subtext1  = rgb(bac2de)
$subtext0  = rgb(a6adc8)
$overlay2  = rgb(9399b2)
$overlay1  = rgb(7f849c)
$overlay0  = rgb(6c7086)
$surface2  = rgb(585b70)
$surface1  = rgb(45475a)
$surface0  = rgb(313244)
$base      = rgb(1e1e2e)
$mantle    = rgb(181825)
$crust     = rgb(11111b)
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_parsing() {
        let m = Modifiers::parse("SUPER_SHIFT");
        assert!(m.super_key);
        assert!(m.shift);
        assert!(!m.ctrl);
        assert!(!m.alt);
    }

    #[test]
    fn test_dispatcher_parsing() {
        assert_eq!(Dispatcher::parse("exec"), Dispatcher::Exec);
        assert_eq!(Dispatcher::parse("killactive"), Dispatcher::KillActive);
        assert_eq!(Dispatcher::parse("workspace"), Dispatcher::Workspace);
        assert_eq!(Dispatcher::parse("movetoworkspace"), Dispatcher::MoveToWorkspace);
    }

    #[test]
    fn test_monitor_parsing() {
        let m = MonitorConfig::parse_line("eDP-1, 1920x1080@144, 0x0, 1.25").unwrap();
        assert_eq!(m.name, "eDP-1");
        assert_eq!(m.width, 1920);
        assert_eq!(m.height, 1080);
        assert_eq!(m.refresh_rate, 144);
        assert_eq!(m.scale, 1.25);
    }

    #[test]
    fn test_config_parsing() {
        let conf = r#"
        # Sample Omarchy Config
        $mainMod = SUPER
        source = ~/.config/hypr/monitors.conf
        exec-once = waybar
        env = XCURSOR_SIZE, 24
        monitor=DP-1,2560x1440@165,0x0,1.0
        bind = $mainMod, Return, exec, alacritty
        bind = $mainMod, Q, killactive,
        windowrule = float, ^(pavucontrol)$
        "#;
        let parsed = HyprlandConfig::parse(conf);
        assert_eq!(parsed.variables.get("$mainMod").unwrap(), "SUPER");
        assert_eq!(parsed.sources.len(), 1);
        assert_eq!(parsed.exec_once.len(), 1);
        assert_eq!(parsed.env_vars.get("XCURSOR_SIZE").unwrap(), "24");
        assert_eq!(parsed.monitors.len(), 1);
        assert_eq!(parsed.binds.len(), 2);
        assert_eq!(parsed.binds[0].key, "Return");
        assert_eq!(parsed.binds[0].dispatcher, Dispatcher::Exec);
        assert_eq!(parsed.window_rules.len(), 1);
    }

    #[test]
    fn test_catppuccin_colors_generation() {
        let colors = HyprlandConfig::render_catppuccin_colors();
        assert!(colors.contains("$mauve     = rgb(cba6f7)"));
        assert!(colors.contains("$base      = rgb(1e1e2e)"));
    }
}
