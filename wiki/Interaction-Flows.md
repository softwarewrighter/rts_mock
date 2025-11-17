# Interaction Flows

This page documents the user interaction flows and sequences in the RTS Mock project using UML-style sequence diagrams.

## 📊 Overview

All user interactions follow a consistent event-driven pattern:

```mermaid
graph LR
    USER[User Action] --> DOM[DOM Event]
    DOM --> JS[JavaScript Handler]
    JS --> WASM[WASM Function]
    WASM --> HELPER[Helper Function]
    HELPER --> LOG[Console Log]
    LOG --> STATUS[Status Display Update]
    STATUS --> USER

    style USER fill:#4169E1
    style WASM fill:#90EE90
    style STATUS fill:#FFD700
```

---

## 1️⃣ Application Initialization Flow

The application must initialize WASM before any interactions can occur.

```mermaid
sequenceDiagram
    actor User
    participant Browser
    participant HTML
    participant JS
    participant WASM
    participant Console

    User->>Browser: Navigate to index.html
    Browser->>HTML: Load HTML/CSS/SVG
    HTML->>Browser: Render UI
    Browser->>JS: Execute <script type="module">
    JS->>JS: import init from './pkg/rts_mock.js'
    JS->>WASM: await init()
    WASM-->>JS: WASM module loaded
    JS->>JS: Setup event handlers
    JS->>JS: Override console.log
    JS->>JS: Initialize map view
    JS->>WASM: log_status("Game UI initialized...")
    WASM->>Console: Log initialization message
    Console->>HTML: Update status display
    HTML-->>User: Display ready UI
    Note over User,HTML: Application ready for interaction
```

### Key Initialization Steps

1. **Load Static Assets** (HTML, CSS, SVG)
2. **Load WASM Module** (`init()`)
3. **Import WASM Functions** (handle_map_click, etc.)
4. **Setup Event Listeners** (click, mousedown, mousemove, keydown)
5. **Override console.log** (to update status display)
6. **Initialize Map View** (center viewport)
7. **Make Functions Global** (attach to `window` object)

---

## 2️⃣ Simple Button Click Flow

Basic interaction pattern for build, research, and unit control buttons.

### Build Button Click

```mermaid
sequenceDiagram
    actor User
    participant Button
    participant DOM
    participant JS
    participant WASM
    participant Helper
    participant Console
    participant Status

    User->>Button: Click "Barracks"
    Button->>DOM: onclick event
    DOM->>JS: window.handleBuildButton('barracks')
    JS->>WASM: handle_build_button('barracks')
    WASM->>Helper: format_status_message('Build', 'barracks selected')
    Helper-->>WASM: 'Build: barracks selected'
    WASM->>Helper: log_status('Build: barracks selected')
    Helper->>Console: console.log('RTS Status: Build: barracks selected')
    Console->>Status: Update textContent
    Status-->>User: Display "Build: barracks selected"

    Note over User,Status: Total latency: <10ms
```

### Research Button Click

```mermaid
sequenceDiagram
    actor User
    participant Button
    participant WASM
    participant Console

    User->>Button: Click "Armor Tech"
    Button->>WASM: handle_research_button('armor')
    WASM->>WASM: format_status_message('Research', 'armor selected')
    WASM->>Console: log_status('Research: armor selected')
    Console-->>User: Update status display

    Note over User,Console: Same pattern as build buttons
```

### Unit Command Click

```mermaid
sequenceDiagram
    actor User
    participant Button
    participant WASM
    participant Console

    User->>Button: Click "ATTACK"
    Button->>WASM: handle_unit_command('attack')
    WASM->>WASM: format_status_message('Unit command', 'attack')
    WASM->>Console: log_status('Unit command: attack')
    Console-->>User: Update status display

    Note over User,Console: Cyan styling instead of green
```

---

## 3️⃣ Map Click Flow (Complex)

Map clicks involve coordinate transformation from screen space to world space.

```mermaid
sequenceDiagram
    actor User
    participant MapSVG
    participant DOM
    participant JS
    participant WASM
    participant Helper
    participant Console
    participant Status

    User->>MapSVG: Click at screen (400, 300)
    MapSVG->>DOM: click event
    DOM->>JS: Event listener triggered
    JS->>JS: Get SVG bounding rect
    JS->>JS: screen → SVG coords<br/>(400, 300) → (400, 300)
    JS->>JS: SVG → Viewport coords<br/>Adjust for (400, 100) offset
    JS->>JS: Apply inverse isometric transform<br/>matrix(0.866, 0.5, -0.866, 0.5, ...)
    JS->>JS: Viewport → World coords<br/>Add viewport position
    JS->>WASM: handle_map_click(1234.5, 978.2)
    WASM->>Helper: format_coordinates(1234.5, 978.2)
    Helper-->>WASM: "(1235, 978)"
    WASM->>Helper: format_status_message("Map clicked at", "(1235, 978)")
    Helper-->>WASM: "Map clicked at: (1235, 978)"
    WASM->>Console: log_status("Map clicked at: (1235, 978)")
    Console->>Status: Update display
    Status-->>User: Show coordinates

    Note over User,Status: Complex coordinate transformation
```

### Coordinate Transformation Steps

1. **Screen Coordinates** - Raw pixel position from click event
2. **SVG Coordinates** - Normalized to SVG viewBox (800×600)
3. **Adjusted Coordinates** - Account for isometric transform offset
4. **Inverse Transform** - Reverse the isometric matrix
5. **World Coordinates** - Add viewport position for final coords

---

## 4️⃣ Map Dragging Flow

Dragging updates viewport position to pan across the larger world map.

```mermaid
sequenceDiagram
    actor User
    participant MapDiv
    participant Document
    participant JS
    participant MapContent

    User->>MapDiv: Mouse down (left button)
    MapDiv->>JS: mousedown event
    JS->>JS: Set isDragging = true
    JS->>JS: Store dragStart position
    JS->>JS: Store viewportStart position
    JS->>MapDiv: Add 'dragging' class<br/>(cursor: grabbing)

    Note over User,MapDiv: User drags mouse...

    User->>Document: Mouse move (while button down)
    Document->>JS: mousemove event
    JS->>JS: Calculate delta (dx, dy)
    JS->>JS: Update viewport position<br/>viewportX = viewportStart.x + dx<br/>viewportY = viewportStart.y + dy
    JS->>JS: Clamp viewport to bounds
    JS->>MapContent: Update SVG transform
    MapContent-->>User: Map pans smoothly

    Note over User,MapContent: Continuous updates while dragging

    User->>Document: Mouse up
    Document->>JS: mouseup event
    JS->>JS: Set isDragging = false
    JS->>MapDiv: Remove 'dragging' class<br/>(cursor: grab)

    Note over User,MapDiv: Drag complete
```

### State Management

JavaScript manages dragging state:
- `isDragging`: boolean
- `dragStart`: {x, y} - initial mouse position
- `viewportStart`: {x, y} - initial viewport position
- `mapState.viewportX/Y`: current viewport position

---

## 5️⃣ Minimap Click Flow

Clicking minimap centers the main map viewport on the clicked position.

```mermaid
sequenceDiagram
    actor User
    participant MinimapSVG
    participant JS
    participant MainMap
    participant WASM
    participant Status

    User->>MinimapSVG: Click at (100, 75)
    MinimapSVG->>JS: click event
    JS->>JS: Get SVG point
    JS->>JS: Transform screen → SVG coords
    JS->>JS: Scale minimap → world coords<br/>scaleX = 2400/200 = 12<br/>scaleY = 1800/150 = 12
    JS->>JS: Calculate world position<br/>worldX = 100 * 12 = 1200<br/>worldY = 75 * 12 = 900
    JS->>JS: Center viewport on position<br/>viewportX = 1200 - 400 = 800<br/>viewportY = 900 - 300 = 600
    JS->>JS: Clamp viewport to bounds
    JS->>MainMap: Update SVG transform
    MainMap-->>User: Map jumps to new position
    JS->>JS: Update minimap viewport rect
    JS->>WASM: handle_minimap_click(100, 75)
    WASM->>Status: Log minimap coordinates
    Status-->>User: Display status

    Note over User,Status: Instant navigation + status log
```

---

## 6️⃣ Edge Scrolling Flow

Moving mouse near edge triggers continuous scrolling.

```mermaid
sequenceDiagram
    actor User
    participant MapDiv
    participant Document
    participant JS
    participant MapContent

    User->>MapDiv: Mouse enters map area
    MapDiv->>Document: mousemove event
    Document->>JS: Check if over map
    JS->>JS: Update currentMousePos

    alt Mouse near edge (< 50px)
        JS->>JS: Start scrollInterval (16ms)
        loop Every 16ms
            JS->>JS: handleEdgeScroll()
            JS->>JS: Calculate scroll direction/speed
            JS->>JS: Update viewport position (±20px)
            JS->>JS: Clamp viewport
            JS->>MapContent: Update transform
            MapContent-->>User: Smooth scrolling
        end
    else Mouse moves away from edge
        JS->>JS: Clear scrollInterval
        Note over JS: Scrolling stops
    end

    User->>Document: Mouse leaves map area
    Document->>JS: mousemove event
    JS->>JS: Clear scrollInterval

    Note over User,MapContent: Auto-scroll while near edges
```

### Edge Scroll Parameters

- **Threshold:** 50px from edge
- **Speed:** 20px per frame
- **Frequency:** Every 16ms (~60fps)
- **Edges:** All four (top, bottom, left, right)

---

## 7️⃣ Keyboard Navigation Flow

Arrow keys or WASD scroll the map.

```mermaid
sequenceDiagram
    actor User
    participant Document
    participant JS
    participant MapContent
    participant WASM
    participant Status

    User->>Document: Press "ArrowUp" or "W"
    Document->>JS: keydown event
    JS->>JS: Check key in switch statement
    JS->>JS: viewportY -= 40
    JS->>JS: Clamp viewport to bounds
    JS->>JS: event.preventDefault()<br/>(prevent page scroll)
    JS->>MapContent: Update SVG transform
    MapContent-->>User: Map scrolls up
    JS->>WASM: log_status("Viewport moved to (x, y)")
    WASM->>Status: Update status display
    Status-->>User: Show new coordinates

    Note over User,Status: Discrete 40px steps per keypress
```

### Keyboard Mappings

| Key | Direction | Delta |
|-----|-----------|-------|
| ↑ or W | Up | viewportY -= 40 |
| ↓ or S | Down | viewportY += 40 |
| ← or A | Left | viewportX -= 40 |
| → or D | Right | viewportX += 40 |

---

## 8️⃣ Resource Panel Click Flow

```mermaid
sequenceDiagram
    actor User
    participant ResourceDiv
    participant WASM
    participant Helper
    participant Console
    participant Status

    User->>ResourceDiv: Click "💰 Gold: 1500"
    ResourceDiv->>WASM: handle_resource_click('gold')
    WASM->>Helper: format_status_message('Resource panel clicked', 'gold')
    Helper-->>WASM: 'Resource panel clicked: gold'
    WASM->>Console: log_status('Resource panel clicked: gold')
    Console->>Status: Update textContent
    Status-->>User: Display status

    Note over User,Status: Simple click → log pattern
```

---

## 9️⃣ Console Log Override Flow

The application overrides `console.log` to update the status display.

```mermaid
sequenceDiagram
    participant WASM
    participant Console
    participant JS
    participant StatusDiv
    participant User

    WASM->>Console: console.log("RTS Status: Map clicked at: (100, 200)")
    Console->>JS: Overridden console.log function
    JS->>JS: originalLog(message)<br/>(preserve console output)
    JS->>JS: Check if message contains "RTS Status:"
    alt Contains "RTS Status:"
        JS->>JS: Extract message<br/>Remove "RTS Status: " prefix
        JS->>StatusDiv: statusEl.textContent = message
        StatusDiv-->>User: Display "Map clicked at: (100, 200)"
    else Does not contain "RTS Status:"
        Note over JS,StatusDiv: Ignore - not a status message
    end

    Note over WASM,User: Dual output: console + status display
```

### Implementation

```javascript
// Override console.log
const originalLog = console.log;
console.log = function(message) {
    originalLog(message);  // Still log to console
    const statusEl = document.getElementById('status');
    if (statusEl && typeof message === 'string' && message.includes('RTS Status:')) {
        statusEl.textContent = message.replace('RTS Status: ', '');
    }
};
```

---

## 🔄 Complete Interaction Summary

```mermaid
graph TB
    subgraph "User Actions"
        CLICK[Click Element]
        DRAG[Drag Map]
        KEYBOARD[Press Key]
        HOVER[Hover Near Edge]
    end

    subgraph "Event Handlers"
        ONCLICK[onclick Handler]
        MOUSEDOWN[onmousedown]
        MOUSEMOVE[onmousemove]
        KEYDOWN[onkeydown]
    end

    subgraph "JavaScript Layer"
        COORD[Coordinate Transform]
        STATE[State Update]
        SCROLL[Auto Scroll]
    end

    subgraph "WASM Layer"
        HANDLERS[Event Handlers]
        HELPERS[Format Helpers]
        LOG[log_status]
    end

    subgraph "Output"
        CONSOLE[Console Log]
        STATUS[Status Display]
        VISUAL[Visual Feedback]
    end

    CLICK --> ONCLICK
    DRAG --> MOUSEDOWN
    DRAG --> MOUSEMOVE
    KEYBOARD --> KEYDOWN
    HOVER --> MOUSEMOVE

    ONCLICK --> HANDLERS
    MOUSEDOWN --> STATE
    MOUSEMOVE --> COORD
    MOUSEMOVE --> SCROLL
    KEYDOWN --> STATE

    COORD --> HANDLERS
    STATE --> VISUAL
    SCROLL --> STATE

    HANDLERS --> HELPERS
    HELPERS --> LOG
    LOG --> CONSOLE
    CONSOLE --> STATUS
    STATE --> VISUAL

    style CLICK fill:#4169E1
    style HANDLERS fill:#90EE90
    style STATUS fill:#FFD700
```

---

## ⚡ Performance Characteristics

### Latency Breakdown

| Interaction | Layers | Estimated Latency |
|------------|--------|------------------|
| **Button Click** | DOM → JS → WASM → Console | < 5ms |
| **Map Click** | DOM → Coord Transform → WASM | < 10ms |
| **Map Drag** | DOM → JS State → SVG Transform | < 2ms per frame |
| **Edge Scroll** | Timer → JS State → SVG Transform | 16ms (60fps) |
| **Keyboard** | DOM → JS State → SVG Transform | < 5ms |
| **Minimap Click** | DOM → Coord Scale → Map Update | < 10ms |

### Bottlenecks

✅ **Fast:**
- WASM function calls (native speed)
- SVG transforms (GPU accelerated)
- Event propagation

⚠️ **Potential Issues:**
- Many simultaneous SVG element updates (not currently an issue)
- Complex coordinate transformations (optimized with inverse matrix)

---

## 🔗 Related Pages

- **[Components](Components)** - Component details
- **[Architecture](Architecture)** - System architecture
- **[Data Flow](Data-Flow)** - Data movement patterns
- **[Development Guide](Development-Guide)** - Implementation details

---

[← Back to Home](Home)
