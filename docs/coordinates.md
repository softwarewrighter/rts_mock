# Coordinate System Documentation

**Last Updated:** 2025-11-14

## Overview

The RTS mockup uses two different coordinate systems that need to work together:
1. **Isometric Main Map** - 3D-style projection for the game world
2. **2D Minimap** - Top-down orthogonal view for navigation

This document explains how these systems work and how coordinates are transformed between them.

## Main Map Coordinate System

### SVG ViewBox
```xml
<svg viewBox="0 0 1200 800" class="map-svg">
```

### Properties
- **Width**: 1200 units
- **Height**: 800 units
- **Origin**: Top-left corner (0, 0)
- **Projection**: Isometric (pseudo-3D)
- **Coordinate System**: Cartesian (x increases right, y increases down)

### Isometric Projection

The main map uses an isometric projection where:
- Objects appear tilted at ~26.5° (arctan(0.5))
- Horizontal lines are rotated 45° clockwise
- Vertical distances appear compressed by factor of 0.5

#### Visual Representation
```
        (0, 0)
          /\
         /  \
        /    \
       /      \
      /  Map   \
     /  Space   \
    /            \
   /______________\
(0, 800)      (1200, 800)
```

### Object Positioning

Objects on the isometric map are positioned using their base coordinates:

**Buildings** (e.g., Barracks):
```xml
<polygon points="200,300 250,280 300,300 250,320" fill="#8B4513"/>
```
- Base center: (250, 300)
- Width: 100 units
- Height: 40 units (compressed)

**Mountains**:
```xml
<polygon points="650,150 720,120 750,180 680,200" fill="#666"/>
```
- Peak: (700, 120)
- Base: ~(700, 190)

**Units** (soldiers):
```xml
<polygon points="240,600 245,595 250,600 245,605" fill="#00ff00"/>
```
- Center: (245, 600)
- Size: 10x10 diamond

### Click Handling

When a user clicks on the main map, JavaScript receives screen coordinates which SVG automatically converts to viewBox coordinates:

```javascript
svg.addEventListener('click', (e) => {
    const pt = svg.createSVGPoint();
    pt.x = e.clientX;
    pt.y = e.clientY;
    const svgP = pt.matrixTransform(svg.getScreenCTM().inverse());
    // svgP.x and svgP.y are in viewBox coordinates (0-1200, 0-800)
    handle_map_click(svgP.x, svgP.y);
});
```

## Minimap Coordinate System

### SVG ViewBox
```xml
<svg viewBox="0 0 300 200" class="minimap-svg">
```

### Properties
- **Width**: 300 units
- **Height**: 200 units
- **Origin**: Top-left corner (0, 0)
- **Projection**: Orthogonal (2D top-down)
- **Coordinate System**: Cartesian (x increases right, y increases down)
- **Scale Ratio**: 4:1 (main map is 4x larger)

### Visual Representation
```
(0,0) +-----------------+ (300,0)
      |                 |
      |  [Viewport Box] |
      |                 |
(0,200) +---------------+ (300,200)
```

### Viewport Indicator

The yellow rectangle on the minimap shows the current view area:

```xml
<rect x="100" y="75" width="150" height="100"
      stroke="yellow" stroke-width="2" fill="none"/>
```

This indicates:
- Viewport center in minimap: (175, 125)
- Viewport size: 150x100 units
- Corresponds to main map area: 600x400 units centered at (700, 500)

### Object Representation

Objects are simplified on the minimap:

**Buildings**: Small colored rectangles
```xml
<rect x="50" y="75" width="4" height="4" fill="#8B4513"/>
```
- Represents barracks at main map position (200, 300)
- Scale: 200 / 4 = 50, 300 / 4 = 75

**Water**: Blue ellipses
```xml
<ellipse cx="207" cy="100" rx="60" ry="40" fill="#0066cc"/>
```
- Represents large lake at main map (830, 400)
- Scale: 830 / 4 = 207.5 ≈ 207

**Units**: Small dots
```xml
<circle cx="60" cy="150" r="1" fill="#ffff00"/>
```

## Coordinate Transformations

### Minimap to Main Map (Click Handling)

When user clicks on minimap, we need to center the main map view at that location:

```rust
pub fn handle_minimap_click(minimap_x: f64, minimap_y: f64) {
    // Scale factor: main_map_size / minimap_size
    const SCALE_X: f64 = 1200.0 / 300.0; // = 4.0
    const SCALE_Y: f64 = 800.0 / 200.0;  // = 4.0

    // Convert to main map coordinates
    let map_x = minimap_x * SCALE_X;
    let map_y = minimap_y * SCALE_Y;

    log_status(&format!("Minimap clicked at ({:.0}, {:.0})", map_x, map_y));
}
```

**Example:**
- User clicks minimap at (75, 50)
- Converts to main map: (300, 200)
- This would center the viewport at that location (if scrolling implemented)

### Main Map to Minimap (Object Placement)

When placing objects on the minimap based on main map coordinates:

```rust
fn main_to_minimap(main_x: f64, main_y: f64) -> (f64, f64) {
    const SCALE_X: f64 = 300.0 / 1200.0; // = 0.25
    const SCALE_Y: f64 = 200.0 / 800.0;  // = 0.25

    let mini_x = main_x * SCALE_X;
    let mini_y = main_y * SCALE_Y;

    (mini_x, mini_y)
}
```

**Example:**
- Building at main map (400, 600)
- Minimap position: (100, 150)

### Isometric to Cartesian (Future Enhancement)

For true 3D-to-2D mapping, we need to convert isometric coordinates to cartesian:

```rust
// Isometric to Cartesian
fn iso_to_cart(iso_x: f64, iso_y: f64, tile_width: f64, tile_height: f64) -> (f64, f64) {
    let cart_x = (iso_x / tile_width + iso_y / tile_height) / 2.0;
    let cart_y = (iso_y / tile_height - iso_x / tile_width) / 2.0;
    (cart_x, cart_y)
}

// Cartesian to Isometric
fn cart_to_iso(cart_x: f64, cart_y: f64, tile_width: f64, tile_height: f64) -> (f64, f64) {
    let iso_x = (cart_x - cart_y) * tile_width;
    let iso_y = (cart_x + cart_y) * tile_height;
    (iso_x, iso_y)
}
```

**Note**: Currently not implemented. The map uses "fake" isometric (visual only) where objects are positioned directly in 2D space but rendered with isometric appearance.

## Scrolling and Viewport

### Current Implementation (v0.1.0)

- Fixed viewport showing entire map
- No scrolling implemented
- Minimap viewport indicator is static

### Planned Implementation

The viewport would track which portion of the main map is visible:

```rust
struct Viewport {
    center_x: f64,      // Main map coordinates
    center_y: f64,
    width: f64,         // Viewport size in main map units
    height: f64,
}

impl Viewport {
    fn minimap_rect(&self) -> (f64, f64, f64, f64) {
        // Calculate minimap rectangle position
        let mini_x = (self.center_x - self.width / 2.0) * 0.25;
        let mini_y = (self.center_y - self.height / 2.0) * 0.25;
        let mini_w = self.width * 0.25;
        let mini_h = self.height * 0.25;
        (mini_x, mini_y, mini_w, mini_h)
    }
}
```

## Edge Cases and Bounds

### Clamping to Map Bounds

When converting coordinates, ensure they stay within valid ranges:

```rust
fn clamp_to_map(x: f64, y: f64) -> (f64, f64) {
    let clamped_x = x.max(0.0).min(1200.0);
    let clamped_y = y.max(0.0).min(800.0);
    (clamped_x, clamped_y)
}

fn clamp_to_minimap(x: f64, y: f64) -> (f64, f64) {
    let clamped_x = x.max(0.0).min(300.0);
    let clamped_y = y.max(0.0).min(200.0);
    (clamped_x, clamped_y)
}
```

### Handling Float Precision

Use rounding for display coordinates:

```rust
fn format_coordinates(x: f64, y: f64) -> String {
    format!("({:.0}, {:.0})", x, y)  // Round to nearest integer
}
```

## Testing Coordinates

### Unit Tests

```rust
#[test]
fn test_minimap_to_main_scale() {
    let (main_x, main_y) = minimap_to_main(75.0, 50.0);
    assert_eq!(main_x, 300.0);
    assert_eq!(main_y, 200.0);
}

#[test]
fn test_main_to_minimap_scale() {
    let (mini_x, mini_y) = main_to_minimap(400.0, 600.0);
    assert_eq!(mini_x, 100.0);
    assert_eq!(mini_y, 150.0);
}

#[test]
fn test_coordinate_clamping() {
    let (x, y) = clamp_to_map(-10.0, 1500.0);
    assert_eq!(x, 0.0);
    assert_eq!(y, 800.0);
}
```

## Reference Tables

### Scale Factors

| Conversion | X Factor | Y Factor |
|-----------|----------|----------|
| Minimap → Main | 4.0 | 4.0 |
| Main → Minimap | 0.25 | 0.25 |

### Dimension Reference

| Component | Width | Height | Aspect Ratio |
|----------|-------|--------|--------------|
| Main Map | 1200 | 800 | 3:2 |
| Minimap | 300 | 200 | 3:2 |
| Viewport (future) | 600 | 400 | 3:2 |

### Object Sizes (Main Map)

| Object Type | Typical Size | Units |
|------------|--------------|-------|
| Building | 100 x 40 | pixels |
| Unit | 10 x 10 | pixels |
| Tree | 50 x 60 | pixels |
| Mountain | 100 x 80 | pixels |

### Object Sizes (Minimap)

| Object Type | Typical Size | Units |
|------------|--------------|-------|
| Building | 4 x 4 | pixels |
| Unit | 2 (radius) | pixels |
| Water | 60 x 40 (radii) | pixels |

---

*Understanding these coordinate systems is essential for implementing features like unit movement, building placement, and camera scrolling.*
