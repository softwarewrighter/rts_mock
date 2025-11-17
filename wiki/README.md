# Wiki Pages for GitHub

This directory contains comprehensive documentation pages for the RTS Mock project GitHub Wiki.

## 📚 Wiki Pages Included

1. **Home.md** - Main wiki landing page with navigation
2. **Architecture.md** - System architecture with block diagrams
3. **Components.md** - Detailed UI component documentation
4. **Interaction-Flows.md** - UML sequence diagrams for user interactions
5. **Data-Flow.md** - Data flow diagrams and state management
6. **Testing.md** - Testing strategy and test documentation
7. **Development-Guide.md** - Setup, build, and contribution guide
8. **API-Reference.md** - Complete function reference

## 🚀 How to Upload to GitHub Wiki

### Method 1: Via GitHub Web Interface

1. **Enable Wiki** (if not already enabled):
   - Go to `https://github.com/softwarewrighter/rts_mock/settings`
   - Check "Wikis" under Features
   - Save changes

2. **Create Initial Page**:
   - Visit `https://github.com/softwarewrighter/rts_mock/wiki`
   - Click "Create the first page"
   - Enter any content and save (will be replaced)

3. **Clone Wiki Repository**:
   ```bash
   git clone https://github.com/softwarewrighter/rts_mock.wiki.git
   cd rts_mock.wiki
   ```

4. **Copy Wiki Pages**:
   ```bash
   # From main repository root
   cp wiki/*.md ../rts_mock.wiki/
   ```

5. **Commit and Push**:
   ```bash
   cd ../rts_mock.wiki
   git add *.md
   git commit -m "Add comprehensive wiki documentation"
   git push origin master
   ```

6. **Verify**:
   - Visit `https://github.com/softwarewrighter/rts_mock/wiki`
   - Pages should now be visible

### Method 2: Direct Git Clone and Push

```bash
# Clone wiki repository
git clone https://github.com/softwarewrighter/rts_mock.wiki.git
cd rts_mock.wiki

# Copy all wiki pages
cp ../rts_mock/wiki/*.md .

# Remove this README (not needed in wiki)
rm README.md

# Commit and push
git add *.md
git commit -m "Add comprehensive project documentation

Includes:
- Architecture with UML diagrams
- Component documentation
- Interaction flow sequences
- Data flow diagrams
- Testing strategy
- Development guide
- API reference
"
git push origin master
```

## 📋 Page Structure

The wiki follows this organization:

```
Home (Home.md)
├── Architecture (Architecture.md)
│   └── Technology stack
│   └── Design patterns
│   └── Build flow
│
├── Components (Components.md)
│   └── Resource Panel
│   └── Main Map
│   └── Minimap
│   └── Build/Research/Controls
│
├── Interaction Flows (Interaction-Flows.md)
│   └── Initialization flow
│   └── Click handlers
│   └── Map navigation
│
├── Data Flow (Data-Flow.md)
│   └── Coordinate transformations
│   └── State management
│   └── Message routing
│
├── Testing (Testing.md)
│   └── Unit tests
│   └── WASM tests
│   └── Test coverage
│
├── Development Guide (Development-Guide.md)
│   └── Setup instructions
│   └── Build process
│   └── Contributing guidelines
│
└── API Reference (API-Reference.md)
    └── Public functions
    └── Helper functions
    └── Type mappings
```

## 🎨 Features

### Mermaid Diagrams

All pages include **Mermaid diagrams** for visualization:
- Architecture block diagrams
- Sequence diagrams for interactions
- State diagrams for flows
- Data flow diagrams
- Component relationship graphs

**Note:** GitHub natively supports Mermaid in wikis, so diagrams will render automatically.

### Internal Links

Pages use wiki-style internal links:
```markdown
[Architecture](Architecture)
[Components](Components)
```

These work automatically in GitHub wikis.

### Code Examples

Each page includes:
- Rust code snippets
- JavaScript examples
- Bash commands
- Configuration examples

## ✅ Verification Checklist

After uploading to GitHub Wiki:

- [ ] All 8 pages are visible in wiki sidebar
- [ ] Mermaid diagrams render correctly
- [ ] Internal links work (click between pages)
- [ ] Code blocks have proper syntax highlighting
- [ ] Tables display correctly
- [ ] Images (if any) load properly
- [ ] Search function finds content

## 🔗 Quick Links

Once uploaded, the wiki will be accessible at:
- **Wiki Home:** `https://github.com/softwarewrighter/rts_mock/wiki`
- **Architecture:** `https://github.com/softwarewrighter/rts_mock/wiki/Architecture`
- **API Reference:** `https://github.com/softwarewrighter/rts_mock/wiki/API-Reference`

## 📝 Maintenance

To update wiki pages:

1. Edit markdown files in this directory
2. Test locally with any markdown viewer
3. Follow upload steps above to update GitHub wiki

**Note:** GitHub wikis use Git, so you can also clone, edit directly, and push changes.

---

## 🤝 Contributing

If you update the wiki:
1. Update the corresponding `.md` file in this directory
2. Commit changes to main repository
3. Push updates to GitHub wiki repository

This keeps both locations in sync.
