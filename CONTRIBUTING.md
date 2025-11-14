# Contributing to RTS Mock

Thank you for your interest in contributing to this project! This document provides guidelines and instructions for contributing.

## Project Philosophy

This is a **Rust-first** WebAssembly project with the following core principles:

1. **Rust/WASM for all logic**: Business logic, domain logic, and as much presentation logic as possible should be in Rust
2. **Minimal JavaScript**: JavaScript should only be used for WASM initialization and minimal DOM event wiring
3. **No TypeScript**: We use Rust's type system instead of TypeScript
4. **Clean separation**: HTML (structure), CSS (presentation), Rust (behavior/logic)
5. **Educational focus**: Code should be clear and well-documented for learning purposes

## Getting Started

### Prerequisites

- Rust (latest stable) - Install from https://rustup.rs/
- wasm-pack - Install from https://rustwasm.github.io/wasm-pack/installer/
- A modern web browser
- Git

### Setting Up Development Environment

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/YOUR-USERNAME/rts_mock.git
   cd rts_mock
   ```

2. Build the WASM package:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

3. Serve locally:
   ```bash
   basic-http-server -a 0.0.0.0:8000
   ```

4. Open `http://localhost:8000/index.html` in your browser

## Development Workflow

### Before Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Ensure tests pass:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

### Making Changes

1. **Rust Code** (src/lib.rs):
   - All interaction handlers should use `#[wasm_bindgen]` attribute
   - Use descriptive function names
   - Add unit tests for helper functions
   - Add WASM tests for browser integration
   - Keep functions focused and single-purpose

2. **HTML** (index.html):
   - Semantic HTML structure only
   - Minimal inline event handlers (onclick, etc.)
   - No inline styles (use CSS classes)
   - Keep accessibility in mind (ARIA labels, semantic tags)

3. **CSS** (styles.css or inline):
   - Follow existing color scheme (green terminal aesthetic)
   - Use CSS Grid for layout
   - Ensure responsive design
   - No JavaScript in CSS (use Rust for dynamic behavior)

4. **Documentation**:
   - Update README.md if adding features
   - Update docs/overview.md for architectural changes
   - Add code comments for complex logic
   - Update CLAUDE.md for development guidance

### Code Style

#### Rust
- Follow standard Rust conventions (`cargo fmt`)
- Use `cargo clippy` and fix all warnings
- Prefer explicit types over inference in public APIs
- Use descriptive variable names
- Keep functions under 50 lines when possible

#### JavaScript (Minimal)
- Only in `<script type="module">` section of index.html
- Only for WASM initialization and event wiring
- Keep under 50 lines total
- No business logic - move to Rust instead

#### CSS
- Use kebab-case for class names
- Group related rules together
- Comment sections (/* Resource Panel */)
- Mobile-first responsive design

### Testing

#### Unit Tests
```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_format_coordinates
```

#### WASM Tests
```bash
# Run in Firefox headless
wasm-pack test --headless --firefox

# Run in Chrome headless
wasm-pack test --headless --chrome
```

#### Manual Testing
1. Build: `wasm-pack build --target web --out-dir pkg`
2. Serve: `basic-http-server -a 0.0.0.0:8000`
3. Test all UI interactions in browser
4. Check browser console for errors
5. Verify status messages appear correctly

### Pre-Commit Checklist

- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] All tests pass: `cargo test`
- [ ] WASM builds successfully: `wasm-pack build --target web --out-dir pkg`
- [ ] Manual testing completed in browser
- [ ] Documentation updated if needed
- [ ] Commit message is descriptive

## Contribution Guidelines

### What We're Looking For

**Welcome contributions:**
- Bug fixes
- New interactive UI elements (buttons, menus, etc.)
- Visual enhancements (animations, effects)
- Performance improvements
- Documentation improvements
- Test coverage improvements
- Accessibility improvements
- Mobile responsiveness improvements

**Not currently accepting:**
- Full game logic implementation (out of scope for mockup)
- Server-side components
- TypeScript additions
- Heavy JavaScript frameworks (React, Vue, etc.)
- Non-Rust logic implementations

### Pull Request Process

1. **Create descriptive PR title**:
   - Good: "Add unit selection with visual feedback"
   - Bad: "Update code"

2. **PR Description should include**:
   - What: What does this change do?
   - Why: Why is this change needed?
   - How: How does it work?
   - Testing: How was it tested?
   - Screenshots: If visual changes, include before/after

3. **Link related issues**:
   - Use "Fixes #123" or "Closes #123" in description

4. **Ensure CI passes**:
   - All tests must pass
   - No clippy warnings
   - Code must be formatted

5. **Request review**:
   - Wait for maintainer review
   - Address feedback promptly
   - Keep discussion respectful

### Commit Message Format

We use conventional commits for clarity:

```
type(scope): brief description

Longer description if needed explaining what and why.

- Bullet points for details
- Additional context

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(ui): add building placement mode with grid snapping

fix(minimap): correct coordinate transformation for clicks

docs(architecture): add state management section

test(lib): add unit tests for coordinate helpers
```

## Code Review

### What We Look For

- **Correctness**: Does it work as intended?
- **Testing**: Are there appropriate tests?
- **Documentation**: Is it well-documented?
- **Style**: Does it follow project conventions?
- **Performance**: Is it reasonably efficient?
- **Simplicity**: Is it as simple as possible?
- **Rust-first**: Does it maximize Rust usage vs JavaScript?

### Review Timeline

- Initial response: Within 3-5 days
- Full review: Within 1-2 weeks
- Merge: After approval and CI passing

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open a GitHub Issue with reproduction steps
- **Feature Ideas**: Open a GitHub Issue with use case description
- **Documentation**: Check docs/ folder and README.md
- **Real-time Help**: Open a Draft PR and ask questions there

## Recognition

All contributors will be:
- Listed in the repository contributors
- Credited in release notes
- Given co-author credit via Git commits

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to RTS Mock! Your efforts help make this project better for everyone learning Rust and WebAssembly.
