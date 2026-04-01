#!/usr/bin/env python3
"""Cross-compare constructor and adversary for SubsetSum → Partition."""
import itertools
import sys
sys.path.insert(0, "docs/paper/verify-reductions")

from verify_subsetsum_partition import (
    reduce as c_reduce,
    is_subset_sum_feasible as c_is_source_feasible,
    is_partition_feasible as c_is_target_feasible,
    extract_subsetsum_solution as c_extract,
)
from adversary_subsetsum_partition import (
    reduce as a_reduce_raw,
    is_feasible_source as a_is_source_feasible_raw,
    is_feasible_target as a_is_target_feasible_raw,
    SubsetSumInstance,
)

agree = disagree = 0
feasibility_mismatch = 0
extraction_mismatch = 0


def a_reduce(sizes, target):
    """Adapter for adversary reduce."""
    ss = SubsetSumInstance(list(sizes), target)
    part = a_reduce_raw(ss)
    return part.elements


for n in range(1, 6):
    max_val = min(8, 2**n)
    instances_tested = 0
    max_instances = 200

    if n <= 3:
        all_size_combos = list(itertools.product(range(1, max_val + 1), repeat=n))
    else:
        import random
        random.seed(999 + n)
        all_size_combos = [tuple(random.randint(1, max_val) for _ in range(n))
                           for _ in range(100)]

    for sizes in all_size_combos:
        sizes = list(sizes)
        sigma = sum(sizes)
        targets = [0, 1, sigma // 2, sigma, sigma + 1]

        for target in targets:
            if target < 0 or instances_tested >= max_instances:
                continue

            # Run both reductions
            c_part, c_d, c_sigma = c_reduce(sizes, target)
            a_part = a_reduce(sizes, target)

            # Compare structural equivalence
            if c_part == a_part:
                agree += 1
            else:
                disagree += 1
                print(f"  DISAGREE on sizes={sizes}, T={target}")
                print(f"    Constructor: {c_part}")
                print(f"    Adversary:   {a_part}")

            # Compare source feasibility
            c_sf = c_is_source_feasible(sizes, target)
            a_sf = a_is_source_feasible_raw(SubsetSumInstance(sizes, target))
            if c_sf != a_sf:
                feasibility_mismatch += 1
                print(f"  SOURCE FEASIBILITY MISMATCH: sizes={sizes}, T={target}")

            # Compare target feasibility
            c_tf = c_is_target_feasible(c_part)
            a_tf = a_is_target_feasible_raw(a_reduce_raw(SubsetSumInstance(sizes, target)))
            if c_tf != a_tf:
                feasibility_mismatch += 1
                print(f"  TARGET FEASIBILITY MISMATCH: sizes={sizes}, T={target}")

            instances_tested += 1

    print(f"n={n}: tested {instances_tested} instances")

print(f"\nCross-comparison: {agree} agree, {disagree} disagree, "
      f"{feasibility_mismatch} feasibility mismatches")
if disagree > 0 or feasibility_mismatch > 0:
    print("ACTION REQUIRED: investigate discrepancies before proceeding")
    sys.exit(1)
else:
    print("All instances agree between constructor and adversary.")
    sys.exit(0)
