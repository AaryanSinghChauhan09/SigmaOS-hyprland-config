// Integration tests for SigmaOS-hyprland-config
use sigmaos_hyprland_config::HyprlandConfig;
use std::fs;

#[test]
fn test_parse_real_keybindings_conf() {
    let content = fs::read_to_string("keybindings.conf").expect("keybindings.conf should exist");
    let config = HyprlandConfig::parse(&content);
    assert!(!config.binds.is_empty(), "Should parse keybindings from keybindings.conf");
}

#[test]
fn test_parse_real_monitors_conf() {
    let content = fs::read_to_string("monitors.conf").expect("monitors.conf should exist");
    let config = HyprlandConfig::parse(&content);
    assert!(!config.monitors.is_empty(), "Should parse monitor configurations from monitors.conf");
}

#[test]
fn test_parse_real_windowrules_conf() {
    let content = fs::read_to_string("windowrules.conf").expect("windowrules.conf should exist");
    let config = HyprlandConfig::parse(&content);
    assert!(!config.window_rules.is_empty(), "Should parse window rules from windowrules.conf");
}

#[test]
fn test_parse_real_hyprland_conf() {
    let content = fs::read_to_string("hyprland.conf").expect("hyprland.conf should exist");
    let config = HyprlandConfig::parse(&content);
    assert!(!config.monitors.is_empty(), "Should parse monitor rule from hyprland.conf");
    assert!(!config.sources.is_empty(), "Should parse source declarations from hyprland.conf");
    assert!(!config.exec_once.is_empty(), "Should parse exec-once statements from hyprland.conf");
    assert!(!config.env_vars.is_empty(), "Should parse env vars from hyprland.conf");
}
