# Data Flow

This page documents how data flows through the RTS Mock system, from user input to visual output.

## 📊 Overall Data Flow

```mermaid
graph TB
    subgraph "Input Sources"
        MOUSE[Mouse Events]
        KEYBOARD[Keyboard Events]
        TOUCH[Touch Events Future]
    end

    subgraph "Event Processing"
        DOM_EVT[DOM Event Handlers]
        JS_PROC[JavaScript Processing]
        COORD_TRANS[Coordinate Transformation]
    end

    subgraph "Business Logic"
        WASM_HANDLERS[WASM Event Handlers]
        FORMAT[Formatting Functions]
    end

    subgraph "Output"
        CONSOLE_API[Console API]
        DOM_UPDATE[DOM Updates]
        SVG_TRANS[SVG Transforms]
    end

    subgraph "User Feedback"
        STATUS_MSG[Status Messages]
        VISUAL_FB[Visual Feedback]
        BROWSER_LOG[Browser Console]
    end

    MOUSE --> DOM_EVT
    KEYBOARD --> DOM_EVT
    TOUCH --> DOM_EVT

    DOM_EVT --> JS_PROC
    JS_PROC --> COORD_TRANS
    COORD_TRANS --> WASM_HANDLERS

    WASM_HANDLERS --> FORMAT
    FORMAT --> CONSOLE_API

    CONSOLE_API --> DOM_UPDATE
    CONSOLE_API --> BROWSER_LOG
    JS_PROC --> SVG_TRANS

    DOM_UPDATE --> STATUS_MSG
    SVG_TRANS --> VISUAL_FB
    STATUS_MSG --> VISUAL_FB

    style WASM_HANDLERS fill:#90EE90
    style CONSOLE_API fill:#ff9900
    style VISUAL_FB fill:#4169E1
```

---

## 1️⃣ Button Click Data Flow

Simple, synchronous data pipeline for button interactions.

```mermaid
flowchart LR
    A[User Clicks<br/>'Barracks' Button] --> B[DOM Event<br/>onclick]
    B --> C{Event Type}
    C -->|Build| D[handleBuildButton<br/>'barracks']
    C -->|Research| E[handleResearchButton<br/>'armor']
    C -->|Unit Cmd| F[handleUnitCommand<br/>'attack']
    C -->|Resource| G[handleResourceClick<br/>'gold']

    D --> H[WASM Handler]
    E --> H
    F --> H
    G --> H

    H --> I[format_status_message<br/>prefix + details]
    I --> J[log_status<br/>message]
    J --> K[Console Log<br/>'RTS Status: ...']
    K --> L[Status Display<br/>Update]
    L --> M[User Sees<br/>Feedback]

    style A fill:#4169E1
    style H fill:#90EE90
    style M fill:#FFD700
```

### Data Transformation

1. **Input:** Button ID string (e.g., `"barracks"`)
2. **Processing:** Concatenate with prefix (e.g., `"Build: barracks selected"`)
3. **Output:** Formatted status message in UI

**Example:**
```
Input:  "barracks"
        ↓
Format: "Build: " + "barracks" + " selected"
        ↓
Log:    "RTS Status: Build: barracks selected"
        ↓
Display: "Build: barracks selected"
```

---

## 2️⃣ Map Click Data Flow

Complex coordinate transformation pipeline.

```mermaid
flowchart TD
    A[User Clicks Map<br/>Screen 400, 300] --> B[DOM Event]
    B --> C[Get Bounding Rect]
    C --> D[Screen Coordinates<br/>clientX, clientY]

    D --> E[Convert to SVG<br/>Coordinates]
    E --> F[SVG Viewbox<br/>0-800, 0-600]

    F --> G[Adjust for<br/>Isometric Offset<br/>-400, -100]
    G --> H[Apply Inverse<br/>Transform Matrix<br/>det = 0.866*0.5...]

    H --> I[Add Viewport<br/>Position<br/>+ viewportX/Y]
    I --> J[World Coordinates<br/>1234.5, 978.2]

    J --> K[WASM Handler<br/>handle_map_click<br/>x: f64, y: f64]
    K --> L[format_coordinates<br/>Round to integers]
    L --> M[Format String<br/>'1235, 978']

    M --> N[format_status_message<br/>'Map clicked at: ...']
    N --> O[log_status]
    O --> P[Console + Display]
    P --> Q[User Sees<br/>World Coordinates]

    style A fill:#4169E1
    style K fill:#90EE90
    style Q fill:#FFD700
```

### Coordinate Spaces

| Space | Range | Purpose |
|-------|-------|---------|
| **Screen** | Pixel coordinates | Mouse position |
| **SVG** | 0-800 × 0-600 | SVG viewBox |
| **Isometric** | Adjusted for transform | 3D projection |
| **World** | 0-2400 × 0-1800 | Actual map position |

### Transformation Formula

```javascript
// Screen → SVG
svgX = (screenX / rect.width) * 800
svgY = (screenY / rect.height) * 600

// SVG → Adjusted
adjustedX = svgX - 400
adjustedY = svgY - 100

// Inverse Isometric Transform
det = 0.866 * 0.5 - 0.5 * (-0.866)
worldX = ((0.5 * adjustedX) - (-0.866 * adjustedY)) / det + viewportX
worldY = ((-0.5 * adjustedX) + (0.866 * adjustedY)) / det + viewportY
```

---

## 3️⃣ Map Drag State Flow

State management for viewport panning.

```mermaid
stateDiagram-v2
    [*] --> NotDragging
    NotDragging --> Dragging: mousedown
    Dragging --> Dragging: mousemove<br/>(update viewport)
    Dragging --> NotDragging: mouseup
    NotDragging --> NotDragging: mousemove<br/>(ignored)

    state NotDragging {
        [*] --> Idle
        Idle --> EdgeScroll: Mouse near edge
        EdgeScroll --> Idle: Mouse away
    }

    state Dragging {
        [*] --> Tracking
        Tracking --> Updating: Calculate delta
        Updating --> Clamping: Update viewport
        Clamping --> Rendering: Clamp bounds
        Rendering --> Tracking: Apply transform
    }
```

### State Variables

```javascript
// Dragging state
let isDragging = false;
let dragStart = { x: 0, y: 0 };        // Initial mouse position
let viewportStart = { x: 0, y: 0 };    // Initial viewport position

// Map state
const mapState = {
    mapWidth: 2400,
    mapHeight: 1800,
    viewportWidth: 800,
    viewportHeight: 600,
    viewportX: 800,      // Current X position
    viewportY: 600       // Current Y position
};
```

### Data Updates Per Frame

```mermaid
graph LR
    A[mousemove Event] --> B[Calculate dx, dy]
    B --> C[viewportX = start + dx<br/>viewportY = start + dy]
    C --> D[Clamp to Bounds<br/>0 to mapWidth-viewportWidth]
    D --> E[Update SVG Transform<br/>translate-viewportX, -viewportY]
    E --> F[Update Minimap Rect<br/>Scale by 200/2400]
    F --> G[Render Frame<br/>60fps]

    style A fill:#4169E1
    style G fill:#FFD700
```

---

## 4️⃣ Minimap Click Flow

Bidirectional coordinate scaling between minimap and main map.

```mermaid
flowchart TD
    A[User Clicks<br/>Minimap 100, 75] --> B[Get SVG Point]
    B --> C[Screen → SVG<br/>Coordinate Transform]
    C --> D[Minimap Coords<br/>100, 75]

    D --> E[Scale to World<br/>scaleX = 2400/200 = 12<br/>scaleY = 1800/150 = 12]
    E --> F[World Position<br/>1200, 900]

    F --> G[Center Viewport<br/>viewportX = 1200 - 400<br/>viewportY = 900 - 300]
    G --> H[Clamp Bounds<br/>0-1600, 0-1200]

    H --> I[Update Main Map<br/>SVG Transform]
    I --> J[Update Minimap<br/>Viewport Rect]

    J --> K[WASM Log<br/>handle_minimap_click100, 75]
    K --> L[Status Display<br/>Update]

    I --> M[User Sees<br/>Map Jump to Position]
    L --> M

    style A fill:#4169E1
    style K fill:#90EE90
    style M fill:#FFD700
```

### Scaling Factors

**Minimap → World:**
```javascript
scaleX = mapWidth / minimapWidth     // 2400 / 200 = 12
scaleY = mapHeight / minimapHeight   // 1800 / 150 = 12

worldX = minimapX * scaleX
worldY = minimapY * scaleY
```

**World → Minimap:**
```javascript
minimapX = worldX / scaleX
minimapY = worldY / scaleY
```

---

## 5️⃣ Keyboard Navigation Flow

Discrete viewport updates from keyboard input.

```mermaid
flowchart LR
    A[User Presses<br/>Arrow/WASD] --> B[keydown Event]
    B --> C{Which Key?}

    C -->|Up/W| D[viewportY -= 40]
    C -->|Down/S| E[viewportY += 40]
    C -->|Left/A| F[viewportX -= 40]
    C -->|Right/D| G[viewportX += 40]

    D --> H[Clamp Bounds]
    E --> H
    F --> H
    G --> H

    H --> I[Update SVG<br/>Transform]
    I --> J[Update Minimap<br/>Viewport Rect]

    J --> K[Log Status<br/>Viewport moved to...]
    K --> L[User Sees<br/>Map Scroll]

    style A fill:#4169E1
    style L fill:#FFD700
```

### Movement Delta

| Key | Direction | Delta | New Position |
|-----|-----------|-------|--------------|
| ↑/W | Up | y -= 40 | viewportY - 40 |
| ↓/S | Down | y += 40 | viewportY + 40 |
| ←/A | Left | x -= 40 | viewportX - 40 |
| →/D | Right | x += 40 | viewportX + 40 |

---

## 6️⃣ Edge Scroll Continuous Flow

Interval-based continuous updates.

```mermaid
flowchart TD
    A[Mouse Moves<br/>Over Map] --> B[Update<br/>currentMousePos]
    B --> C{Mouse Near<br/>Edge < 50px?}

    C -->|Yes| D{scrollInterval<br/>Running?}
    C -->|No| E[Clear Interval<br/>if exists]

    D -->|No| F[Start Interval<br/>setInterval 16ms]
    D -->|Yes| G[Continue Existing]

    F --> H[handleEdgeScroll]
    G --> H

    H --> I[Calculate dx, dy<br/>±20px based on edge]
    I --> J[viewportX += dx<br/>viewportY += dy]
    J --> K[Clamp Bounds]
    K --> L[Update Map Transform]
    L --> M[Update Minimap]

    M --> N{Still Near Edge?}
    N -->|Yes| H
    N -->|No| E

    E --> O[Scrolling Stopped]

    style A fill:#4169E1
    style H fill:#90EE90
    style O fill:#FFD700
```

### Scroll Parameters

```javascript
const edgeThreshold = 50;      // Pixels from edge
const scrollSpeed = 20;        // Pixels per frame
const intervalMs = 16;         // ~60fps

// Calculate scroll direction
let dx = 0, dy = 0;
if (mouseX < edgeThreshold) dx = -scrollSpeed;
if (mouseX > width - edgeThreshold) dx = scrollSpeed;
if (mouseY < edgeThreshold) dy = -scrollSpeed;
if (mouseY > height - edgeThreshold) dy = scrollSpeed;
```

---

## 7️⃣ Status Message Flow

Console override intercepts and routes messages.

```mermaid
flowchart TD
    A[WASM Function<br/>Calls log_status] --> B[web_sys::console::log_1]
    B --> C[Browser Console API]
    C --> D[Overridden<br/>console.log Function]

    D --> E[Call Original<br/>console.log<br/>Developer Tools Output]
    D --> F{Message Contains<br/>'RTS Status:'?}

    F -->|Yes| G[Extract Message<br/>Remove 'RTS Status: ']
    F -->|No| H[Ignore]

    G --> I[Get Status Element<br/>getElementById'status']
    I --> J[Update textContent]
    J --> K[DOM Renders<br/>New Status]

    E --> L[Developer Sees<br/>Console Log]
    K --> M[User Sees<br/>Status Display]

    style A fill:#90EE90
    style D fill:#ff9900
    style M fill:#4169E1
```

### Message Formatting Pipeline

```rust
// In WASM (Rust)
fn handle_build_button(building_type: &str) {
    // Step 1: Format details
    let details = format!("{} selected", building_type);
    // "barracks selected"

    // Step 2: Add prefix
    let message = format_status_message("Build", &details);
    // "Build: barracks selected"

    // Step 3: Log with "RTS Status:" prefix
    log_status(&message);
    // console.log("RTS Status: Build: barracks selected")
}
```

```javascript
// In JavaScript
console.log = function(message) {
    originalLog(message);  // "RTS Status: Build: barracks selected"

    if (message.includes('RTS Status:')) {
        const cleanMessage = message.replace('RTS Status: ', '');
        // "Build: barracks selected"

        statusEl.textContent = cleanMessage;
        // Updates DOM
    }
};
```

---

## 🔄 Data Type Transformations

### Type Conversions Across Layers

```mermaid
graph LR
    subgraph "JavaScript Types"
        JS_NUM[number]
        JS_STR[string]
        JS_OBJ[object]
    end

    subgraph "WASM Interface"
        WASM_F64[f64]
        WASM_STR[&str]
        WASM_OWNED[String]
    end

    subgraph "Rust Internal"
        RUST_F64[f64]
        RUST_STR[&str]
        RUST_STRING[String]
    end

    JS_NUM -->|wasm-bindgen| WASM_F64
    JS_STR -->|wasm-bindgen| WASM_STR

    WASM_F64 --> RUST_F64
    WASM_STR --> RUST_STR

    RUST_F64 -->|format!| RUST_STRING
    RUST_STR -->|to_string| RUST_STRING

    RUST_STRING -->|wasm-bindgen| WASM_OWNED
    WASM_OWNED -->|return| JS_STR

    style WASM_F64 fill:#90EE90
    style WASM_STR fill:#90EE90
```

### Example: Map Click Coordinates

```javascript
// JavaScript → WASM
handle_map_click(1234.567, 890.123)  // JS numbers
```

```rust
// WASM receives
pub fn handle_map_click(x: f64, y: f64)  // Rust f64
{
    let coords = format_coordinates(x, y);  // → String
    // "(1235, 890)"

    let message = format_status_message("Map clicked at", &coords);  // → String
    // "Map clicked at: (1235, 890)"

    log_status(&message);  // &str passed to console
}
```

```javascript
// JavaScript receives
console.log("RTS Status: Map clicked at: (1235, 890)")  // JS string
```

---

## 📦 Data Persistence

### Current State

❌ **No Persistent Storage**
- All data is ephemeral
- Page reload resets everything
- No save/load functionality

### State Lifetime

| Data | Lifetime | Storage |
|------|----------|---------|
| **Viewport Position** | Until page reload | JavaScript memory |
| **Dragging State** | During drag operation | JavaScript memory |
| **Status Messages** | Until next interaction | DOM textContent |
| **Console Logs** | Until cleared | Browser console |

### Future Considerations

If this were a real game:
- **LocalStorage:** Save game state, settings
- **IndexedDB:** Large data (map state, units)
- **Server Sync:** Multiplayer synchronization
- **Session Storage:** Temporary data

---

## 🔗 Related Pages

- **[Architecture](Architecture)** - System design
- **[Interaction Flows](Interaction-Flows)** - Sequence diagrams
- **[Components](Components)** - Component details
- **[Development Guide](Development-Guide)** - Implementation

---

[← Back to Home](Home)
