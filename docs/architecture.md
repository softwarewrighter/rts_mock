# Architecture Documentation

**Last Updated:** 2025-11-14

## Overview

This RTS mockup follows a clean separation of concerns with Rust/WebAssembly handling all logic and minimal JavaScript for initialization only.

## System Architecture

```
+------------------+
|   Web Browser    |
+------------------+
        |
        | HTTP
        v
+------------------+
|  index.html      |
|  (UI Structure)  |
+------------------+
        |
        | imports
        v
+------------------+
|   styles.css     |
| (Presentation)   |
+------------------+
        |
        | loads WASM
        v
+------------------+
|   WASM Module    |
|   (pkg/*.wasm)   |
+------------------+
        |
        | exposes functions
        v
+------------------+
|   Rust Code      |
|   (src/lib.rs)   |
+------------------+
        |
        | uses
        v
+------------------+
|   web-sys API    |
| (Browser Bindings)|
+------------------+
```

## Component Breakdown

### HTML Layer (index.html)
- **Responsibility**: DOM structure and layout
- **Content**:
  - Resource panel structure
  - SVG map and minimap containers
  - Build/Research/Unit control button groups
  - Status display area
  - Footer with copyright and links
- **Interaction**: Minimal - only event forwarding to WASM

### CSS Layer (styles.css)
- **Responsibility**: Visual presentation only
- **Content**:
  - Grid layout system
  - Color scheme (retro terminal green-on-black)
  - Button and hover states
  - SVG styling
  - Responsive design rules
- **No Logic**: Pure presentation, zero behavior

### JavaScript Layer (Minimal)
- **Responsibility**: WASM initialization only
- **Content**:
  - Load WASM module from pkg/
  - Initialize WASM instance
  - Wire DOM events to WASM functions
- **Size**: ~30 lines total
- **Philosophy**: Thin glue layer, no business logic

### Rust/WASM Layer (src/lib.rs)
- **Responsibility**: ALL application logic
- **Functions Exposed via #[wasm_bindgen]**:
  - `handle_map_click(x, y)` - Process map interactions
  - `handle_minimap_click(x, y)` - Process minimap interactions
  - `handle_build_button(building_type)` - Process building selections
  - `handle_research_button(tech)` - Process research selections
  - `handle_unit_command(command)` - Process unit commands
  - `handle_resource_click(resource)` - Process resource interactions
  - `log_status(message)` - Console logging wrapper

### Web-sys Bindings
- **Responsibility**: Rust interface to browser APIs
- **Used For**:
  - Console logging (console::log_1)
  - DOM manipulation (Document, Element, HtmlElement)
  - Event handling (MouseEvent)
  - Future: Direct DOM updates from Rust

## Data Flow

### User Interaction Flow

```
User Click
    |
    v
Browser Event
    |
    v
JavaScript Event Handler (minimal)
    |
    v
WASM Function Call
    |
    v
Rust Logic Processing
    |
    v
web-sys API Calls
    |
    v
Browser Console / DOM Update
```

### Example: Building Selection

```
1. User clicks "Barracks" button
2. HTML onclick fires: onclick="handleBuild('barracks')"
3. JS function calls: handle_build_button('barracks')
4. Rust receives call in lib.rs
5. Rust formats message: "Build: barracks selected"
6. Rust calls web-sys console::log_1
7. Browser console displays message
8. Status display div updates (via DOM manipulation)
```

## Coordinate System

### Isometric Main Map (SVG)
- **ViewBox**: 1200 x 800 units
- **Coordinate Origin**: Top-left (0, 0)
- **Orientation**: Isometric projection (diamond shape)
- **Features**:
  - Buildings rendered at isometric angles
  - Mountains shown as elevated polygons
  - Water bodies as ellipses
  - Units as small diamonds

### 2D Minimap
- **Size**: 300 x 200 pixels
- **Coordinate Origin**: Top-left (0, 0)
- **Orientation**: Top-down 2D view
- **Scale**: 1:4 ratio to main map
- **Viewport Indicator**: Yellow rectangle showing current view

### Coordinate Transformation
When clicking on minimap, coordinates are scaled to main map:
```rust
map_x = minimap_x * (MAP_WIDTH / MINIMAP_WIDTH)
map_y = minimap_y * (MAP_HEIGHT / MINIMAP_HEIGHT)
```

Future enhancement: Add isometric ↔ cartesian transformation for true coordinate mapping.

## State Management

### Current Approach (v0.1.0)
- **Stateless**: Each interaction is independent
- **No Persistence**: No game state is maintained
- **Demo Mode**: All interactions produce log messages only

### Future Enhancement
Consider adding:
- State struct in Rust to track resources, buildings, units
- RefCell/Rc for mutable state
- Event-driven state updates
- Serialization for save/load

## Build Process

```
Source Code (src/lib.rs)
    |
    | cargo build --target wasm32-unknown-unknown
    v
WASM Binary (.wasm)
    |
    | wasm-bindgen (via wasm-pack)
    v
JavaScript Bindings (pkg/*.js)
    +
WASM Module (pkg/*.wasm)
    |
    | index.html imports
    v
Running Application in Browser
```

## Testing Strategy

### Unit Tests
- **Location**: `#[cfg(test)] mod tests` in lib.rs
- **Target**: Host architecture (x86_64, ARM, etc.)
- **Run With**: `cargo test`
- **Purpose**: Test pure Rust helper functions

### WASM Tests
- **Location**: `#[cfg(all(test, target_arch = "wasm32"))] mod wasm_tests`
- **Target**: wasm32 architecture only
- **Run With**: `wasm-pack test --headless --firefox`
- **Purpose**: Test browser integration and WASM functions

### Integration Tests
- **Future**: End-to-end tests with Playwright
- **Scope**: Full UI interaction testing
- **Coverage**: Click sequences, visual regression

## Performance Considerations

### WASM Advantages
- Near-native performance for game logic
- Zero garbage collection pauses
- Small binary size (~50KB compressed)
- Fast load times

### Current Bottlenecks
- SVG rendering (browser-side)
- DOM manipulation overhead
- No canvas-based rendering yet

### Future Optimizations
- Move to Canvas rendering for main map
- Implement dirty rectangle updates
- Use requestAnimationFrame for smooth updates
- Consider WebGL for advanced graphics

## Security

### Current Model
- No server communication
- No local storage access
- No sensitive data handling
- Pure client-side application

### WASM Sandbox
- Runs in browser security sandbox
- No direct file system access
- No network access (unless explicitly added)
- Memory safety guaranteed by Rust

## Extensibility

### Adding New Features

1. **New UI Element**:
   - Add HTML structure
   - Add CSS styling
   - Create Rust function with #[wasm_bindgen]
   - Wire event in JavaScript init code

2. **New Game Logic**:
   - Implement in Rust (src/lib.rs)
   - Expose via #[wasm_bindgen] if needed
   - No JavaScript changes required

3. **New Visual Effect**:
   - Update SVG or add canvas
   - Add CSS animations
   - Trigger from Rust via web-sys DOM API

## Dependencies

### Rust Crates
- `wasm-bindgen` (0.2): Rust/JS interop
- `web-sys` (0.3): Browser API bindings
- `wasm-bindgen-test` (0.3): WASM testing (dev)

### Build Tools
- `wasm-pack`: WASM build and packaging
- `cargo`: Rust build system
- `rustc`: Rust compiler

### Runtime Requirements
- Modern browser with WASM support (2017+)
- JavaScript enabled
- ~1MB memory for WASM instance

---

*This architecture prioritizes simplicity, maintainability, and Rust-first development.*
