---
name: Rule
about: Contributors to propose a new reduction rule
title: "[Rule] Source problem to target problem"
labels: rule
assignees: ''

---

Source problem: [source problem type name A, e.g. MaximumIndependentSet, find problems we already have here: https://codingthrust.github.io/problem-reductions/]
Target problem: [target problem type name B]

## Algorithm
[
You can show a detailed math decription here, it will be used  for coding implementation and the typst manual (for human) writing.
]

## Validation method
[
Proposed method for developing a test dataset (in json) to verify correctness. It is usually by round trip tests. e.g. map an instance A to B, solve B
e.g.1
Generate test dataset from an existing library: https://github.com/GiggleLiu/ProblemReductions.jl
e.g.2
- Source problem instance 1, expected target problem instance 1
- Source problem instance 2, expected target problem instance 2
]

## Round trip example to show
[
The example shown in the typst manual, focus on human verifiability. It usually includes
- a short description to what it is about
- the proposed source problem instance
]
