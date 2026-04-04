Implement high confidence reduction rules in this pr. the rules are derived in ~/Downloads/reduction_derivations_confidence_upgraded_refs.typ , and confidence levels are at the end of the note. Some rules are verified with code in pr 992 and pr 996. We also have issues for these rules that provides the information. which is mentioned in the note

Rules must be implemented one by one in this branch:
- For each rule, invoke codex in headless mode, provide context for it, and let codex to use add-model to implement.
- Let codex to return all unexpected events during implementation, you record it into a log file.
