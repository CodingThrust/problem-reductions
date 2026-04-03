use crate::prelude::*;
use crate::topology::SimpleGraph;

#[test]
fn test_prelude_exports_rectilinear_picture_compression() {
    let problem = RectilinearPictureCompression::new(vec![vec![true]], 1);
    assert_eq!(problem.bound(), 1);
}

#[test]
fn test_prelude_exports_partition_into_cliques() {
    let problem = PartitionIntoCliques::new(SimpleGraph::new(2, vec![(0, 1)]), 1);
    assert_eq!(problem.num_cliques(), 1);
}
