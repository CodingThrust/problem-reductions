# Workflow
1. File an issue
2. Validate issue:
    - Rule: 1. algorithm is useful, correct, detailed, and optimal, 2. doc contains correct reduction overhead, solid reference, a clear round trip test proposal.
    - Model: 1. useful, classification correct, can be related to existing problems, 2. doc contains correct solving complexity, solid reference.
    - Source white list - build up white list and dataset.
    - Issue check record, saved in a markdown file.
3. Implement
4. Review implementation:
    - Generic code quality check.
    - Faithful to issue, especially the round trip test is not changed, or ignored.

    Remark: in this phase, we do not focus too much on the documentation quality.

5. Agentic test:
    - Setup senarios, e.g.
        1. a student learning the reduction book, and is wondering how to prove A is NP-complete
        2. an algorithm developer develop an algorithm solving problem X, he use pred to check the existence of Y, such that by Y -> X, solving X gives better complexity against existing best known solver for Y.
        3. an engineer want to solve problem X, use pred to find the best algorithm.
    Each agent provides a report about "how to improve".
