use super::*;

#[test]
fn test_bull() {
    let (n, edges) = bull();
    assert_eq!(n, 5);
    assert_eq!(edges.len(), 5);
}

#[test]
fn test_chvatal() {
    let (n, edges) = chvatal();
    assert_eq!(n, 12);
    assert_eq!(edges.len(), 24);
}

#[test]
fn test_cubical() {
    let (n, edges) = cubical();
    assert_eq!(n, 8);
    assert_eq!(edges.len(), 12);
}

#[test]
fn test_desargues() {
    let (n, edges) = desargues();
    assert_eq!(n, 20);
    assert_eq!(edges.len(), 30);
}

#[test]
fn test_diamond() {
    let (n, edges) = diamond();
    assert_eq!(n, 4);
    assert_eq!(edges.len(), 5);
}

#[test]
fn test_dodecahedral() {
    let (n, edges) = dodecahedral();
    assert_eq!(n, 20);
    assert_eq!(edges.len(), 30);
}

#[test]
fn test_frucht() {
    let (n, edges) = frucht();
    assert_eq!(n, 12);
    assert_eq!(edges.len(), 18);
}

#[test]
fn test_heawood() {
    let (n, edges) = heawood();
    assert_eq!(n, 14);
    assert_eq!(edges.len(), 21);
}

#[test]
fn test_house() {
    let (n, edges) = house();
    assert_eq!(n, 5);
    assert_eq!(edges.len(), 6);
}

#[test]
fn test_housex() {
    let (n, edges) = housex();
    assert_eq!(n, 5);
    assert_eq!(edges.len(), 8);
}

#[test]
fn test_icosahedral() {
    let (n, edges) = icosahedral();
    assert_eq!(n, 12);
    assert_eq!(edges.len(), 30);
}

#[test]
fn test_karate() {
    let (n, edges) = karate();
    assert_eq!(n, 34);
    assert_eq!(edges.len(), 78);
}

#[test]
fn test_krackhardtkite() {
    let (n, edges) = krackhardtkite();
    assert_eq!(n, 10);
    assert_eq!(edges.len(), 18);
}

#[test]
fn test_moebiuskantor() {
    let (n, edges) = moebiuskantor();
    assert_eq!(n, 16);
    assert_eq!(edges.len(), 24);
}

#[test]
fn test_octahedral() {
    let (n, edges) = octahedral();
    assert_eq!(n, 6);
    assert_eq!(edges.len(), 12);
}

#[test]
fn test_pappus() {
    let (n, edges) = pappus();
    assert_eq!(n, 18);
    assert_eq!(edges.len(), 27);
}

#[test]
fn test_petersen() {
    let (n, edges) = petersen();
    assert_eq!(n, 10);
    assert_eq!(edges.len(), 15);
}

#[test]
fn test_sedgewickmaze() {
    let (n, edges) = sedgewickmaze();
    assert_eq!(n, 8);
    assert_eq!(edges.len(), 10);
}

#[test]
fn test_tetrahedral() {
    let (n, edges) = tetrahedral();
    assert_eq!(n, 4);
    assert_eq!(edges.len(), 6);
}

#[test]
fn test_truncatedcube() {
    let (n, edges) = truncatedcube();
    assert_eq!(n, 24);
    assert_eq!(edges.len(), 36);
}

#[test]
fn test_truncatedtetrahedron() {
    let (n, edges) = truncatedtetrahedron();
    assert_eq!(n, 12);
    assert_eq!(edges.len(), 18);
}

#[test]
fn test_tutte() {
    let (n, edges) = tutte();
    assert_eq!(n, 46);
    assert_eq!(edges.len(), 69);
}

#[test]
fn test_smallgraph() {
    assert!(smallgraph("petersen").is_some());
    assert!(smallgraph("bull").is_some());
    assert!(smallgraph("nonexistent").is_none());
}

#[test]
fn test_available_graphs() {
    let graphs = available_graphs();
    assert_eq!(graphs.len(), 22);
    assert!(graphs.contains(&"petersen"));
}

#[test]
fn test_all_graphs_have_valid_edges() {
    for name in available_graphs() {
        let (n, edges) = smallgraph(name).unwrap();
        for (u, v) in edges {
            assert!(u < n, "{} has invalid edge: {} >= {}", name, u, n);
            assert!(v < n, "{} has invalid edge: {} >= {}", name, v, n);
            assert!(u != v, "{} has self-loop", name);
        }
    }
}
