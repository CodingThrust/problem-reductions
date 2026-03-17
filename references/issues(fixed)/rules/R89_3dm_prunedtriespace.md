---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING to PRUNED TRIE SPACE MINIMIZATION"
labels: rule
assignees: ''
---

**Source:** 3-DIMENSIONAL MATCHING
**Target:** PRUNED TRIE SPACE MINIMIZATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SR3, p.226

## GJ Source Entry

> [SR3] PRUNED TRIE SPACE MINIMIZATION
> INSTANCE: Finite set S, collection F of functions f: S→Z^+, and a positive integer K.
> QUESTION: Is there a sequence <f_1,f_2,…,f_m> of distinct functions from F such that for every two elements a,b∈S there is some i, 1≤i≤m, for which f_i(a)≠f_i(b) and such that, if N(i) denotes the number of distinct i-tuples X=(x_1,x_2,…,x_i) for which there is more than one a∈S having (f_1(a),f_2(a),…,f_i(a))=X, then Σ_{i=1}^{m} N(i)≤K?
> Reference: [Comer and Sethi, 1976]. Transformation from 3DM.
> Comment: Remains NP-complete even if all f∈F have range {0,1}. Variants in which the "pruned trie" data structure abstracted above is replaced by "full trie," "collapsed trie," or "pruned 0-trie" are also NP-complete. The related "access time minimization" problem is also NP-complete for pruned tries, where we ask for a sequence <f_1,f_2,…,f_m> of functions from F that distinguishes every two elements from S as above and such that, if the access time L(a) for a∈S is defined to be the least i for which no other b∈S has (f_1(b),f_2(b),…,f_i(b)) identical to (f_1(a),f_2(a),…,f_i(a)), then Σ_{a∈S} L(a)≤K.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Comer and Sethi, 1976]**: [`Comer1976`] D. Comer and R. Sethi (1976). "Complexity of Trie index construction". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 197–207. IEEE Computer Society.