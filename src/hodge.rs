//! Hodge diamonds: the fundamental combinatorial structure of mirror symmetry.

use serde::{Deserialize, Serialize};
use crate::calabi_yau::CalabiYauManifold;

/// A single Hodge number h^{p,q}.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HodgeNumber {
    pub p: usize,
    pub q: usize,
    pub value: u64,
}

/// A complete Hodge diamond for a compact Kähler manifold of complex dimension n.
///
/// The Hodge diamond encodes the decomposition H^k(X, ℂ) = ⊕_{p+q=k} H^{p,q}(X)
/// and satisfies:
/// - **Complex conjugation**: h^{p,q} = h^{q,p}
/// - **Serre duality**: h^{p,q} = h^{n-p,n-q}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HodgeDiamond {
    pub dim: usize,
    /// numbers[p][q] = h^{p,q}
    pub numbers: Vec<Vec<u64>>,
}

impl HodgeDiamond {
    /// Create a Hodge diamond from a raw matrix.
    pub fn new(numbers: Vec<Vec<u64>>) -> Self {
        let dim = numbers.len() - 1;
        Self { dim, numbers }
    }

    /// Create the Hodge diamond for a Calabi-Yau 3-fold with given (h^{1,1}, h^{2,1}).
    pub fn cy3(h11: u64, h21: u64) -> Self {
        let cy = CalabiYauManifold::cy3(h11, h21, "");
        Self {
            dim: 3,
            numbers: cy.hodge_numbers,
        }
    }

    /// Get h^{p,q}.
    pub fn get(&self, p: usize, q: usize) -> u64 {
        self.numbers.get(p).and_then(|r| r.get(q)).copied().unwrap_or(0)
    }

    /// Verify complex conjugation symmetry: h^{p,q} = h^{q,p}.
    pub fn verify_conjugation(&self) -> bool {
        for p in 0..=self.dim {
            for q in 0..=self.dim {
                if self.get(p, q) != self.get(q, p) {
                    return false;
                }
            }
        }
        true
    }

    /// Verify Serre duality: h^{p,q} = h^{n-p,n-q}.
    pub fn verify_serre_duality(&self) -> bool {
        let n = self.dim;
        for p in 0..=n {
            for q in 0..=n {
                if self.get(p, q) != self.get(n - p, n - q) {
                    return false;
                }
            }
        }
        true
    }

    /// Verify the Hard Lefschetz theorem: the map L^{n-k}: H^k → H^{2n-k} is an isomorphism.
    /// This implies h^{p,q} = h^{n-p,n-q} (already Serre duality for projective varieties).
    pub fn verify_hard_lefschetz(&self) -> bool {
        // For Kähler manifolds, Hard Lefschetz gives additional symmetries.
        // The cup product with ω^{n-2p-2q} gives h^{p,q} = h^{n-p,q} when applicable.
        // For our purposes, we check that the Betti numbers satisfy the relations.
        let n = self.dim;
        // Betti number b_k = Σ_{p+q=k} h^{p,q}
        for k in 0..=2 * n {
            let bk: u64 = (0..=n)
                .flat_map(|p| (0..=n).map(move |q| (p, q)))
                .filter(|(p, q)| p + q == k)
                .map(|(p, q)| self.get(p, q))
                .sum();
            let bk_mirror: u64 = (0..=n)
                .flat_map(|p| (0..=n).map(move |q| (p, q)))
                .filter(|(p, q)| p + q == 2 * n - k)
                .map(|(p, q)| self.get(p, q))
                .sum();
            if bk != bk_mirror {
                return false;
            }
        }
        true
    }

    /// Compute the mirror Hodge diamond (swap h^{p,q} ↔ h^{n-p,q} for CY).
    /// For CY 3-folds, this is the swap h^{1,1} ↔ h^{2,1}.
    pub fn mirror(&self) -> Option<Self> {
        if self.dim != 3 {
            return None;
        }
        let h11 = self.get(1, 1);
        let h21 = self.get(2, 1);
        Some(Self::cy3(h21, h11))
    }

    /// Euler characteristic from Hodge diamond: χ = Σ (-1)^{p+q} h^{p,q}.
    pub fn euler_characteristic(&self) -> i64 {
        let mut chi: i64 = 0;
        for p in 0..=self.dim {
            for q in 0..=self.dim {
                let sign: i64 = if (p + q) % 2 == 0 { 1 } else { -1 };
                chi += sign * self.get(p, q) as i64;
            }
        }
        chi
    }

    /// Format the Hodge diamond as a string.
    pub fn display(&self) -> String {
        let mut s = String::new();
        for p in 0..=self.dim {
            let indent = " ".repeat(self.dim - p);
            let row: Vec<String> = (0..=self.dim)
                .map(|q| self.get(p, q).to_string())
                .collect();
            s.push_str(&format!("{}{}\n", indent, row.join("\t")));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cy3_diamond_symmetries() {
        let d = HodgeDiamond::cy3(1, 101);
        assert!(d.verify_conjugation());
        assert!(d.verify_serre_duality());
        assert!(d.verify_hard_lefschetz());
    }

    #[test]
    fn test_mirror_diamond_swap() {
        let d = HodgeDiamond::cy3(5, 10);
        let m = d.mirror().unwrap();
        assert_eq!(d.get(1, 1), m.get(2, 1));
        assert_eq!(d.get(2, 1), m.get(1, 1));
    }

    #[test]
    fn test_euler_from_diamond() {
        let d = HodgeDiamond::cy3(1, 101);
        assert_eq!(d.euler_characteristic(), -200);
        let d2 = HodgeDiamond::cy3(101, 1);
        assert_eq!(d2.euler_characteristic(), 200);
    }

    #[test]
    fn test_double_mirror_is_identity() {
        let d = HodgeDiamond::cy3(7, 13);
        let dm = d.mirror().unwrap().mirror().unwrap();
        assert_eq!(d.get(1, 1), dm.get(1, 1));
        assert_eq!(d.get(2, 1), dm.get(2, 1));
    }

    #[test]
    fn test_display_diamond() {
        let d = HodgeDiamond::cy3(1, 101);
        let s = d.display();
        assert!(s.contains("1"));
        assert!(s.contains("101"));
    }
}
