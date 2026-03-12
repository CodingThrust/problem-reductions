# Survey: Agentic Coding and Problem Reduction Rules

**Date:** 2026-03-12
**Papers:** 22
**Strategies used:** Landscape mapping

---

## Theme A: AI Coding Agents — Architectures and Benchmarks

The field has matured from proof-of-concept (Devin [Wu2024Devin], early 2024) to production-grade SDKs (OpenHands [Wang2024OpenHands], [Wang2025OpenHandsSDK]; Claude Code [Anthropic2025ClaudeCode]). The core architectural insight is the Agent-Computer Interface (ACI) — purpose-built tool interfaces for LLM agents [Yang2024SWEagent].

**Benchmarks reveal a capability cliff:** single-issue bug fixes reach ~70-80% (SWE-Bench Verified), but long-horizon multi-file tasks drop to ~20% [Thai2025SWEEVO], [Deng2025SWEBenchPro]. Self-evolving agents (Live-SWE-agent [Xia2025LiveSWEagent]) show promising results at 77.4% on SWE-Bench Verified.

**Industry perspective:** Developers use AI in 60% of work but maintain oversight on 80-100% of delegated tasks [Anthropic2026AgenticCoding]. The key challenge is specification inference — deciphering developer intent [Roychoudhury2025AgenticAI].

**Active groups:** Princeton (SWE-agent), UIUC/OpenHands consortium, Anthropic (Claude Code), Cognition AI (Devin), Scale AI (SWE-Bench Pro).

### Papers
- [Yang2024SWEagent] — SWE-agent: ACI design for coding agents (2024)
- [Wang2024OpenHands] — OpenHands: open platform for AI developers (2024)
- [Wang2025OpenHandsSDK] — OpenHands SDK: composable agent foundation (2025)
- [Thai2025SWEEVO] — SWE-EVO: long-horizon evolution benchmark (2025)
- [Deng2025SWEBenchPro] — SWE-Bench Pro: enterprise-level tasks (2025)
- [Xia2025LiveSWEagent] — Live-SWE-agent: self-evolving agents (2025)
- [Anthropic2025ClaudeCode] — Claude Code: agentic CLI tool (2025)
- [Wu2024Devin] — Devin: autonomous AI engineer (2024)
- [Roychoudhury2025AgenticAI] — Position paper on agentic AI for SE (2025)
- [Anthropic2026AgenticCoding] — Industry trends report (2026)

---

## Theme C: AI-Assisted Discovery of Reductions & Complexity Results

**The most directly relevant theme.** DeepMind's evolutionary approach — FunSearch [RomeraParedes2023FunSearch] (Nature 2023) followed by AlphaEvolve [Novikov2025AlphaEvolve] (2025) — demonstrates that LLM-powered program search can discover genuinely novel mathematical constructions. The breakthrough application to complexity theory: AlphaEvolve discovered new gadget reductions proving improved NP-hardness bounds for MAX-3-CUT (0.9649), MAX-4-CUT (0.987), and metric TSP (111/110) [Nagda2025ReinforcedGeneration].

**Key insight from Nagda et al.:** Verifying AI-discovered gadgets can be exponentially costly — they used AlphaEvolve itself to evolve faster verification procedures (10,000x speedup). This mirrors our project's need for automated reduction verification.

On the formal verification side, URSA [Janicic2025URSA] uses SAT solvers to verify NP-complete reductions — a complementary approach to LLM-based discovery. ALE-Bench [Imajuku2025ALEBench] benchmarks coding agents on NP-hard optimization (competitive with top-100 human contestants).

**Active groups:** Google DeepMind (FunSearch, AlphaEvolve), Sakana AI (ALE-Bench), University of Belgrade (URSA).

### Papers
- [Nagda2025ReinforcedGeneration] — AlphaEvolve discovers new NP-hardness gadgets (2025)
- [Novikov2025AlphaEvolve] — AlphaEvolve: evolutionary coding agent (2025)
- [RomeraParedes2023FunSearch] — FunSearch: LLM program search discovers cap set constructions (Nature 2023)
- [Imajuku2025ALEBench] — ALE-Bench: agents vs humans on NP-hard optimization (2025)
- [Janicic2025URSA] — URSA: SAT-based verification of NP-complete reductions (2025)

---

## Theme D (subset): Physics-Inspired QUBO/Ising Approaches

GNNs trained via QUBO Hamiltonian relaxation can solve MIS, MaxCut, MinVC at million-variable scale [Schuetz2022PhysicsGNN]. QUBO serves as a unifying target representation for combinatorial optimization — directly paralleling this project's use of QUBO as a central reduction hub. Quantum annealing + GNN hybrid approaches show promise for TSP [He2024QuantumTSP].

### Papers
- [Schuetz2022PhysicsGNN] — Physics-inspired GNN for QUBO problems (Nature Machine Intelligence 2022)
- [He2024QuantumTSP] — Quantum annealing + GNN for TSP via QUBO (2024)

---

## Theme E: LLM-Assisted Formal Verification & Program Synthesis

End-to-end formally verified code generation remains largely unsolved. The largest benchmark (VeriCoding [Bursuc2025VeriCoding]) shows 27% success in Lean, 44% in Verus/Rust, 82% in Dafny. The curated CLEVER benchmark [Thakur2025CLEVER] reports near-zero success on 161 hard problems. VeriBench [Miranda2025VeriBench] finds that only self-optimizing agent architectures achieve meaningful compilation rates (~90%).

For C programs specifically, the CoqPL/SYNVER line of work [Mukherjee2025CoqPL], [Mukherjee2025SynVer] demonstrates a two-LLM pipeline: one generates candidates, one generates Coq proofs. This pattern (generate + verify) is the emerging paradigm.

**Active groups:** MIT/Tegmark (VeriCoding), UT Austin/Caltech (CLEVER), Purdue (SYNVER), Stanford/ICML workshop (VeriBench).

### Papers
- [Bursuc2025VeriCoding] — VeriCoding: 12,504 formal specs across Lean/Dafny/Verus (2025)
- [Thakur2025CLEVER] — CLEVER: curated Lean verification benchmark (2025)
- [Miranda2025VeriBench] — VeriBench: end-to-end Lean 4 benchmark (2025)
- [Mukherjee2025CoqPL] — Automated verification of LLM-synthesized C (CoqPL 2025)
- [Mukherjee2025SynVer] — SYNVER: synthesis + Coq proof generation (ASE 2025)

---

## Key Open Problems

1. **Automated gadget discovery at scale** — AlphaEvolve works but verification is exponentially costly; can we build faster feedback loops?
2. **End-to-end reduction pipelines** — No system yet discovers a reduction, implements it, AND formally verifies correctness
3. **Long-horizon agent capability** — Agents fail at ~80% of multi-file, multi-step tasks (the kind needed for implementing reductions)
4. **Verified code generation** — Only 27% success on formal specs in Lean; major bottleneck for trustworthy AI-discovered reductions
5. **QUBO as universal target** — Can GNN/physics-inspired solvers be integrated into a reduction-aware optimization pipeline?

## Key Bottlenecks

1. **Verification cost** — Checking candidate gadgets/reductions is often exponentially expensive
2. **Specification gap** — LLMs struggle to produce formal specs from informal mathematical descriptions
3. **Agent scaffolding** — No standard architecture for combining code generation + formal verification + domain-specific evaluation
4. **Benchmark coverage** — No benchmark specifically targets reduction implementation and verification
