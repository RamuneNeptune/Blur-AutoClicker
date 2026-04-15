use super::mouse::{current_cursor_position, current_virtual_screen_rect, VirtualScreenRect};
use super::ClickerConfig;

pub fn detect_failsafe(
    cursor: (i32, i32),
    screen: VirtualScreenRect,
    config: &ClickerConfig,
) -> Option<String> {
    let left = screen.left;
    let top = screen.top;
    let right = screen.right();
    let bottom = screen.bottom();

    if config.corner_stop_enabled {
        if cursor.0 <= left + config.corner_stop_tl && cursor.1 <= top + config.corner_stop_tl {
            return Some(String::from("Top-left corner failsafe"));
        }
        if cursor.0 >= right - config.corner_stop_tr && cursor.1 <= top + config.corner_stop_tr {
            return Some(String::from("Top-right corner failsafe"));
        }
        if cursor.0 <= left + config.corner_stop_bl && cursor.1 >= bottom - config.corner_stop_bl
        {
            return Some(String::from("Bottom-left corner failsafe"));
        }
        if cursor.0 >= right - config.corner_stop_br
            && cursor.1 >= bottom - config.corner_stop_br
        {
            return Some(String::from("Bottom-right corner failsafe"));
        }
    }

    if config.edge_stop_enabled {
        if cursor.1 <= top + config.edge_stop_top {
            return Some(String::from("Top edge failsafe"));
        }
        if cursor.0 >= right - config.edge_stop_right {
            return Some(String::from("Right edge failsafe"));
        }
        if cursor.1 >= bottom - config.edge_stop_bottom {
            return Some(String::from("Bottom edge failsafe"));
        }
        if cursor.0 <= left + config.edge_stop_left {
            return Some(String::from("Left edge failsafe"));
        }
    }

    None
}

pub fn should_stop_for_failsafe(config: &ClickerConfig) -> Option<String> {
    let cursor = current_cursor_position()?;
    let screen = current_virtual_screen_rect()?;
    detect_failsafe(cursor, screen, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> ClickerConfig {
        ClickerConfig {
            interval: 0.04,
            variation: 0.0,
            limit: 0,
            duty: 45.0,
            time_limit: 0.0,
            button: 1,
            double_click_enabled: false,
            double_click_delay_ms: 40,
            position_enabled: false,
            pos_x: 0,
            pos_y: 0,
            offset: 0.0,
            offset_chance: 0.0,
            smoothing: 0,
            corner_stop_enabled: true,
            corner_stop_tl: 50,
            corner_stop_tr: 50,
            corner_stop_bl: 50,
            corner_stop_br: 50,
            edge_stop_enabled: true,
            edge_stop_top: 40,
            edge_stop_right: 40,
            edge_stop_bottom: 40,
            edge_stop_left: 40,
        }
    }

    #[test]
    fn detects_edges_against_virtual_screen_offsets() {
        let config = sample_config();
        let screen = VirtualScreenRect {
            left: -1920,
            top: 0,
            width: 3840,
            height: 1080,
        };

        let reason = detect_failsafe((-1915, 500), screen, &config);
        assert_eq!(reason.as_deref(), Some("Left edge failsafe"));

        let reason = detect_failsafe((1915, 500), screen, &config);
        assert_eq!(reason.as_deref(), Some("Right edge failsafe"));
    }

    #[test]
    fn detects_corners_against_virtual_screen_offsets() {
        let config = sample_config();
        let screen = VirtualScreenRect {
            left: -1280,
            top: -200,
            width: 2560,
            height: 1440,
        };

        let reason = detect_failsafe((-1275, -190), screen, &config);
        assert_eq!(reason.as_deref(), Some("Top-left corner failsafe"));

        let reason = detect_failsafe((1275, 1235), screen, &config);
        assert_eq!(reason.as_deref(), Some("Bottom-right corner failsafe"));
    }
}
