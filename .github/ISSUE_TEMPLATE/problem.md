---
name: Problem
about: Contributors to propose a new model problem
title: "[Model] Problem name"
labels: model
assignees: ''

---

## Description
[Problem name, e.g. Maximum Independent Set]

[Description, with clear annotation of information source. e.g. 
Given a Graph G=(V,E), an independent set of vertices, i.e. a subset of V such that no two vertices in it are joined by an edge in E. To goal is to find the the independent set with maximum cardinality.

Ref: https://www.csc.kth.se/~viggo/wwwcompendium/node34.html
]

## Schema
[JSON Schema description, e.g.
Type name: MaximumIndependentSet
Vartiants: Different graph topolofy, weighted or unweighted (implemented as type parameters)
Fields:
- `graph`, a graph description
- `weights`, a vector of numbers, for weights on vertices
]
