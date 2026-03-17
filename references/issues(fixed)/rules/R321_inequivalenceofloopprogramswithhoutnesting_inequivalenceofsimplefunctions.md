---
name: Rule
about: Propose a new reduction rule
title: "[Rule] INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING to INEQUIVALENCE OF SIMPLE FUNCTIONS"
labels: rule
assignees: ''
---

**Source:** INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING
**Target:** INEQUIVALENCE OF SIMPLE FUNCTIONS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO15

## GJ Source Entry

> [PO15]  INEQUIVALENCE OF SIMPLE FUNCTIONS
> INSTANCE:  Finite set X of variables, two expressions f and g over X, each being a composition of functions from the collection "s(x)=x+1," "p(x)=max{x−1,0}," "plus(x,y)=x+y," "div(x,t)=⌊x/t⌋," "mod(x,t)=x−t·⌊x/t⌋," "w(x,y)=if y=0 then x else 0," and "selectin(x1,x2,...,xn)=xi" where x,y,xi ∈ X, i,n,t ∈ Z+, and i ≤ n.
> QUESTION:  Is there an assignment of non-negative integer values to the variables in X for which the values of f and g differ?
> Reference:  [Tsichritzis, 1970]. Transformation from INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING.
> Comment:  Remains NP-complete even if f and g are defined only in terms of w(x,y), in terms of plus and mod, or in terms of plus and p [Lieberherr, 1977].  Variants in which f and g are defined in terms of plus and "sub1(x)=max{0,1−x}," or solely in terms of "minus(x,y)=max{0,x−y}," (where in both cases x,y ∈ X ∪ Z+) are also NP-complete [Constable, Hunt, and Sahni, 1974].

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

- **[Tsichritzis, 1970]**: [`Tsichritzis1970`] Dennis Tsichritzis (1970). "The equivalence problem of simple programs". *Journal of the Association for Computing Machinery* 17, pp. 729–738.
- **[Lieberherr, 1977]**: [`Lieberherr1977`] Karl Lieberherr (1977). "".
- **[Constable, Hunt, and Sahni, 1974]**: [`Constable1974`] R. L. Constable and H. B. Hunt, III and S. Sahni (1974). "On the computational complexity of scheme equivalence". Cornell University.