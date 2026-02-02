# KSG and Triangular Lattice Refactoring Design

## Overview

Refactor the `unitdiskmapping` module to:
1. Split unweighted and weighted gadgets into independent implementations (no wrapper pattern)
2. Rename to reflect the actual graph types: King's Subgraph (KSG) and Triangular lattice
3. Organize by lattice type (two groups) rather than by weight mode (three groups)

## Background

### Julia (UnitDiskMapping.jl) Naming

Julia uses three mapping modes:
- `UnWeighted()` - Square lattice, unweighted nodes
- `Weighted()` - Square lattice, weighted nodes
- `TriangularWeighted()` - Triangular lattice, weighted nodes

Julia uses a `WeightedGadget{T}` wrapper to add weight vectors to base gadgets.

### Current Rust Implementation

- `map_graph()` - Square lattice unweighted mapping
- `map_graph_triangular()` - Triangular lattice weighted mapping
- `WeightedGadget<G>` wrapper similar to Julia
- Gadgets: `Cross`, `Turn`, `Branch` (square); `TriCross`, `TriTurn`, `TriBranch` (triangular)

### Problems with Current Approach

1. **Wrapper complexity** - `WeightedGadget<T>` adds indirection and complexity
2. **Unclear naming** - "square lattice" doesn't convey the King's Subgraph structure
3. **Mixed organization** - Files don't clearly separate by lattice type

## Design Decisions

### 1. Gadget Implementation: Duplicate Structs

**Decision:** Use separate struct types for unweighted and weighted gadgets instead of a wrapper.

**Rationale:**
- Simpler implementation without complex generics
- Independent evolution - each can be optimized separately
- More explicit and easier to understand
- Matches user preference for explicit naming

**Example:**
```rust
// ksg/gadgets.rs - Unweighted
pub struct KsgCross<const CONNECTED: bool>;
impl KsgCross<CONNECTED> {
    pub fn source_graph(&self) -> Vec<(usize, usize)> { ... }
    pub fn mapped_graph(&self) -> Vec<(usize, usize)> { ... }
    pub fn mis_overhead(&self) -> i32 { ... }
}

// ksg/gadgets_weighted.rs - Weighted (independent implementation)
pub struct WeightedKsgCross<const CONNECTED: bool>;
impl WeightedKsgCross<CONNECTED> {
    pub fn source_graph(&self) -> Vec<(usize, usize, i32)> { ... }
    pub fn mapped_graph(&self) -> Vec<(usize, usize, i32)> { ... }
    pub fn mis_overhead(&self) -> i32 { ... }  // 2x unweighted
}
```

### 2. Naming Convention

**Decision:** Use explicit prefixes reflecting lattice type and weight mode.

| Lattice | Mode | Gadget Prefix | Example |
|---------|------|---------------|---------|
| King's Subgraph | Unweighted | `Ksg` | `KsgCross`, `KsgTurn` |
| King's Subgraph | Weighted | `WeightedKsg` | `WeightedKsgCross`, `WeightedKsgTurn` |
| Triangular | Weighted | `WeightedTri` | `WeightedTriCross`, `WeightedTriTurn` |

**Rationale:**
- Explicit naming avoids ambiguity
- `Ksg` prefix clearly indicates King's Subgraph (8-connectivity)
- `WeightedTri` makes clear triangular is always weighted in this implementation

### 3. Module Organization: Two Groups by Lattice Type

**Decision:** Organize into two submodules by lattice geometry, not by weight mode.

**Rationale:**
- Lattice geometry is the fundamental distinction
- Weighted vs unweighted is a parameter, not a different lattice
- Matches Julia's mental model
- Allows sharing code within each lattice type
- Scales well if `triangular::map_unweighted()` is added later

### 4. API Style: Module Namespace

**Decision:** Use module namespace for function organization.

```rust
use problemreductions::rules::unitdiskmapping::{ksg, triangular};

let result1 = ksg::map_unweighted(n, &edges);
let result2 = ksg::map_weighted(n, &edges);
let result3 = triangular::map_weighted(n, &edges);
```

**Rationale:**
- Groups related functions naturally
- Clean call sites
- Matches file structure
- Scales well for future additions

### 5. Migration Strategy: Clean Break

**Decision:** Delete old files immediately, no deprecation period.

**Rationale:**
- PR #13 is not yet merged, so no backward compatibility needed
- Cleaner codebase without deprecated re-exports
- Avoids confusion during transition

## File Structure

### Before (Current)
```
src/rules/unitdiskmapping/
├── mod.rs
├── alpha_tensor.rs
├── copyline.rs
├── gadgets.rs              # Mixed square gadgets
├── gadgets_unweighted.rs   # Unweighted square gadgets
├── grid.rs
├── map_graph.rs            # Square lattice mapping
├── pathdecomposition.rs
├── triangular.rs           # Triangular gadgets + mapping
└── weighted.rs             # WeightedGadget wrapper
```

### After (New)
```
src/rules/unitdiskmapping/
├── mod.rs                    # Re-exports ksg and triangular
├── alpha_tensor.rs           # Shared - verification tool
├── copyline.rs               # Shared - copy line creation
├── grid.rs                   # Shared - grid representation
├── pathdecomposition.rs      # Shared - vertex ordering
│
├── ksg/
│   ├── mod.rs                # Exports gadgets and mapping functions
│   ├── gadgets.rs            # KsgCross, KsgTurn, KsgBranch, etc.
│   ├── gadgets_weighted.rs   # WeightedKsgCross, WeightedKsgTurn, etc.
│   └── mapping.rs            # map_unweighted(), map_weighted()
│
└── triangular/
    ├── mod.rs                # Exports gadgets and mapping functions
    ├── gadgets.rs            # WeightedTriCross, WeightedTriTurn, etc.
    └── mapping.rs            # map_weighted()
```

### Files to Delete
- `gadgets.rs` (merged into `ksg/gadgets.rs`)
- `gadgets_unweighted.rs` (merged into `ksg/gadgets.rs`)
- `weighted.rs` (no longer needed)
- `triangular.rs` (split into `triangular/gadgets.rs` and `triangular/mapping.rs`)
- `map_graph.rs` (split into `ksg/mapping.rs`)

## Julia vs Rust Naming Comparison

This comparison should be added to issue #8.

| Concept | Julia (UnitDiskMapping.jl) | Rust (new) |
|---------|---------------------------|------------|
| **Modes/Methods** | | |
| Square unweighted | `UnWeighted()` | `ksg::map_unweighted()` |
| Square weighted | `Weighted()` | `ksg::map_weighted()` |
| Triangular weighted | `TriangularWeighted()` | `triangular::map_weighted()` |
| **Lattice types** | | |
| Square lattice | `SquareGrid` | King's Subgraph (KSG) |
| Triangular lattice | `TriangularGrid` | Triangular |
| **Gadgets (square unweighted)** | | |
| Crossing | `Cross{CON}` | `KsgCross<CON>` |
| Turn | `Turn` | `KsgTurn` |
| Branch | `Branch` | `KsgBranch` |
| Branch fix | `BranchFix` | `KsgBranchFix` |
| W-turn | `WTurn` | `KsgWTurn` |
| End turn | `EndTurn` | `KsgEndTurn` |
| Trivial turn | `TrivialTurn` | `KsgTrivialTurn` |
| T-connection | `TCon` | `KsgTCon` |
| **Gadgets (square weighted)** | | |
| Weighted wrapper | `WeightedGadget{T}` | *(none - separate types)* |
| Crossing | `WeightedGadget{Cross{CON}}` | `WeightedKsgCross<CON>` |
| Turn | `WeightedGadget{Turn}` | `WeightedKsgTurn` |
| Branch | `WeightedGadget{Branch}` | `WeightedKsgBranch` |
| **Gadgets (triangular)** | | |
| Crossing | `TriCross{CON}` | `WeightedTriCross<CON>` |
| Turn | `TriTurn` | `WeightedTriTurn` |
| Branch | `TriBranch` | `WeightedTriBranch` |
| W-turn | `TriWTurn` | `WeightedTriWTurn` |
| End turn | `TriEndTurn` | `WeightedTriEndTurn` |
| Trivial turn (left) | `TriTrivialTurn` (rotated) | `WeightedTriTrivialTurnLeft` |
| Trivial turn (right) | `TriTrivialTurn` | `WeightedTriTrivialTurnRight` |
| T-connection (left) | `TriTCon` (rotated) | `WeightedTriTConLeft` |
| T-connection (up) | `TriTCon` | `WeightedTriTConUp` |
| T-connection (down) | `TriTCon` (rotated) | `WeightedTriTConDown` |
| Branch fix | `TriBranchFix` | `WeightedTriBranchFix` |
| Branch fix B | `TriBranchFixB` | `WeightedTriBranchFixB` |

**Key architectural difference:** Julia uses a `WeightedGadget{T}` wrapper pattern to add weights to any gadget. Rust uses independent weighted types (`WeightedKsgCross`, `WeightedTriCross`) for cleaner separation and simpler implementation.

## Public API

### King's Subgraph (KSG) Module

```rust
pub mod ksg {
    // Mapping functions
    pub fn map_unweighted(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult;
    pub fn map_weighted(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult;
    pub fn map_unweighted_with_order(...) -> MappingResult;
    pub fn map_weighted_with_order(...) -> MappingResult;

    // Unweighted gadgets
    pub struct KsgCross<const CONNECTED: bool>;
    pub struct KsgTurn;
    pub struct KsgBranch;
    pub struct KsgBranchFix;
    pub struct KsgBranchFixB;
    pub struct KsgWTurn;
    pub struct KsgEndTurn;
    pub struct KsgTrivialTurn;
    pub struct KsgTCon;
    pub struct KsgDanglingLeg;

    // Weighted gadgets
    pub struct WeightedKsgCross<const CONNECTED: bool>;
    pub struct WeightedKsgTurn;
    pub struct WeightedKsgBranch;
    // ... etc

    // Constants
    pub const SPACING: usize = 4;
    pub const PADDING: usize = 2;
}
```

### Triangular Module

```rust
pub mod triangular {
    // Mapping functions
    pub fn map_weighted(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult;
    pub fn map_weighted_with_order(...) -> MappingResult;

    // Weighted gadgets (triangular is always weighted)
    pub struct WeightedTriCross<const CONNECTED: bool>;
    pub struct WeightedTriTurn;
    pub struct WeightedTriBranch;
    pub struct WeightedTriBranchFix;
    pub struct WeightedTriBranchFixB;
    pub struct WeightedTriWTurn;
    pub struct WeightedTriEndTurn;
    pub struct WeightedTriTrivialTurnLeft;
    pub struct WeightedTriTrivialTurnRight;
    pub struct WeightedTriTConLeft;
    pub struct WeightedTriTConUp;
    pub struct WeightedTriTConDown;
    pub struct WeightedTriDanglingLeg;

    // Constants
    pub const SPACING: usize = 6;
    pub const PADDING: usize = 2;
}
```

## Testing Strategy

1. **Gadget unit tests** - Each gadget has MIS equivalence tests
2. **Mapping integration tests** - Compare with Julia trace files
3. **Round-trip tests** - map_config_back extracts valid solutions
4. **Existing test files** - Update imports, keep test logic

## Migration Checklist

- [ ] Create `ksg/` directory structure
- [ ] Create `triangular/` directory structure
- [ ] Migrate unweighted KSG gadgets to `ksg/gadgets.rs`
- [ ] Create weighted KSG gadgets in `ksg/gadgets_weighted.rs`
- [ ] Rename triangular gadgets with `WeightedTri` prefix
- [ ] Split mapping functions into respective modules
- [ ] Update `mod.rs` exports
- [ ] Update all test imports
- [ ] Update documentation
- [ ] Delete old files
- [ ] Post Julia comparison to issue #8
- [ ] Run full test suite
- [ ] Run clippy
