use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Helper function to format status messages
fn format_status_message(prefix: &str, details: &str) -> String {
    format!("{}: {}", prefix, details)
}

// Helper function to format coordinates
fn format_coordinates(x: f64, y: f64) -> String {
    format!("({:.0}, {:.0})", x, y)
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn log_status(message: &str) {
    console::log_1(&format!("RTS Status: {}", message).into());
}

#[wasm_bindgen]
pub fn handle_map_click(x: f64, y: f64) {
    let message = format_status_message("Map clicked at", &format_coordinates(x, y));
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_minimap_click(x: f64, y: f64) {
    let message = format_status_message("Minimap clicked at", &format_coordinates(x, y));
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_build_button(building_type: &str) {
    let message = format_status_message("Build", &format!("{} selected", building_type));
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_research_button(tech: &str) {
    let message = format_status_message("Research", &format!("{} selected", tech));
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_unit_command(command: &str) {
    let message = format_status_message("Unit command", command);
    log_status(&message);
}

#[wasm_bindgen]
pub fn handle_resource_click(resource: &str) {
    let message = format_status_message("Resource panel clicked", resource);
    log_status(&message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_status_message() {
        let result = format_status_message("Test", "message");
        assert_eq!(result, "Test: message");
    }

    #[test]
    fn test_format_coordinates() {
        let result = format_coordinates(123.45, 678.90);
        assert_eq!(result, "(123, 679)");
    }

    #[test]
    fn test_format_coordinates_zero() {
        let result = format_coordinates(0.0, 0.0);
        assert_eq!(result, "(0, 0)");
    }

    #[test]
    fn test_format_coordinates_negative() {
        let result = format_coordinates(-10.5, -20.8);
        assert_eq!(result, "(-10, -21)");
    }
}

#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_greet_wasm() {
        greet("WASM");
        // Note: This test will trigger an alert in the browser
        // In a real app, you'd mock the alert function
    }

    #[wasm_bindgen_test]
    fn test_handle_map_click_wasm() {
        handle_map_click(100.0, 200.0);
        // This test verifies the function doesn't panic
        // In a real app, you'd capture and verify the console output
    }

    #[wasm_bindgen_test]
    fn test_handle_build_button_wasm() {
        handle_build_button("barracks");
        // This test verifies the function doesn't panic with valid input
    }

    #[wasm_bindgen_test]
    fn test_handle_empty_string_wasm() {
        handle_build_button("");
        handle_research_button("");
        handle_unit_command("");
        handle_resource_click("");
        // Test edge case with empty strings
    }

    #[wasm_bindgen_test]
    fn test_coordinate_edge_cases_wasm() {
        handle_map_click(0.0, 0.0);
        handle_map_click(-1.0, -1.0);
        handle_map_click(f64::MAX, f64::MIN);
        // Test edge cases for coordinate handling
    }
}