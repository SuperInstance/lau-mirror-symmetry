//! Calabi-Yau manifolds: Ricci-flat Kähler manifolds with SU(n) holonomy.

use serde::{Deserialize, Serialize};

/// A Calabi-Yau n-fold.
///
/// A Calabi-Yau manifold is a compact Kähler manifold with vanishing first
/// Chern class (c₁ = 0). By Yau's theorem, it admits a Ricci-flat metric.
/// The holonomy group is contained in SU(n) where n is the complex dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalabiYauManifold {
    /// Complex dimension (n-fold)
    pub complex_dim: usize,
    /// Hodge numbers h^{p,q} for p,q = 0..=n
    pub hodge_numbers: Vec<Vec<u64>>,
    /// Euler characteristic χ = Σ_{p,q} (-1)^{p+q} h^{p,q}
    pub euler_characteristic: i64,
    /// Whether SU(n) holonomy is strict (vs. a proper subgroup)
    pub strict_holonomy_su_n: bool,
    /// Name/label for this manifold (e.g. "Quintic")
    pub name: String,
}

impl CalabiYauManifold {
    /// Create a Calabi-Yau 3-fold from its Hodge numbers h^{1,1} and h^{2,1}.
    ///
    /// For a CY 3-fold, the Hodge diamond is:
    /// ```text
    ///            1
    ///        0       0
    ///    0      h^{1,1}     0
    /// 1    h^{2,1}   h^{1,1}    1
    ///    0      h^{1,1}     0
    ///        0       0
    ///            1
    /// ```
    ///
    /// Wait, the correct diamond for a CY 3-fold is:
    /// ```text
    ///              1
    ///          0       0
    ///     0      h^{1,1}      0
    /// 1     h^{2,1}     h^{2,1}     1
    ///     0      h^{1,1}      0
    ///          0       0
    ///              1
    /// ```
    pub fn cy3(h11: u64, h21: u64, name: &str) -> Self {
        // Euler characteristic: χ = 2(h^{1,1} - h^{2,1})
        let euler = 2 * (h11 as i64 - h21 as i64);

        // Full Hodge diamond for CY 3-fold (row p, col q)
        // h^{p,q} matrix indexed [p][q]
        let hodge_numbers = vec![
            vec![1, 0, 0, 1],        // p=0
            vec![0, h11, h21, 0],     // p=1
            vec![0, h21, h11, 0],     // p=2 (Serre duality)
            vec![1, 0, 0, 1],         // p=3
        ];

        Self {
            complex_dim: 3,
            hodge_numbers,
            euler_characteristic: euler,
            strict_holonomy_su_n: true,
            name: name.to_string(),
        }
    }

    /// The quintic Calabi-Yau threefold in CP⁴.
    /// h^{1,1} = 1, h^{2,1} = 101, χ = -200.
    pub fn quintic() -> Self {
        Self::cy3(1, 101, "Quintic")
    }

    /// The mirror quintic.
    /// h^{1,1} = 101, h^{2,1} = 1, χ = 200.
    pub fn mirror_quintic() -> Self {
        Self::cy3(101, 1, "Mirror Quintic")
    }

    /// A Calabi-Yau 1-fold (elliptic curve / torus T²).
    /// h^{1,0} = 1, h^{0,1} = 1, χ = 0.
    pub fn elliptic_curve() -> Self {
        let hodge_numbers = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        Self {
            complex_dim: 1,
            hodge_numbers,
            euler_characteristic: 0,
            strict_holonomy_su_n: true,
            name: "Elliptic Curve".to_string(),
        }
    }

    /// A K3 surface (Calabi-Yau 2-fold).
    /// h^{0,0}=1, h^{1,0}=0, h^{0,1}=0, h^{1,1}=20, h^{2,0}=1, h^{0,2}=1
    /// Euler characteristic = 24.
    pub fn k3_surface() -> Self {
        let hodge_numbers = vec![
            vec![1, 0, 1],
            vec![0, 20, 0],
            vec![1, 0, 1],
        ];
        Self {
            complex_dim: 2,
            hodge_numbers,
            euler_characteristic: 24,
            strict_holonomy_su_n: true,
            name: "K3 Surface".to_string(),
        }
    }

    /// Verify that c₁ = 0 (vanishing first Chern class).
    /// For a Calabi-Yau, this is encoded in the Hodge diamond symmetry.
    pub fn verify_vanishing_first_chern_class(&self) -> bool {
        // For CY n-fold: h^{n,0} = 1 (unique holomorphic volume form)
        // and h^{0,0} = 1
        let n = self.complex_dim;
        if n + 1 > self.hodge_numbers.len() {
            return false;
        }
        self.hodge_numbers[0][0] == 1 && self.hodge_numbers[n][0] == 1
    }

    /// Verify the Kähler condition: h^{p,0} = 0 for 0 < p < n.
    pub fn verify_kahler_condition(&self) -> bool {
        let n = self.complex_dim;
        for p in 1..n {
            if self.hodge_numbers[p][0] != 0 {
                return false;
            }
        }
        true
    }

    /// Verify SU(n) holonomy conditions:
    /// - h^{0,0} = 1
    /// - h^{p,0} = 0 for 0 < p < n
    /// - h^{n,0} = 1
    pub fn verify_su_n_holonomy(&self) -> bool {
        let n = self.complex_dim;
        self.hodge_numbers[0][0] == 1
            && self.hodge_numbers[n][0] == 1
            && self.verify_kahler_condition()
    }

    /// Verify Ricci-flatness (encoded as c₁ = 0 in cohomology).
    pub fn verify_ricci_flat(&self) -> bool {
        self.verify_vanishing_first_chern_class()
    }

    /// Verify all Calabi-Yau conditions.
    pub fn verify_calabi_yau(&self) -> bool {
        self.verify_ricci_flat()
            && self.verify_kahler_condition()
            && self.verify_su_n_holonomy()
    }

    /// Get h^{p,q}
    pub fn hodge(&self, p: usize, q: usize) -> Option<u64> {
        self.hodge_numbers.get(p).and_then(|row| row.get(q)).copied()
    }

    /// For a CY 3-fold, return (h^{1,1}, h^{2,1}).
    pub fn h11_h21(&self) -> Option<(u64, u64)> {
        if self.complex_dim == 3 {
            Some((
                self.hodge_numbers[1][1],
                self.hodge_numbers[2][1],
            ))
        } else {
            None
        }
    }

    /// Compute the mirror of this CY 3-fold: swap h^{1,1} ↔ h^{2,1}.
    pub fn mirror(&self) -> Option<Self> {
        if self.complex_dim != 3 {
            return None;
        }
        let (h11, h21) = self.h11_h21()?;
        let mirror_name = format!("Mirror({})", self.name);
        Some(Self::cy3(h21, h11, &mirror_name))
    }

    /// Verify Serre duality: h^{p,q} = h^{n-p,n-q}.
    pub fn verify_serre_duality(&self) -> bool {
        let n = self.complex_dim;
        for p in 0..=n {
            for q in 0..=n {
                let hpq = self.hodge(p, q).unwrap_or(0);
                let hnqnp = self.hodge(n - p, n - q).unwrap_or(0);
                if hpq != hnqnp {
                    return false;
                }
            }
        }
        true
    }

    /// Verify complex conjugation symmetry: h^{p,q} = h^{q,p}.
    pub fn verify_conjugation_symmetry(&self) -> bool {
        let n = self.complex_dim;
        for p in 0..=n {
            for q in 0..=n {
                let hpq = self.hodge(p, q).unwrap_or(0);
                let hqp = self.hodge(q, p).unwrap_or(0);
                if hpq != hqp {
                    return false;
                }
            }
        }
        true
    }

    /// Compute the middle cohomology dimension for a CY n-fold.
    /// h^{n,n} = 1 always. The interesting part is h^{n,0} = 1.
    pub fn middle_cohomology_dimension(&self) -> u64 {
        let n = self.complex_dim;
        (0..=n).map(|q| self.hodge(n, q).unwrap_or(0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quintic_hodge_numbers() {
        let q = CalabiYauManifold::quintic();
        assert_eq!(q.hodge(1, 1), Some(1));
        assert_eq!(q.hodge(2, 1), Some(101));
        assert_eq!(q.euler_characteristic, -200);
    }

    #[test]
    fn test_mirror_quintic() {
        let mq = CalabiYauManifold::mirror_quintic();
        assert_eq!(mq.hodge(1, 1), Some(101));
        assert_eq!(mq.hodge(2, 1), Some(1));
        assert_eq!(mq.euler_characteristic, 200);
    }

    #[test]
    fn test_quintic_mirror_swap() {
        let q = CalabiYauManifold::quintic();
        let mq = q.mirror().unwrap();
        assert_eq!(q.hodge(1, 1), mq.hodge(2, 1));
        assert_eq!(q.hodge(2, 1), mq.hodge(1, 1));
        assert_eq!(q.euler_characteristic, -mq.euler_characteristic);
    }

    #[test]
    fn test_quintic_calabi_yau_conditions() {
        let q = CalabiYauManifold::quintic();
        assert!(q.verify_calabi_yau());
        assert!(q.verify_su_n_holonomy());
        assert!(q.verify_ricci_flat());
    }

    #[test]
    fn test_serre_duality_quintic() {
        let q = CalabiYauManifold::quintic();
        assert!(q.verify_serre_duality());
    }

    #[test]
    fn test_conjugation_symmetry_quintic() {
        let q = CalabiYauManifold::quintic();
        assert!(q.verify_conjugation_symmetry());
    }

    #[test]
    fn test_k3_surface() {
        let k3 = CalabiYauManifold::k3_surface();
        assert_eq!(k3.complex_dim, 2);
        assert_eq!(k3.euler_characteristic, 24);
        assert_eq!(k3.hodge(1, 1), Some(20));
        assert!(k3.verify_calabi_yau());
    }

    #[test]
    fn test_elliptic_curve() {
        let ec = CalabiYauManifold::elliptic_curve();
        assert_eq!(ec.complex_dim, 1);
        assert_eq!(ec.euler_characteristic, 0);
        assert!(ec.verify_calabi_yau());
        assert!(ec.verify_serre_duality());
    }

    #[test]
    fn test_mirror_euler_sign_flip() {
        // For CY 3-folds, mirror flips sign of Euler characteristic
        let cy = CalabiYauManifold::cy3(5, 20, "Test");
        let m = cy.mirror().unwrap();
        assert_eq!(cy.euler_characteristic, -30);
        assert_eq!(m.euler_characteristic, 30);
    }

    #[test]
    fn test_k3_self_mirror() {
        // K3 surface is its own mirror in dimension 2
        let k3 = CalabiYauManifold::k3_surface();
        assert!(k3.verify_serre_duality());
        assert!(k3.verify_conjugation_symmetry());
    }

    #[test]
    fn test_custom_cy3() {
        let cy = CalabiYauManifold::cy3(3, 7, "Custom");
        assert_eq!(cy.h11_h21(), Some((3, 7)));
        assert_eq!(cy.euler_characteristic, -8);
        assert!(cy.verify_calabi_yau());
    }
}
