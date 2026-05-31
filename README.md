# lau-mirror-symmetry

Mirror symmetry is a duality between two seemingly different geometric worlds: the symplectic (A-model) and the complex (B-model). Calabi-Yau manifolds come in mirror pairs (X, X̃) where counting curves on X equals computing periods on X̃. It's one of the deepest surprises in modern mathematics.

## The math in 60 seconds

A **Calabi-Yau manifold** has vanishing first Chern class (Ricci-flat Kähler). For a CY 3-fold, the Hodge diamond has h^{1,1} and h^{2,1} as the key invariants. **Mirror symmetry** exchanges them: the mirror X̃ has h^{1,1}(X̃) = h^{2,1}(X) and vice versa.

Key structures this crate implements:

- **Gromov-Witten invariants:** counts of rational curves in X (the quintic: 2875, 609250, 317206375...)
- **Quantum cohomology:** a deformed cup product QH*(X) with GW corrections
- **Mirror map:** transforms A-model parameters ↔ B-model parameters
- **Picard-Fuchs equations:** differential equations governing the periods
- **Homological mirror symmetry:** Fukaya category (X) ≅ Derived category (X̃)

References: Cox & Katz, *Mirror Symmetry and Algebraic Geometry* (1999)

## Quick start

```rust
use lau_mirror_symmetry::{CalabiYau, HodgeDiamond, GromovWitten, MirrorMap};

// Quintic threefold (the classic example)
let quintic = CalabiYau::quintic();

// Hodge diamond
let diamond = HodgeDiamond::from_manifold(&quintic);
//          1
//      0       0
//  0       1       0
//      0       0       (mirror swaps h^{1,1}=1 ↔ h^{2,1}=101)
//          1

// Gromov-Witten invariants (degree 1 through 3)
let gw = GromovWitten::compute(&quintic, 3);
assert_eq!(gw.degree(1), 2875);
assert_eq!(gw.degree(2), 609250);

// Mirror quintic
let mirror = quintic.mirror();
let mirror_map = MirrorMap::between(&quintic, &mirror);
```

## Key types

| Type | What it is |
|------|-----------|
| `CalabiYau` | A Ricci-flat Kähler manifold with SU(n) holonomy |
| `HodgeDiamond` | The H^{p,q} numbers with conjugation symmetry and mirror swap |
| `GromovWitten` | Counts of rational curves, organized by degree |
| `QuantumCohomology` | Deformed cup product with WDVV associativity |
| `MirrorMap` | The A-model ↔ B-model parameter transformation |
| `PicardFuchs` | Differential equations for period integrals |

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-mirror-symmetry/issues) or PR.
