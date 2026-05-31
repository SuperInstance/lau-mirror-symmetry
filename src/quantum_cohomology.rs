//! Quantum cohomology: the deformed cup product with GW invariants.
//!
//! The quantum cohomology ring QH*(X) is a deformation of the ordinary
//! cohomology ring H*(X) where the product incorporates Gromov-Witten invariants.
//! For a Calabi-Yau 3-fold with h^{1,1} = 1, this is:
//!
//! QH*(X) = ℂ[x, q] / (W(x, q))
//!
//! where W is the quantum period / Landau-Ginzburg potential.

use serde::{Deserialize, Serialize};
use crate::gromov_witten::GWTable;

/// An element of the quantum cohomology ring.
/// Represented as a polynomial in the generators x₁, ..., xₖ and the parameter q.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QHElement {
    /// Coefficients indexed by monomial degree in (x, q).
    /// For a single generator: terms[d] is the coefficient of x^d * q^e for
    /// various e values.
    /// Simplified: we use a flat vector where terms[i] corresponds to degree i.
    pub terms: Vec<f64>,
}

impl QHElement {
    pub fn zero() -> Self {
        Self { terms: vec![0.0] }
    }

    pub fn constant(c: f64) -> Self {
        Self { terms: vec![c] }
    }

    pub fn variable(degree: usize) -> Self {
        let mut terms = vec![0.0; degree + 1];
        terms[degree] = 1.0;
        Self { terms }
    }

    /// Add two elements.
    pub fn add(&self, other: &QHElement) -> QHElement {
        let max_len = self.terms.len().max(other.terms.len());
        let mut result = vec![0.0; max_len];
        for (i, &t) in self.terms.iter().enumerate() {
            result[i] += t;
        }
        for (i, &t) in other.terms.iter().enumerate() {
            result[i] += t;
        }
        QHElement { terms: result }
    }

    /// Multiply two elements (classical part only, no quantum corrections).
    pub fn classical_multiply(&self, other: &QHElement) -> QHElement {
        let n = self.terms.len() + other.terms.len() - 1;
        let mut result = vec![0.0; n];
        for (i, &a) in self.terms.iter().enumerate() {
            for (j, &b) in other.terms.iter().enumerate() {
                result[i + j] += a * b;
            }
        }
        QHElement { terms: result }
    }
}

/// The quantum cohomology ring for a Calabi-Yau manifold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCohomologyRing {
    /// Name of the manifold
    pub name: String,
    /// Dimension of H^{1,1} (number of Kähler moduli)
    pub h11: usize,
    /// Classical ring dimension
    pub classical_dim: usize,
    /// GW invariants table
    pub gw_table: Option<GWTable>,
    /// The quantum parameter q (as a formal variable we track degree)
    pub q_expansion_order: usize,
}

impl QuantumCohomologyRing {
    /// Create the quantum cohomology ring for the quintic.
    ///
    /// QH*(Q) = ℂ[x, q] / (x⁵ + 5·q·∏_{d=1}^∞ (1 + d·q)^5)
    ///
    /// At degree 0, this reduces to the classical relation x⁵ = 0 (in the appropriate grading).
    pub fn quintic() -> Self {
        let gw_table = GWTable::quintic_table();
        Self {
            name: "Quintic".to_string(),
            h11: 1,
            classical_dim: 4, // H*(ℙ⁴) has dimension 5, but we quotient
            gw_table: Some(gw_table),
            q_expansion_order: 10,
        }
    }

    /// Create a general quantum cohomology ring.
    pub fn new(name: &str, h11: usize, classical_dim: usize) -> Self {
        Self {
            name: name.to_string(),
            h11,
            classical_dim,
            gw_table: None,
            q_expansion_order: 10,
        }
    }

    /// Compute the quantum product of two cohomology classes.
    ///
    /// (a ∪_q b) = a ∪ b + Σ_{β≠0} ⟨a, b, ·⟩_{0,β} q^β
    ///
    /// where the sum is over curve classes β and the correlator uses GW invariants.
    pub fn quantum_product(&self, a: &QHElement, b: &QHElement) -> QHElement {
        // Start with classical product
        let classical = a.classical_multiply(b);

        // Add quantum corrections from GW invariants
        if let Some(ref _gw) = self.gw_table {
            let quantum_correction = QHElement::zero();
            // For the quintic, the quantum correction at degree d contributes
            // to the relation x⁴ + ... = 0
            // Simplified: we just add the classical product for now
            // A full implementation would compute 3-point correlators
            classical.add(&quantum_correction)
        } else {
            classical
        }
    }

    /// Verify associativity of the quantum product.
    /// (a ∪_q b) ∪_q c = a ∪_q (b ∪_q c)
    pub fn verify_associativity(&self, a: &QHElement, b: &QHElement, c: &QHElement) -> bool {
        let ab = self.quantum_product(a, b);
        let ab_c = self.quantum_product(&ab, c);

        let bc = self.quantum_product(b, c);
        let a_bc = self.quantum_product(a, &bc);

        // Check equality with tolerance for floating point
        let max_len = ab_c.terms.len().max(a_bc.terms.len());
        for i in 0..max_len {
            let t1 = ab_c.terms.get(i).copied().unwrap_or(0.0);
            let t2 = a_bc.terms.get(i).copied().unwrap_or(0.0);
            if (t1 - t2).abs() > 1e-10 {
                return false;
            }
        }
        true
    }

    /// The classical cohomology ring for ℙⁿ: ℂ[x]/(x^{n+1}).
    pub fn projective_space(n: usize) -> Self {
        Self::new(&format!("P{}", n), 1, n + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qh_element_add() {
        let a = QHElement { terms: vec![1.0, 2.0, 3.0] };
        let b = QHElement { terms: vec![4.0, 5.0] };
        let c = a.add(&b);
        assert_eq!(c.terms, vec![5.0, 7.0, 3.0]);
    }

    #[test]
    fn test_qh_element_classical_multiply() {
        let a = QHElement { terms: vec![1.0, 1.0] }; // 1 + x
        let b = QHElement { terms: vec![1.0, 1.0] }; // 1 + x
        let c = a.classical_multiply(&b); // (1+x)² = 1 + 2x + x²
        assert_eq!(c.terms, vec![1.0, 2.0, 1.0]);
    }

    #[test]
    fn test_quantum_product_associativity() {
        let ring = QuantumCohomologyRing::quintic();
        let a = QHElement { terms: vec![0.0, 1.0] }; // x
        let b = QHElement { terms: vec![0.0, 0.0, 1.0] }; // x²
        let c = QHElement { terms: vec![0.0, 0.0, 0.0, 1.0] }; // x³
        assert!(ring.verify_associativity(&a, &b, &c));
    }

    #[test]
    fn test_quantum_product_associativity_simple() {
        let ring = QuantumCohomologyRing::projective_space(4);
        let a = QHElement::variable(1);
        let b = QHElement::variable(1);
        let c = QHElement::variable(1);
        assert!(ring.verify_associativity(&a, &b, &c));
    }

    #[test]
    fn test_classical_ring_dimension() {
        let ring = QuantumCohomologyRing::projective_space(3);
        assert_eq!(ring.classical_dim, 4);
    }

    #[test]
    fn test_quintic_ring_h11() {
        let ring = QuantumCohomologyRing::quintic();
        assert_eq!(ring.h11, 1);
    }

    #[test]
    fn test_qh_zero_element() {
        let z = QHElement::zero();
        assert_eq!(z.terms, vec![0.0]);
    }

    #[test]
    fn test_qh_constant_element() {
        let c = QHElement::constant(42.0);
        assert_eq!(c.terms, vec![42.0]);
    }

    #[test]
    fn test_multiply_by_zero() {
        let z = QHElement::zero();
        let a = QHElement { terms: vec![1.0, 2.0, 3.0] };
        let p = z.classical_multiply(&a);
        assert!(p.terms.iter().all(|&t| t.abs() < 1e-10));
    }
}
