# RTS Mock - Project Overview

**Last Updated:** 2025-11-14

## Project Purpose

This is a web-based Real-Time Strategy (RTS) game UI mockup built with Rust and WebAssembly. The project serves as a technical demonstration and prototype showcasing:

- **RTS-style user interface** with classic elements (resource panel, build menu, research menu, unit controls)
- **Rust + WebAssembly integration** for high-performance web applications
- **Interactive SVG-based graphics** for scalable game-like interfaces
- **Retro terminal aesthetic** with a green-on-black color scheme

**Important:** This is purely a UI demonstration project - it contains no actual game logic, state management, or gameplay mechanics. All interactions produce status messages rather than functional game behavior.

## Technology Stack

- **Rust** (Edition 2021) - Core interaction logic
- **wasm-bindgen** - Rust/JavaScript interop layer
- **web-sys** - Browser API bindings
- **WebAssembly** - High-performance web execution
- **HTML5/CSS3** - UI layout and styling (inline in index.html)
- **SVG** - Vector graphics for map and minimap
- **Minimal JavaScript** - WASM initialization only

## Current Status

### Implemented Features

1. **UI Components**
   - Resource panel (Gold, Wood, Stone, Food, Population)
   - Interactive main map with SVG terrain, buildings, and units
   - Minimap with viewport indicator
   - Build menu (8 building types)
   - Research menu (7 technologies)
   - Unit control panel (Move, Attack, Patrol, Stop)

2. **Interactive Elements**
   - Map clicking with coordinate display
   - Minimap clicking with coordinate conversion
   - Building selection
   - Technology research selection
   - Unit command issuance
   - Resource counter clicking

3. **Recent Enhancements** (from git history)
   - Coordinate system alignment between isometric main map and 2D minimap
   - RTS-style map scrolling and minimap integration
   - Comprehensive test suite with unit and WASM tests
   - Clean project documentation

4. **Code Quality**
   - Unit tests for helper functions (format_status_message, format_coordinates)
   - WASM-specific browser tests
   - Test coverage for edge cases (empty strings, coordinate extremes)
   - Linting-ready (cargo clippy)
   - Formatting standards (cargo fmt)

### Project Structure

```
rts_mock/
├── src/
│   ├── lib.rs          # Main WASM module (interaction handlers)
│   └── main.rs         # Unused CLI entry point
├── pkg/                # Generated WASM output (git-ignored)
├── docs/               # Documentation
│   └── overview.md     # This file
├── index.html          # Complete UI with inline CSS/JS
├── Cargo.toml          # Rust dependencies
├── CLAUDE.md           # Development guidance
└── README.md           # User-facing documentation
```

### Build Status

- **Build System:** wasm-pack (web target)
- **Output:** pkg/ directory with WASM binary and JS bindings
- **Deployment:** Static file serving (any HTTP server)
- **Tests:** Passing (4 unit tests, 5 WASM tests)

## Possible Next Steps

### Enhancement Ideas

1. **Add Game State Management**
   - Implement actual resource tracking and modification
   - Track building construction and placement
   - Manage unit creation and movement
   - Handle technology research progress

2. **Expand Map Functionality**
   - Enable unit selection by clicking
   - Implement drag-to-select for multiple units
   - Add building placement mode with grid snapping
   - Implement fog of war visualization

3. **Improve Visual Design**
   - Add sprite sheets for more detailed graphics
   - Implement animations for unit movement
   - Add particle effects for actions
   - Enhance minimap with color-coded terrain

4. **Develop Game Logic**
   - Resource gathering mechanics
   - Building construction timers
   - Unit pathfinding algorithms
   - Combat system basics

5. **Technical Improvements**
   - Separate CSS into external stylesheet
   - Add state management library (e.g., store pattern)
   - Implement proper event handling architecture
   - Add TypeScript definitions for type safety

6. **Testing & Documentation**
   - Add integration tests
   - Implement visual regression testing
   - Create API documentation
   - Add interactive examples/demos

7. **Performance Optimization**
   - Benchmark WASM performance
   - Optimize SVG rendering
   - Implement canvas-based rendering option
   - Add lazy loading for assets

### Learning/Demonstration Uses

This project is well-suited for:
- Teaching Rust/WASM web development
- Demonstrating browser API integration
- Prototyping RTS game interfaces
- Experimenting with SVG-based game graphics
- Portfolio demonstration of Rust web capabilities

## Development Workflow

```bash
# Build WASM package
wasm-pack build --target web --out-dir pkg

# Run tests
cargo test

# Code quality
cargo clippy
cargo fmt

# Serve locally
python -m http.server 8000
# Then open http://localhost:8000/index.html
```

## Notes

- All styling is inline in index.html - no separate CSS files
- The pkg/ directory is auto-generated and git-ignored
- Event handlers are minimal JavaScript that forward to Rust functions
- Status messages appear both in console and on-screen display
- Project follows clean separation: Rust (logic) + HTML/CSS (presentation)

---

*This document serves as a high-level overview for project management purposes. For detailed technical information, see README.md and CLAUDE.md.*
