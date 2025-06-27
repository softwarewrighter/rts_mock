use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
    log_status(&format!("Map clicked at ({:.0}, {:.0})", x, y));
}

#[wasm_bindgen]
pub fn handle_minimap_click(x: f64, y: f64) {
    log_status(&format!("Minimap clicked at ({:.0}, {:.0})", x, y));
}

#[wasm_bindgen]
pub fn handle_build_button(building_type: &str) {
    log_status(&format!("Build {} selected", building_type));
}

#[wasm_bindgen]
pub fn handle_research_button(tech: &str) {
    log_status(&format!("Research {} selected", tech));
}

#[wasm_bindgen]
pub fn handle_unit_command(command: &str) {
    log_status(&format!("Unit command: {}", command));
}

#[wasm_bindgen]
pub fn handle_resource_click(resource: &str) {
    log_status(&format!("Resource panel clicked: {}", resource));
}