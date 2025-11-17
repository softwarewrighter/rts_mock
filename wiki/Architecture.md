# Architecture

This page describes the high-level architecture of the RTS Mock project, including system design, technology stack, and architectural patterns.

## 📐 High-Level Architecture

The RTS Mock follows a **clean separation of concerns** architecture with three distinct layers:

```mermaid
graph TB
    subgraph "Presentation Layer"
        HTML[index.html<br/>Complete UI Layout]
        CSS[Inline CSS<br/>Styling & Theme]
        SVG[SVG Graphics<br/>Map & Minimap]
    end

    subgraph "Glue Layer"
        JS[Minimal JavaScript<br/>WASM Initialization]
        JSEVT[Event Listeners<br/>onclick handlers]
    end

    subgraph "Logic Layer"
        WASM[WASM Binary<br/>Compiled Rust]
        HANDLERS[Event Handlers<br/>wasm-bindgen exports]
        HELPERS[Helper Functions<br/>Formatting utilities]
    end

    subgraph "Browser APIs"
        DOM[DOM API]
        CONSOLE[Console API]
        WEBSYS[web-sys bindings]
    end

    HTML --> CSS
    HTML --> SVG
    HTML --> JSEVT
    JSEVT --> JS
    JS --> HANDLERS
    HANDLERS --> WASM
    WASM --> HELPERS
    HANDLERS --> WEBSYS
    WEBSYS --> DOM
    WEBSYS --> CONSOLE

    style WASM fill:#90EE90
    style HTML fill:#4169E1
    style JS fill:#FFD700
```

---

## 🏛️ Architectural Layers

### 1. Presentation Layer (HTML/CSS/SVG)

**Location:** `index.html`

**Responsibilities:**
- Define complete UI structure and layout
- Provide visual styling (retro terminal aesthetic)
- Render SVG graphics for maps
- Display static content

**Key Design Decisions:**
- ✅ All styling is **inline** in `index.html` (no separate CSS files)
- ✅ Uses **CSS Grid** for responsive layout
- ✅ SVG for scalable graphics
- ✅ Fixed positioning for overlays (status, footer)

```mermaid
graph LR
    subgraph "UI Layout CSS Grid"
        RES[Resource Panel]
        MAP[Main Map SVG]
        MINI[Minimap SVG]
        BUILD[Build Menu]
        RSRCH[Research Menu]
        CTRL[Unit Controls]
        STATUS[Status Display]
        FOOTER[Footer]
    end

    RES --> |Grid Area: resources| GRID[Grid Container]
    MAP --> |Grid Area: map| GRID
    MINI --> |Grid Area: minimap| GRID
    BUILD --> |Grid Area: build| GRID
    RSRCH --> |Grid Area: research| GRID
    CTRL --> |Grid Area: controls| GRID
    STATUS --> |Fixed: bottom 30px| GRID
    FOOTER --> |Fixed: bottom 0| GRID

    style GRID fill:#333333
```

### 2. Glue Layer (JavaScript)

**Location:** `index.html` (inline `<script type="module">`)

**Responsibilities:**
- Initialize WASM module
- Set up event handlers
- Manage map viewport state
- Handle coordinate transformations
- Provide keyboard controls

**Key Design Decisions:**
- ✅ **Minimal JavaScript** - only what's necessary for WASM/DOM bridging
- ✅ Map state management kept in JS (viewport position, dragging state)
- ✅ Coordinate transformation for isometric projection
- ✅ Edge scrolling and keyboard navigation

```mermaid
graph TD
    INIT[WASM Initialization] --> IMPORT[Import WASM Functions]
    IMPORT --> SETUP[Setup Event Handlers]
    SETUP --> MAP_EVT[Map Events]
    SETUP --> MINI_EVT[Minimap Events]
    SETUP --> BTN_EVT[Button Events]
    SETUP --> KBD_EVT[Keyboard Events]

    MAP_EVT --> |Click| COORD[Coordinate Transform]
    COORD --> WASM_FUNC[Call WASM Function]

    MINI_EVT --> |Click| VIEWPORT[Update Viewport]
    VIEWPORT --> RENDER[Re-render View]

    BTN_EVT --> |onclick| WASM_FUNC
    KBD_EVT --> |Arrow/WASD| SCROLL[Scroll Map]

    style INIT fill:#FFD700
    style WASM_FUNC fill:#90EE90
```

### 3. Logic Layer (Rust/WASM)

**Location:** `src/lib.rs`

**Responsibilities:**
- Handle all UI interactions
- Format status messages
- Coordinate formatting
- Expose public API via `wasm-bindgen`

**Key Design Decisions:**
- ✅ **No state management** - purely reactive to events
- ✅ All public functions exposed via `#[wasm_bindgen]`
- ✅ Helper functions for message formatting
- ✅ Console logging for status updates

```mermaid
graph TD
    subgraph "WASM Public API"
        MAP_CLICK[handle_map_click]
        MINI_CLICK[handle_minimap_click]
        BUILD[handle_build_button]
        RESEARCH[handle_research_button]
        UNIT_CMD[handle_unit_command]
        RES_CLICK[handle_resource_click]
        LOG[log_status]
    end

    subgraph "Helper Functions"
        FMT_STATUS[format_status_message]
        FMT_COORD[format_coordinates]
    end

    MAP_CLICK --> FMT_COORD
    MINI_CLICK --> FMT_COORD
    FMT_COORD --> FMT_STATUS
    BUILD --> FMT_STATUS
    RESEARCH --> FMT_STATUS
    UNIT_CMD --> FMT_STATUS
    RES_CLICK --> FMT_STATUS
    FMT_STATUS --> LOG
    LOG --> CONSOLE[web_sys::console]

    style MAP_CLICK fill:#ff9900
    style CONSOLE fill:#4169E1
```

---

## 🔧 Technology Stack Details

### Core Technologies

```mermaid
graph LR
    subgraph "Development"
        RUST[Rust 2021]
        CARGO[Cargo]
        WASM_PACK[wasm-pack]
    end

    subgraph "Runtime"
        WASM_BIN[WASM Binary]
        JS_GLUE[JS Glue Code]
        HTML_UI[HTML UI]
    end

    subgraph "Browser"
        WASM_VM[WASM VM]
        JS_ENGINE[JavaScript Engine]
        RENDER[Rendering Engine]
    end

    RUST --> |Compile| WASM_PACK
    CARGO --> WASM_PACK
    WASM_PACK --> WASM_BIN
    WASM_PACK --> JS_GLUE
    WASM_BIN --> WASM_VM
    JS_GLUE --> JS_ENGINE
    HTML_UI --> RENDER

    style RUST fill:#ff9900
    style WASM_BIN fill:#90EE90
    style RENDER fill:#4169E1
```

### Dependencies

**Rust Dependencies (`Cargo.toml`):**

| Crate | Version | Purpose |
|-------|---------|---------|
| `wasm-bindgen` | Latest | Rust ↔ JavaScript interop |
| `web-sys` | Latest | Browser API bindings |
| `wasm-bindgen-test` | Latest | WASM testing framework |

**Build Tools:**

- **wasm-pack** - WASM compilation and packaging
- **cargo** - Rust build system and package manager

---

## 🎨 Design Patterns

### 1. Event-Driven Architecture

All user interactions follow an event-driven pattern:

```mermaid
sequenceDiagram
    participant User
    participant DOM
    participant JS
    participant WASM
    participant Console

    User->>DOM: Click Button
    DOM->>JS: onclick Event
    JS->>WASM: handle_build_button("barracks")
    WASM->>WASM: format_status_message()
    WASM->>Console: log_status()
    Console->>DOM: Update Status Display
    DOM->>User: Visual Feedback
```

### 2. Separation of Concerns

**Clear boundaries between layers:**

| Layer | Technology | Responsibility |
|-------|-----------|----------------|
| **Presentation** | HTML/CSS/SVG | Structure & Styling |
| **Interaction** | JavaScript | Event Handling & State |
| **Logic** | Rust/WASM | Business Logic |
| **Output** | web-sys | Console & DOM Updates |

### 3. Functional Composition

Helper functions compose to build complex behaviors:

```rust
// Low-level helpers
format_coordinates(x, y) -> String
format_status_message(prefix, details) -> String

// High-level handlers
handle_map_click(x, y) {
    message = format_status_message("Map clicked at", format_coordinates(x, y))
    log_status(message)
}
```

---

## 📦 File Structure

```
rts_mock/
├── src/
│   ├── lib.rs              # Main WASM module
│   │   ├── Public API (wasm-bindgen exports)
│   │   ├── Helper functions
│   │   ├── Unit tests
│   │   └── WASM tests
│   └── main.rs             # Unused (CLI entry point)
│
├── pkg/                    # Generated WASM output
│   ├── rts_mock.js         # JS glue code
│   ├── rts_mock_bg.wasm    # WASM binary
│   └── rts_mock.d.ts       # TypeScript definitions
│
├── index.html              # Complete UI
│   ├── HTML structure
│   ├── Inline CSS
│   ├── SVG graphics
│   └── Inline JavaScript
│
├── docs/
│   ├── overview.md         # Project overview
│   └── claude-web-research-preview-notes.md
│
├── wiki/                   # Wiki pages (this documentation)
│
├── Cargo.toml              # Rust dependencies
├── CLAUDE.md               # Development guidance
├── README.md               # User documentation
└── LICENSE                 # MIT License
```

---

## 🔄 Build & Deployment Flow

```mermaid
graph TB
    SRC[src/lib.rs<br/>Rust Source] --> BUILD[wasm-pack build]
    BUILD --> PKG[pkg/ directory]

    PKG --> WASM[rts_mock_bg.wasm<br/>Binary]
    PKG --> JSGLUE[rts_mock.js<br/>Glue Code]
    PKG --> TYPES[rts_mock.d.ts<br/>TypeScript Defs]

    WASM --> SERVE[HTTP Server]
    JSGLUE --> SERVE
    HTML[index.html] --> SERVE
    SERVE --> BROWSER[Web Browser]

    BROWSER --> LOAD[Load HTML]
    LOAD --> INIT[Initialize WASM]
    INIT --> READY[Ready for Interaction]

    style BUILD fill:#ff9900
    style BROWSER fill:#4169E1
    style READY fill:#90EE90
```

**Build Command:**
```bash
wasm-pack build --target web --out-dir pkg
```

**Output:**
- `pkg/rts_mock_bg.wasm` - Compiled WASM binary
- `pkg/rts_mock.js` - JavaScript glue code
- `pkg/rts_mock.d.ts` - TypeScript type definitions
- `pkg/package.json` - NPM package metadata

---

## 🚀 Performance Characteristics

### Advantages of WASM Architecture

✅ **Fast Execution**
- Native-speed computation for event handlers
- Efficient string formatting and manipulation

✅ **Small Binary Size**
- Minimal logic = minimal WASM size
- No heavy dependencies

✅ **Type Safety**
- Rust's type system catches errors at compile time
- `wasm-bindgen` ensures correct JS ↔ WASM communication

✅ **Memory Safety**
- Rust guarantees memory safety without GC overhead
- No memory leaks from logic layer

### Current Limitations

⚠️ **No State Persistence**
- All state is ephemeral (resets on page reload)
- No local storage or server communication

⚠️ **Minimal Logic**
- This is a UI mockup - no actual game logic
- All interactions just log status messages

⚠️ **Inline Styles**
- CSS is inline in HTML (maintainability tradeoff)
- Intentional design choice for simplicity

---

## 🔗 Related Pages

- **[Components](Components)** - Detailed component breakdown
- **[Interaction Flows](Interaction-Flows)** - Sequence diagrams
- **[Data Flow](Data-Flow)** - Data movement patterns
- **[Development Guide](Development-Guide)** - Build and deploy instructions

---

[← Back to Home](Home)
