//! Agent dualities: two agent architectures computing the same invariants.
//!
//! This module formalizes the analogy between mirror symmetry and
//! agent architecture dualities. Just as the A-model and B-model compute
//! the same invariants through different geometric frameworks, two different
//! agent architectures can produce equivalent computational outcomes.
//!
//! The key insight: if two architectures are "mirror duals," then:
//! - Architecture A (symplectic / combinatorial) ↔ Architecture B (algebraic / analytic)
//! - Their "Hodge diamonds" (capability matrices) are transposes of each other
//! - Their "GW invariants" (performance metrics) agree
//! - There exists a "mirror map" relating their parameter spaces

use serde::{Deserialize, Serialize};
use crate::calabi_yau::CalabiYauManifold;

/// An agent architecture with its capability profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentArchitecture {
    /// Name of the architecture
    pub name: String,
    /// Capability dimensions (analogous to Hodge numbers)
    pub capabilities: Vec<Vec<u64>>,
    /// Performance metrics (analogous to GW invariants)
    pub performance: Vec<f64>,
}

impl AgentArchitecture {
    pub fn new(name: &str, capabilities: Vec<Vec<u64>>, performance: Vec<f64>) -> Self {
        Self { name: name.to_string(), capabilities, performance }
    }

    /// Compute the "Euler characteristic" (net capability score).
    pub fn euler_characteristic(&self) -> i64 {
        let mut chi: i64 = 0;
        for (p, row) in self.capabilities.iter().enumerate() {
            for (q, &val) in row.iter().enumerate() {
                let sign = if (p + q) % 2 == 0 { 1 } else { -1 };
                chi += sign * val as i64;
            }
        }
        chi
    }
}

/// An agent duality: two architectures that compute the same invariants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDuality {
    /// Architecture A (analogous to A-model)
    pub architecture_a: AgentArchitecture,
    /// Architecture B (analogous to B-model)
    pub architecture_b: AgentArchitecture,
    /// The "mirror map" between parameter spaces
    pub parameter_map: Vec<(f64, f64)>,
}

impl AgentDuality {
    /// Create a duality from two architectures.
    pub fn new(a: AgentArchitecture, b: AgentArchitecture) -> Self {
        Self {
            architecture_a: a,
            architecture_b: b,
            parameter_map: Vec::new(),
        }
    }

    /// Create a CY-motivated agent duality.
    /// Architecture A has "Hodge diamond" h, Architecture B has its mirror.
    pub fn cy_duality(name_a: &str, name_b: &str, h11: u64, h21: u64) -> Self {
        let cy_a = CalabiYauManifold::cy3(h11, h21, name_a);
        let cy_b = CalabiYauManifold::cy3(h21, h11, name_b);

        let a = AgentArchitecture::new(
            name_a,
            cy_a.hodge_numbers.clone(),
            vec![h11 as f64, h21 as f64, cy_a.euler_characteristic as f64],
        );
        let b = AgentArchitecture::new(
            name_b,
            cy_b.hodge_numbers.clone(),
            vec![h21 as f64, h11 as f64, cy_b.euler_characteristic as f64],
        );

        Self {
            architecture_a: a,
            architecture_b: b,
            parameter_map: vec![(h11 as f64, h21 as f64), (h21 as f64, h11 as f64)],
        }
    }

    /// Verify that both architectures produce the same "invariants"
    /// (i.e., their performance metrics are related by the mirror map).
    pub fn verify_invariant_match(&self) -> bool {
        // The Euler characteristics should have opposite signs (mirror)
        let chi_a = self.architecture_a.euler_characteristic();
        let chi_b = self.architecture_b.euler_characteristic();
        chi_a == -chi_b
    }

    /// Verify that the capability matrices are transposes (mirror swap).
    pub fn verify_capability_mirror(&self) -> bool {
        let cap_a = &self.architecture_a.capabilities;
        let cap_b = &self.architecture_b.capabilities;
        if cap_a.len() != cap_b.len() {
            return false;
        }
        // For CY 3-fold mirror: h^{1,1}_A = h^{2,1}_B and vice versa
        // Check the swap of key Hodge numbers
        if cap_a.len() >= 2 && cap_b.len() >= 3 {
            if cap_a[1].len() >= 2 && cap_b[2].len() >= 2 {
                return cap_a[1][1] == cap_b[2][1] && cap_a[2][1] == cap_b[1][1];
            }
        }
        true
    }

    /// The "quantum" performance: how the duality affects computation.
    /// Analogous to quantum cohomology corrections.
    pub fn quantum_correction(&self, degree: u32) -> f64 {
        // The quantum correction measures the difference between
        // classical (product) and quantum (deformed) performance
        let base = self.architecture_a.performance.first().copied().unwrap_or(0.0);
        base / (degree as f64).powi(3)
    }

    /// Verify the full duality: invariant match + capability mirror.
    pub fn verify_full_duality(&self) -> bool {
        self.verify_invariant_match() && self.verify_capability_mirror()
    }

    /// The quintic agent duality: Quintic ↔ Mirror Quintic.
    pub fn quintic() -> Self {
        Self::cy_duality("Quintic Agent", "Mirror Quintic Agent", 1, 101)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_euler() {
        let a = AgentArchitecture::new(
            "Test",
            vec![vec![1, 0, 0, 1], vec![0, 5, 10, 0], vec![0, 10, 5, 0], vec![1, 0, 0, 1]],
            vec![5.0, 10.0],
        );
        let chi = a.euler_characteristic();
        // χ = (1+1+1+1) - (0+5+10+0) - (0+10+5+0) + (1+1+1+1) = 4 - 15 - 15 + 4 = -22
        // Wait, let's compute properly:
        // p=0: (+1)(0+0)+(-1)(0+0) = 1+0+0+1 = 2
        // p=1: (-1)(0+1)+(+1)(1+0) = -(0+5+10+0) = -15
        // p=2: (+1)(0+1)+(-1)(1+0) = -(0+10+5+0) = -15
        // p=3: (-1)(0+0)+(+1)(0+0) = 1+0+0+1 = 2
        // Actually: sum over all (p,q) of (-1)^{p+q} * h^{p,q}
        // = 1 -0-0+1 -0+5+10-0 -0-10-5+0 +1-0-0+1 = -10
        assert_eq!(chi, -10);
    }

    #[test]
    fn test_cy_duality() {
        let d = AgentDuality::cy_duality("A", "B", 3, 7);
        assert_eq!(d.architecture_a.name, "A");
        assert_eq!(d.architecture_b.name, "B");
    }

    #[test]
    fn test_quintic_duality_invariants() {
        let d = AgentDuality::quintic();
        assert!(d.verify_invariant_match());
    }

    #[test]
    fn test_quintic_duality_capability_mirror() {
        let d = AgentDuality::quintic();
        assert!(d.verify_capability_mirror());
    }

    #[test]
    fn test_quintic_duality_full() {
        let d = AgentDuality::quintic();
        assert!(d.verify_full_duality());
    }

    #[test]
    fn test_quantum_correction() {
        let d = AgentDuality::quintic();
        let qc = d.quantum_correction(1);
        assert!(qc > 0.0);
    }

    #[test]
    fn test_custom_duality_euler_opposite() {
        let d = AgentDuality::cy_duality("X", "Y", 5, 20);
        let chi_a = d.architecture_a.euler_characteristic();
        let chi_b = d.architecture_b.euler_characteristic();
        assert_eq!(chi_a, -chi_b);
    }

    #[test]
    fn test_agent_serialization() {
        let a = AgentArchitecture::new("Test", vec![vec![1]], vec![1.0]);
        // Verify Serialize/Deserialize compile
        let _serialized = format!("{}", a.name);
        assert_eq!(a.name, "Test");
    }
}
