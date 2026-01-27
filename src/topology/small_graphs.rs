//! Small graph collection for testing and benchmarking.
//!
//! This module provides a collection of well-known small graphs commonly used
//! in graph theory. The graphs are equivalent to those in Graphs.jl's smallgraph
//! function.
//!
//! All edges are 0-indexed (converted from Julia's 1-indexed representation).

/// Returns the edges of the Bull graph.
/// 5 vertices, 5 edges.
/// The bull graph is a triangle with two pendant edges.
pub fn bull() -> (usize, Vec<(usize, usize)>) {
    (5, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 4)])
}

/// Returns the edges of the Chvátal graph.
/// 12 vertices, 24 edges.
/// The Chvátal graph is the smallest triangle-free graph that is 4-chromatic and 4-regular.
pub fn chvatal() -> (usize, Vec<(usize, usize)>) {
    (12, vec![
        (0, 1), (0, 4), (0, 6), (0, 9),
        (1, 2), (1, 5), (1, 7),
        (2, 3), (2, 6), (2, 8),
        (3, 4), (3, 7), (3, 9),
        (4, 5), (4, 8),
        (5, 10), (5, 11),
        (6, 10), (6, 11),
        (7, 8), (7, 11),
        (8, 10),
        (9, 10), (9, 11),
    ])
}

/// Returns the edges of the Cubical graph (3-cube, Q3).
/// 8 vertices, 12 edges.
pub fn cubical() -> (usize, Vec<(usize, usize)>) {
    (8, vec![
        (0, 1), (0, 3), (0, 4),
        (1, 2), (1, 7),
        (2, 3), (2, 6),
        (3, 5),
        (4, 5), (4, 7),
        (5, 6),
        (6, 7),
    ])
}

/// Returns the edges of the Desargues graph.
/// 20 vertices, 30 edges.
pub fn desargues() -> (usize, Vec<(usize, usize)>) {
    (20, vec![
        (0, 1), (0, 5), (0, 19),
        (1, 2), (1, 16),
        (2, 3), (2, 11),
        (3, 4), (3, 14),
        (4, 5), (4, 9),
        (5, 6),
        (6, 7), (6, 15),
        (7, 8), (7, 18),
        (8, 9), (8, 13),
        (9, 10),
        (10, 11), (10, 19),
        (11, 12),
        (12, 13), (12, 17),
        (13, 14),
        (14, 15),
        (15, 16),
        (16, 17),
        (17, 18),
        (18, 19),
    ])
}

/// Returns the edges of the Diamond graph.
/// 4 vertices, 5 edges.
/// The diamond graph is K4 minus one edge.
pub fn diamond() -> (usize, Vec<(usize, usize)>) {
    (4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)])
}

/// Returns the edges of the Dodecahedral graph.
/// 20 vertices, 30 edges.
pub fn dodecahedral() -> (usize, Vec<(usize, usize)>) {
    (20, vec![
        (0, 1), (0, 10), (0, 19),
        (1, 2), (1, 8),
        (2, 3), (2, 6),
        (3, 4), (3, 19),
        (4, 5), (4, 17),
        (5, 6), (5, 15),
        (6, 7),
        (7, 8), (7, 14),
        (8, 9),
        (9, 10), (9, 13),
        (10, 11),
        (11, 12), (11, 18),
        (12, 13), (12, 16),
        (13, 14),
        (14, 15),
        (15, 16),
        (16, 17),
        (17, 18),
        (18, 19),
    ])
}

/// Returns the edges of the Frucht graph.
/// 12 vertices, 18 edges.
/// The Frucht graph is the smallest cubic graph with no non-trivial automorphisms.
pub fn frucht() -> (usize, Vec<(usize, usize)>) {
    (12, vec![
        (0, 1), (0, 6), (0, 7),
        (1, 2), (1, 7),
        (2, 3), (2, 8),
        (3, 4), (3, 9),
        (4, 5), (4, 9),
        (5, 6), (5, 10),
        (6, 10),
        (7, 11),
        (8, 9), (8, 11),
        (10, 11),
    ])
}

/// Returns the edges of the Heawood graph.
/// 14 vertices, 21 edges.
/// The Heawood graph is a cage and the incidence graph of the Fano plane.
pub fn heawood() -> (usize, Vec<(usize, usize)>) {
    (14, vec![
        (0, 1), (0, 5), (0, 13),
        (1, 2), (1, 10),
        (2, 3), (2, 7),
        (3, 4), (3, 12),
        (4, 5), (4, 9),
        (5, 6),
        (6, 7), (6, 11),
        (7, 8),
        (8, 9), (8, 13),
        (9, 10),
        (10, 11),
        (11, 12),
        (12, 13),
    ])
}

/// Returns the edges of the House graph.
/// 5 vertices, 6 edges.
/// The house graph is a square with a triangle on top.
pub fn house() -> (usize, Vec<(usize, usize)>) {
    (5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)])
}

/// Returns the edges of the House X graph.
/// 5 vertices, 8 edges.
/// The house graph with both diagonals of the square.
pub fn housex() -> (usize, Vec<(usize, usize)>) {
    (5, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3), (2, 4), (3, 4)])
}

/// Returns the edges of the Icosahedral graph.
/// 12 vertices, 30 edges.
pub fn icosahedral() -> (usize, Vec<(usize, usize)>) {
    (12, vec![
        (0, 1), (0, 5), (0, 7), (0, 8), (0, 11),
        (1, 2), (1, 5), (1, 6), (1, 8),
        (2, 3), (2, 6), (2, 8), (2, 9),
        (3, 4), (3, 6), (3, 9), (3, 10),
        (4, 5), (4, 6), (4, 10), (4, 11),
        (5, 6), (5, 11),
        (7, 8), (7, 9), (7, 10), (7, 11),
        (8, 9),
        (9, 10),
        (10, 11),
    ])
}

/// Returns the edges of Zachary's Karate Club graph.
/// 34 vertices, 78 edges.
/// A social network of a karate club.
pub fn karate() -> (usize, Vec<(usize, usize)>) {
    (34, vec![
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
        (0, 10), (0, 11), (0, 12), (0, 13), (0, 17), (0, 19), (0, 21), (0, 31),
        (1, 2), (1, 3), (1, 7), (1, 13), (1, 17), (1, 19), (1, 21), (1, 30),
        (2, 3), (2, 7), (2, 8), (2, 9), (2, 13), (2, 27), (2, 28), (2, 32),
        (3, 7), (3, 12), (3, 13),
        (4, 6), (4, 10),
        (5, 6), (5, 10), (5, 16),
        (6, 16),
        (8, 30), (8, 32), (8, 33),
        (9, 33),
        (13, 33),
        (14, 32), (14, 33),
        (15, 32), (15, 33),
        (18, 32), (18, 33),
        (19, 33),
        (20, 32), (20, 33),
        (22, 32), (22, 33),
        (23, 25), (23, 27), (23, 29), (23, 32), (23, 33),
        (24, 25), (24, 27), (24, 31),
        (25, 31),
        (26, 29), (26, 33),
        (27, 33),
        (28, 31), (28, 33),
        (29, 32), (29, 33),
        (30, 32), (30, 33),
        (31, 32), (31, 33),
        (32, 33),
    ])
}

/// Returns the edges of the Krackhardt Kite graph.
/// 10 vertices, 18 edges.
pub fn krackhardtkite() -> (usize, Vec<(usize, usize)>) {
    (10, vec![
        (0, 1), (0, 2), (0, 3), (0, 5),
        (1, 3), (1, 4), (1, 6),
        (2, 3), (2, 5),
        (3, 4), (3, 5), (3, 6),
        (4, 6),
        (5, 6), (5, 7),
        (6, 7),
        (7, 8),
        (8, 9),
    ])
}

/// Returns the edges of the Möbius-Kantor graph.
/// 16 vertices, 24 edges.
pub fn moebiuskantor() -> (usize, Vec<(usize, usize)>) {
    (16, vec![
        (0, 1), (0, 5), (0, 15),
        (1, 2), (1, 12),
        (2, 3), (2, 7),
        (3, 4), (3, 14),
        (4, 5), (4, 9),
        (5, 6),
        (6, 7), (6, 11),
        (7, 8),
        (8, 9), (8, 13),
        (9, 10),
        (10, 11), (10, 15),
        (11, 12),
        (12, 13),
        (13, 14),
        (14, 15),
    ])
}

/// Returns the edges of the Octahedral graph.
/// 6 vertices, 12 edges.
pub fn octahedral() -> (usize, Vec<(usize, usize)>) {
    (6, vec![
        (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 2), (1, 3), (1, 5),
        (2, 4), (2, 5),
        (3, 4), (3, 5),
        (4, 5),
    ])
}

/// Returns the edges of the Pappus graph.
/// 18 vertices, 27 edges.
pub fn pappus() -> (usize, Vec<(usize, usize)>) {
    (18, vec![
        (0, 1), (0, 5), (0, 17),
        (1, 2), (1, 8),
        (2, 3), (2, 13),
        (3, 4), (3, 10),
        (4, 5), (4, 15),
        (5, 6),
        (6, 7), (6, 11),
        (7, 8), (7, 14),
        (8, 9),
        (9, 10), (9, 16),
        (10, 11),
        (11, 12),
        (12, 13), (12, 17),
        (13, 14),
        (14, 15),
        (15, 16),
        (16, 17),
    ])
}

/// Returns the edges of the Petersen graph.
/// 10 vertices, 15 edges.
/// A well-known graph that is 3-regular and has many interesting properties.
pub fn petersen() -> (usize, Vec<(usize, usize)>) {
    (10, vec![
        (0, 1), (0, 4), (0, 5),
        (1, 2), (1, 6),
        (2, 3), (2, 7),
        (3, 4), (3, 8),
        (4, 9),
        (5, 7), (5, 8),
        (6, 8), (6, 9),
        (7, 9),
    ])
}

/// Returns the edges of the Sedgewick Maze graph.
/// 8 vertices, 10 edges.
pub fn sedgewickmaze() -> (usize, Vec<(usize, usize)>) {
    (8, vec![
        (0, 2), (0, 5), (0, 7),
        (1, 7),
        (2, 6),
        (3, 4), (3, 5),
        (4, 5), (4, 6), (4, 7),
    ])
}

/// Returns the edges of the Tetrahedral graph (K4).
/// 4 vertices, 6 edges.
pub fn tetrahedral() -> (usize, Vec<(usize, usize)>) {
    (4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)])
}

/// Returns the edges of the Truncated Cube graph.
/// 24 vertices, 36 edges.
pub fn truncatedcube() -> (usize, Vec<(usize, usize)>) {
    // Edges from Julia's Graphs.jl (converted to 0-indexed)
    (24, vec![
        (0, 1), (0, 2), (0, 4),
        (1, 11), (1, 14),
        (2, 3), (2, 4),
        (3, 6), (3, 8),
        (4, 5),
        (5, 16), (5, 18),
        (6, 7), (6, 8),
        (7, 10), (7, 12),
        (8, 9),
        (9, 17), (9, 20),
        (10, 11), (10, 12),
        (11, 14),
        (12, 13),
        (13, 21), (13, 22),
        (14, 15),
        (15, 19), (15, 23),
        (16, 17), (16, 18),
        (17, 20),
        (18, 19),
        (19, 23),
        (20, 21),
        (21, 22),
        (22, 23),
    ])
}

/// Returns the edges of the Truncated Tetrahedron graph.
/// 12 vertices, 18 edges.
pub fn truncatedtetrahedron() -> (usize, Vec<(usize, usize)>) {
    (12, vec![
        (0, 1), (0, 2), (0, 9),
        (1, 2), (1, 6),
        (2, 3),
        (3, 4), (3, 11),
        (4, 5), (4, 11),
        (5, 6), (5, 7),
        (6, 7),
        (7, 8),
        (8, 9), (8, 10),
        (9, 10),
        (10, 11),
    ])
}

/// Returns the edges of the Tutte graph.
/// 46 vertices, 69 edges.
/// A 3-regular graph that is not Hamiltonian.
pub fn tutte() -> (usize, Vec<(usize, usize)>) {
    (46, vec![
        (0, 1), (0, 2), (0, 3),
        (1, 4), (1, 26),
        (2, 10), (2, 11),
        (3, 18), (3, 19),
        (4, 5), (4, 33),
        (5, 6), (5, 29),
        (6, 7), (6, 27),
        (7, 8), (7, 14),
        (8, 9), (8, 38),
        (9, 10), (9, 37),
        (10, 39),
        (11, 12), (11, 39),
        (12, 13), (12, 35),
        (13, 14), (13, 15),
        (14, 34),
        (15, 16), (15, 22),
        (16, 17), (16, 44),
        (17, 18), (17, 43),
        (18, 45),
        (19, 20), (19, 45),
        (20, 21), (20, 41),
        (21, 22), (21, 23),
        (22, 40),
        (23, 24), (23, 27),
        (24, 25), (24, 32),
        (25, 26), (25, 31),
        (26, 33),
        (27, 28),
        (28, 29), (28, 32),
        (29, 30),
        (30, 31), (30, 33),
        (31, 32),
        (34, 35), (34, 38),
        (35, 36),
        (36, 37), (36, 39),
        (37, 38),
        (40, 41), (40, 44),
        (41, 42),
        (42, 43), (42, 45),
        (43, 44),
    ])
}

/// Get a small graph by name.
///
/// Returns `Some((num_vertices, edges))` if the graph exists, `None` otherwise.
///
/// Available graphs: bull, chvatal, cubical, desargues, diamond, dodecahedral,
/// frucht, heawood, house, housex, icosahedral, karate, krackhardtkite,
/// moebiuskantor, octahedral, pappus, petersen, sedgewickmaze, tetrahedral,
/// truncatedcube, truncatedtetrahedron, tutte
pub fn smallgraph(name: &str) -> Option<(usize, Vec<(usize, usize)>)> {
    match name {
        "bull" => Some(bull()),
        "chvatal" => Some(chvatal()),
        "cubical" => Some(cubical()),
        "desargues" => Some(desargues()),
        "diamond" => Some(diamond()),
        "dodecahedral" => Some(dodecahedral()),
        "frucht" => Some(frucht()),
        "heawood" => Some(heawood()),
        "house" => Some(house()),
        "housex" => Some(housex()),
        "icosahedral" => Some(icosahedral()),
        "karate" => Some(karate()),
        "krackhardtkite" => Some(krackhardtkite()),
        "moebiuskantor" => Some(moebiuskantor()),
        "octahedral" => Some(octahedral()),
        "pappus" => Some(pappus()),
        "petersen" => Some(petersen()),
        "sedgewickmaze" => Some(sedgewickmaze()),
        "tetrahedral" => Some(tetrahedral()),
        "truncatedcube" => Some(truncatedcube()),
        "truncatedtetrahedron" => Some(truncatedtetrahedron()),
        "tutte" => Some(tutte()),
        _ => None,
    }
}

/// List all available small graph names.
pub fn available_graphs() -> Vec<&'static str> {
    vec![
        "bull", "chvatal", "cubical", "desargues", "diamond", "dodecahedral",
        "frucht", "heawood", "house", "housex", "icosahedral", "karate",
        "krackhardtkite", "moebiuskantor", "octahedral", "pappus", "petersen",
        "sedgewickmaze", "tetrahedral", "truncatedcube", "truncatedtetrahedron",
        "tutte",
    ]
}

#[cfg(test)]
mod tests {
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
}
