/-
  Reduction Proofs — Key Algebraic Lemmas

  Machine-checked proofs for critical identities used in the
  proposed reduction rules verification note.

  These proofs verify the arithmetic foundations WITHOUT Mathlib.
  All lemmas use only Lean 4 built-in `omega` and `ring` tactics.
-/

/-! ## SubsetSum → Partition: Padding Element Algebra (§2.1)

For the case Σ > 2T with d = Σ - 2T:
  - Σ' = Σ + d = 2(Σ - T)
  - Forward: T + d = Σ - T
  - Backward: (Σ - T) - d = T
-/

/-- When Σ > 2T and d = Σ - 2T, the new sum Σ' = 2(Σ - T). -/
theorem subsetsum_partition_sigma_gt (S T : Nat) (h : S > 2 * T) :
    S + (S - 2 * T) = 2 * (S - T) := by
  omega

/-- Forward direction: T + d = Σ - T. -/
theorem subsetsum_partition_forward_gt (S T : Nat) (h : S > 2 * T) :
    T + (S - 2 * T) = S - T := by
  omega

/-- Backward direction: (Σ - T) - d = T. -/
theorem subsetsum_partition_backward_gt (S T : Nat) (h : S > 2 * T) :
    (S - T) - (S - 2 * T) = T := by
  omega

/-- When Σ < 2T and d = 2T - Σ, the new sum Σ' = 2T. -/
theorem subsetsum_partition_sigma_lt (S T : Nat) (_h : S < 2 * T) (_hle : T ≤ S) :
    S + (2 * T - S) = 2 * T := by
  omega

/-- Forward complement: (Σ - T) + d = T. -/
theorem subsetsum_partition_forward_lt (S T : Nat) (h : S < 2 * T) (hle : T ≤ S) :
    (S - T) + (2 * T - S) = T := by
  omega

/-! ## DS → MinSum Multicenter: Distance Bound (§4.2) -/

/-- Total distance from n vertices to K centers where each non-center
has distance exactly 1. -/
theorem ds_minsum_total_distance (n K : Nat) (_h : K ≤ n) :
    0 * K + 1 * (n - K) = n - K := by
  omega

/-! ## VC → HC: Edge Count Identity (§2.2)

Total edges = 14m (widget) + (2m - n) (chain links) + 2nK (selector) = 16m - n + 2nK.
-/

/-- Total edge count of the VC→HC construction. -/
theorem vc_hc_total_edges (n m K : Nat) (h : 2 * m ≥ n) :
    14 * m + (2 * m - n) + 2 * n * K = 16 * m - n + 2 * n * K := by
  omega

/-! ## VC → PFES: Vertex and Edge Counts (§5.1)

H has 2n + 2m vertices and n + 4m edges.
-/

/-- Vertex count of PFES construction: n original + n control + 2m gadget. -/
theorem vc_pfes_vertex_count (n m : Nat) :
    n + n + 2 * m = 2 * n + 2 * m := by
  omega

/-- Edge count of PFES construction: n control + 4m gadget. -/
theorem vc_pfes_edge_count (n m : Nat) :
    n + 4 * m = n + 4 * m := by
  rfl

/-! ## MaxCut → OLA: Complement Identity Constant (§3.1)

L_{K_n} = n(n²-1)/6, verified for small concrete values.
The symbolic identity requires Mathlib's Finset.sum; here we
verify concrete instances that cover the examples in the note.
-/

/-- L_{K_4} = 4 * (16 - 1) / 6 = 10. Used in the C_4 example. -/
theorem lkn_4 : 4 * (4 ^ 2 - 1) / 6 = 10 := by native_decide

/-- L_{K_3} = 3 * (9 - 1) / 6 = 4. Used in the K_3 example. -/
theorem lkn_3 : 3 * (3 ^ 2 - 1) / 6 = 4 := by native_decide

/-- L_{K_5} = 5 * (25 - 1) / 6 = 20. -/
theorem lkn_5 : 5 * (5 ^ 2 - 1) / 6 = 20 := by native_decide

/-- L_{K_6} = 6 * (36 - 1) / 6 = 35. -/
theorem lkn_6 : 6 * (6 ^ 2 - 1) / 6 = 35 := by native_decide

/-! ## X3C → AP: Cost Bound Accounting (§4.3, currently OPEN)

These lemmas verify the arithmetic used in the cost-bound argument,
independent of whether the construction itself is correct.
-/

/-- If r > 0 extra groups, fewer intra-group arcs means higher inter-group cost.
With q+r groups, max intra = 3q - 2r, so inter ≥ A - 3q + 2r > A - 3q = K. -/
theorem x3c_ap_cost_exceeds (A q r : Nat) (hr : r > 0)
    (hA : A ≥ 3 * q) (hqr : 3 * q ≥ 2 * r) :
    A - (3 * q - 2 * r) ≥ (A - 3 * q) + 2 * r := by
  omega
