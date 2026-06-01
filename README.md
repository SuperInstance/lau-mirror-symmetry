# lau-mirror-symmetry

**Mirror symmetry — the duality between symplectic and complex geometry: Calabi-Yau manifolds, Hodge diamonds, Gromov-Witten invariants, quantum cohomology, mirror maps, Picard-Fuchs equations, and homological mirror symmetry.**

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

64 tests · 1,855 lines of Rust · 8 modules

---

## What This Does

Mirror symmetry is one of the deepest dualities in modern mathematics. It asserts that for every Calabi-Yau manifold X, there exists a "mirror" X̃ where counting curves on X equals computing periods on X̃ — two completely different geometric calculations that produce the same numbers. This crate implements the mathematical structures underlying this correspondence.

You get:
- **Calabi-Yau manifolds** with Ricci-flat Kähler metrics and SU(n) holonomy verification
- **Hodge diamonds** with conjugation symmetry, Serre duality, and Hard Lefschetz verification
- **Gromov-Witten invariants** counting pseudo-holomorphic curves (including the famous quintic: 2875, 609250, 317206375…)
- **Quantum cohomology** with deformed cup products and associativity verification
- **Mirror maps** between A-model (symplectic) and B-model (complex) geometries
- **Picard-Fuchs equations** governing period integrals with monodromy analysis
- **Homological mirror symmetry** — Fukaya categories ↔ derived categories
- **Agent dualities** — two agent architectures computing the same invariants, mirroring the A/B-model correspondence

---

## Key Idea

Mirror symmetry exchanges two geometric worlds:

| A-Model (Symplectic) | B-Model (Complex) |
|---|---|
| Count curves (Gromov-Witten) | Compute periods (Picard-Fuchs) |
| Quantum cohomology ring | Variation of Hodge structure |
| Fukaya category | Derived category of coherent sheaves |
| Kähler moduli (h^{1,1}) | Complex structure moduli (h^{2,1}) |
| Combinatorial/enumerative | Analytic/transcendental |

The mirror map translates between them: the same physical theory has two geometric realizations, and computing on either side gives identical answers.

---

## Install

```toml
[dependencies]
lau-mirror-symmetry = "0.1"
```

Requires Rust 2021 edition. Dependencies: `serde`, `nalgebra`.

---

## Quick Start

```rust
use lau_mirror_symmetry::{CalabiYauManifold, HodgeDiamond, GromovWittenInvariant, MirrorMap};

// The quintic Calabi-Yau threefold in P^4
let quintic = CalabiYauManifold::quintic(); // h^{1,1} = 1, h^{2,1} = 101, χ = -200
assert!(quintic.verify_calabi_yau());

// Hodge diamond: verify all symmetries
let diamond = HodgeDiamond::cy3(1, 101);
assert!(diamond.verify_conjugation());     // h^{p,q} = h^{q,p}
assert!(diamond.verify_serre_duality());   // h^{p,q} = h^{n-p,n-q}
assert!(diamond.verify_hard_lefschetz());

// Gromov-Witten invariants: counts of rational curves
let n1 = GromovWittenInvariant::quintic_genus_zero(1).unwrap();
assert_eq!(n1.value, 2875);  // lines on the quintic
let n2 = GromovWittenInvariant::quintic_genus_zero(2).unwrap();
assert_eq!(n2.value, 609250); // conics
let n3 = GromovWittenInvariant::quintic_genus_zero(3).unwrap();
assert_eq!(n3.value, 317206375); // cubics

// Mirror: swap h^{1,1} ↔ h^{2,1}
let mirror = quintic.mirror().unwrap();
assert_eq!(mirror.euler_characteristic, 200); // sign flip

// Mirror map between the pair
let mm = MirrorMap::quintic();
assert_eq!(mm.kahler_moduli, 1);    // A-model has 1 Kähler modulus
assert_eq!(mm.complex_moduli, 101); // B-model has 101 complex structure moduli
assert!(mm.verify_involution());
```

---

## API Reference

### `CalabiYauManifold`

A Ricci-flat Kähler manifold with SU(n) holonomy. Constructed from Hodge numbers with full CY condition verification.

```rust
let q = CalabiYauManifold::quintic();            // h^{1,1}=1, h^{2,1}=101
let mq = CalabiYauManifold::mirror_quintic();     // h^{1,1}=101, h^{2,1}=1
let k3 = CalabiYauManifold::k3_surface();         // χ=24
let ec = CalabiYauManifold::elliptic_curve();     // χ=0
let custom = CalabiYauManifold::cy3(5, 20, "Custom"); // arbitrary CY3

// Verification
q.verify_calabi_yau();            // all conditions
q.verify_su_n_holonomy();         // h^{p,0}=0 for 0<p<n, h^{n,0}=1
q.verify_serre_duality();         // h^{p,q}=h^{n-p,n-q}
q.verify_conjugation_symmetry();  // h^{p,q}=h^{q,p}
```

Pre-built manifolds: `quintic()`, `mirror_quintic()`, `k3_surface()`, `elliptic_curve()`, `cy3(h11, h21, name)`.

### `HodgeDiamond`

The complete Hodge diamond for a compact Kähler manifold, encoding the Hodge decomposition H^k(X, ℂ) = ⊕_{p+q=k} H^{p,q}(X).

```rust
let d = HodgeDiamond::cy3(1, 101);
d.get(1, 1);                      // h^{1,1} = 1
d.get(2, 1);                      // h^{2,1} = 101
d.euler_characteristic();          // χ = -200
d.verify_hard_lefschetz();         // b_k = b_{2n-k}
d.mirror();                        // swap h^{1,1} ↔ h^{2,1}
println!("{}", d.display());       // formatted diamond
```

### `GromovWittenInvariant`

Counts of stable maps ƒ: C → X from genus-g curves, organized by curve class β ∈ H₂(X, ℤ).

```rust
// The famous quintic invariants
let n1 = GromovWittenInvariant::quintic_genus_zero(1).unwrap();
assert_eq!(n1.value, 2875);

// Table of invariants
let table = GWTable::quintic_table();
table.get(0, 3);  // Some(317206375)
table.verify_wdvv(); // WDVV associativity check

// Multiple cover formula
let covered = GromovWittenInvariant::multiple_cover_formula(&n1, 2);
```

Curve classes β ∈ H₂(X, ℤ) support addition, scaling, and degree computation.

### `QuantumCohomologyRing`

The deformed cohomology ring QH*(X) where the cup product incorporates GW corrections.

```rust
let ring = QuantumCohomologyRing::quintic();
// QH*(Q) = C[x,q] / (quantum relation)

let a = QHElement::variable(1); // x
let b = QHElement::variable(2); // x²
let c = QHElement::variable(3); // x³

// Quantum product (classical + GW corrections)
let prod = ring.quantum_product(&a, &b);

// Associativity: (a ∪_q b) ∪_q c = a ∪_q (b ∪_q c)
assert!(ring.verify_associativity(&a, &b, &c));
```

Elements are polynomials in generators x₁,…,xₖ and the quantum parameter q.

### `MirrorMap`

The transformation between A-model Kähler parameters and B-model complex structure parameters.

```rust
let mm = MirrorMap::quintic();

// A-model parameter t from B-model parameter z
let t = mm.a_model_from_b_model(0.1, 3); // series expansion

// B-model parameter from A-model parameter
let z = mm.b_model_from_a_model(0.01, 2);

// Instanton numbers via the mirror theorem
mm.instanton_number(1); // Some(2875)
mm.instanton_number(2); // Some(609250)

// Yukawa coupling: κ_{ttt} = Σ_d N_d · d³ · q^d / (1-q^d)
let kappa = mm.yukawa_coupling(0.5, 3);
```

### `PicardFuchsEquation`

Linear ODE satisfied by the period integrals of a CY family. For the mirror quintic: θ⁴ - 5z·∏(5θ+k)/5⁴ = 0.

```rust
let pf = PicardFuchsEquation::quintic(); // 4th order, maximally unipotent

// Fundamental period ω₀(z) = Σ (5n)!/(n!⁵) zⁿ
let omega = pf.fundamental_period(0.01, 100);

// Mirror map t(z) = ω₁(z)/ω₀(z)
let t = pf.mirror_map(0.01, 100);

// Monodromy analysis
assert!(pf.is_maximally_unipotent()); // all indicial roots = 0
assert!(pf.verify_monodromy_det_1()); // det(M) = 1

let m = pf.monodromy_matrix(); // Jordan block with 1s on diagonal
```

### `FukayaCategory` / `DerivedCategory`

Homological mirror symmetry (Kontsevich): D^π Fuk(X) ≅ D^b Coh(X̃).

```rust
// A-model: Lagrangian submanifolds with Floer cohomology
let mut fuk = FukayaCategory::new("Quintic (A-model)");
fuk.add_lagrangian("Real Quintic", vec![1], 0);
fuk.floer_cohomology(0, 0); // HF*(L,L) = H*(L)

// B-model: Coherent sheaves with Ext groups
let mut der = DerivedCategory::new("Mirror Quintic (B-model)");
der.add_sheaf("O", vec![1, 0, 0, 0], 1);
der.ext_groups(0, 0); // Ext*(O,O) = H*(X)

// Standard collection for projective space
der.add_projective_space_collection(4); // O, O(1), ..., O(4)

// Full HMS duality
let hms = HMSDuality::quintic();
assert!(hms.verify_euler_match());
assert!(hms.verify_bijection());
```

### `AgentDuality`

Applies mirror symmetry as a metaphor for agent architectures: two architectures computing the same invariants through different computational frameworks.

```rust
let duality = AgentDuality::quintic();
assert!(duality.verify_full_duality());
assert!(duality.verify_invariant_match());    // "GW invariants" match
assert!(duality.verify_capability_mirror());   // Hodge numbers are swapped

let custom = AgentDuality::cy_duality("Arch A", "Arch B", 3, 7);
let correction = custom.quantum_correction(1); // "quantum" performance correction
```

---

## How It Works

The crate is structured as the A/B-model correspondence:

```
A-Model (Symplectic)                  B-Model (Complex)
─────────────────────                 ──────────────────
CalabiYauManifold ──mirror──→ CalabiYauManifold (h^{1,1} ↔ h^{2,1})
       │                                     │
       ▼                                     ▼
GromovWittenInvariant               PicardFuchsEquation
(curve counting)                    (period integrals)
       │                                     │
       ▼                                     ▼
QuantumCohomologyRing  ──map──→   MirrorMap
(deformed product)                  (A↔B parameter transform)
       │                                     │
       ▼                                     ▼
FukayaCategory       ──HMS──→     DerivedCategory
(Lagrangians + Floer)              (Sheaves + Ext groups)
```

**Top level:** A Calabi-Yau 3-fold X with Hodge numbers (h^{1,1}, h^{2,1}) has a mirror X̃ with numbers swapped: (h^{2,1}, h^{1,1}). Euler characteristic flips sign.

**A-model side:** Counts curves via Gromov-Witten invariants N_{g,β}. The quantum cohomology ring QH*(X) deforms the classical cup product using these invariants. The WDVV equations ensure associativity.

**B-model side:** Periods of holomorphic forms satisfy Picard-Fuchs differential equations. The monodromy of these equations around singular loci encodes the mirror map.

**Bottom level:** Kontsevich's HMS conjecture equates the derived Fukaya category (symplectic side) with the bounded derived category of coherent sheaves (complex side).

---

## The Math

### Calabi-Yau Manifolds

A **Calabi-Yau n-fold** is a compact Kähler manifold with c₁ = 0. By Yau's theorem it admits a Ricci-flat metric. The holonomy is contained in SU(n).

For a CY 3-fold, the Hodge diamond is determined by two numbers:

```
              1
          0       0
     0      h^{1,1}      0
 1      h^{2,1}     h^{2,1}     1
     0      h^{1,1}      0
          0       0
              1
```

The **mirror** swaps h^{1,1} ↔ h^{2,1}, flipping χ = 2(h^{1,1} − h^{2,1}).

### Gromov-Witten Invariants

The genus-0, degree-d GW invariant N_{0,d} counts (virtually) the number of rational curves of degree d in X. For the quintic:

| d | N_{0,d} |
|---|---------|
| 1 | 2,875 |
| 2 | 609,250 |
| 3 | 317,206,375 |

These satisfy the **WDVV equations** (associativity of the quantum product) and are related by the **multiple cover formula**: N_{0,dβ} involves a d^{-3} factor.

### Quantum Cohomology

QH*(X) deforms the classical cup product:

**(a ∪_q b) = a ∪ b + Σ_{β≠0} ⟨a, b, ·⟩_{0,β} q^β**

The deformation parameter q tracks the curve class β. For the quintic: QH*(Q) = ℂ[x, q] / (x⁵ + 5·q·∏(1+dq)^5).

### Mirror Map and Picard-Fuchs

The **mirror map** q = exp(2πi·t) relates Kähler parameter t to complex structure parameter z via periods of the mirror manifold. The periods satisfy **Picard-Fuchs equations** — linear ODEs whose solutions encode all mirror symmetry predictions.

For the quintic mirror: θ⁴ − 5z·(5θ+1)(5θ+2)(5θ+3)(5θ+4)/5⁴ = 0, with maximally unipotent monodromy (all indicial roots are 0).

### Homological Mirror Symmetry

Kontsevich's HMS conjecture (1994): D^π Fuk(X) ≅ D^b Coh(X̃). This upgrades numerical mirror symmetry to a categorical equivalence — the entire Fukaya category of Lagrangian submanifolds on the symplectic side matches the derived category of coherent sheaves on the complex side.

---

## Module Overview

| Module | Tests | Key Types | Purpose |
|--------|-------|-----------|---------|
| `calabi_yau` | 11 | `CalabiYauManifold` | CY manifolds with Hodge numbers and condition verification |
| `hodge` | 5 | `HodgeDiamond`, `HodgeNumber` | Hodge diamonds with symmetry checks |
| `gromov_witten` | 8 | `GromovWittenInvariant`, `CurveClass`, `GWTable` | Curve counting invariants |
| `quantum_cohomology` | 9 | `QuantumCohomologyRing`, `QHElement` | Deformed cup product |
| `mirror_map` | 8 | `MirrorMap`, `MirrorSide` | A↔B parameter transformation |
| `picard_fuchs` | 8 | `PicardFuchsEquation` | Period ODEs and monodromy |
| `homological` | 7 | `FukayaCategory`, `DerivedCategory`, `HMSDuality` | Categorical mirror symmetry |
| `agent_duality` | 8 | `AgentDuality`, `AgentArchitecture` | Agent architecture analogy |

---

## References

- **Mirror Symmetry:** Cox & Katz, *Mirror Symmetry and Algebraic Geometry* (1999)
- **Calabi-Yau Manifolds:** Hübsch, *Calabi-Yau Manifolds: A Bestiary for Physicists* (1992)
- **Gromov-Witten Theory:** Kontsevich & Manin, *Gromov-Witten classes, quantum cohomology, and enumerative geometry* (1994)
- **Homological Mirror Symmetry:** Kontsevich, *Homological algebra of mirror symmetry* (1994)
- **Picard-Fuchs:** Candelas, de la Ossa, Green & Parkes, *A pair of Calabi-Yau manifolds as an exactly soluble superconformal theory* (1991)

---

## License

MIT
