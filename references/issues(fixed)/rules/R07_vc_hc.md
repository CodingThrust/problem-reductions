---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to HAMILTONIAN CIRCUIT"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** HAMILTONIAN CIRCUIT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.4, p.56-60

## Reduction Algorithm

> Theorem 3.4 HAMILTONIAN CIRCUIT is NP-complete
> Proof: It is easy to see that HC E NP, because a nondeterministic algorithm need only guess an ordering of the vertices and check in polynomial time that all the required edges belong to the edge set of the given graph.
>
> We transform VERTEX COVER to HC. Let an arbitrary instance of VC be given by the graph G = (V,E) and the positive integer K <= |V|. We must construct a graph G' = (V',E') such that G' has a Hamiltonian circuit if and only if G has a vertex cover of size K or less.
>
> Once more our construction can be viewed in terms of components connected together by communication links. First, the graph G' has K "selector" vertices a1,a2, . . . , aK, which will be used to select K vertices from the vertex set V for G. Second, for each edge in E, G' contains a "cover-testing" component that will be used to ensure that at least one endpoint of that edge is among the selected K vertices. The component for e = {u,v} E E is illustrated in Figure 3.4. It has 12 vertices,
>
> V'_e = {(u,e,i),(v,e,i): 1 <= i <= 6}
>
> and 14 edges,
>
> E'_e = {{(u,e,i),(u,e,i+1)},{(v,e,i),(v,e,i+1)}: 1 <= i <= 5}
>      U {{(u,e,3),(v,e,1)},{(v,e,3),(u,e,1)}}
>      U {{(u,e,6),(v,e,4)},{(v,e,6),(u,e,4)}}
>
> In the completed construction, the only vertices from this cover-testing component that will be involved in any additional edges are (u,e,1), (v,e,1), (u,e,6), and (v,e,6). This will imply, as the reader may readily verify, that any Hamiltonian circuit of G' will have to meet the edges in E'_e in exactly one of the three configurations shown in Figure 3.5. Thus, for example, if the circuit "enters" this component at (u,e,1), it will have to "exit" at (u,e,6) and visit either all 12 vertices in the component or just the 6 vertices (u,e,i), 1 <= i <= 6.
>
> Additional edges in our overall construction will serve to join pairs of cover-testing components or to join a cover-testing component to a selector vertex. For each vertex v E V, let the edges incident on v be ordered (arbitrarily) as e_{v[1]}, e_{v[2]}, . . . , e_{v[deg(v)]}, where deg(v) denotes the degree of v in G, that is, the number of edges incident on v. All the cover-testing components corresponding to these edges (having v as endpoint) are joined together by the following connecting edges:
>
> E'_v = {{(v,e_{v[i]},6),(v,e_{v[i+1]},1)}: 1 <= i < deg(v)}
>
> As shown in Figure 3.6, this creates a single path in G' that includes exactly those vertices (x,y,z) having x = v.
>
> The final connecting edges in G' join the first and last vertices from each of these paths to every one of the selector vertices a1,a2, . . . , aK. These edges are specified as follows:
>
> E'' = {{a_i,(v,e_{v[1]},1)},{a_i,(v,e_{v[deg(v)]},6)}: 1 <= i <= K, v E V}
>
> The completed graph G' = (V',E') has
>
> V' = {a_i: 1 <= i <= K} U (U_{e E E} V'_e)
>
> and
>
> E' = (U_{e E E} E'_e) U (U_{v E V} E'_v) U E''
>
> It is not hard to see that G' can be constructed from G and K in polynomial time.
>
> We claim that G' has a Hamiltonian circuit if and only if G has a vertex cover of size K or less. Suppose <v1,v2, . . . , vn>, where n = |V'|, is a Hamiltonian circuit for G'. Consider any portion of this circuit that begins at a vertex in the set {a1,a2, . . . , aK}, ends at a vertex in {a1,a2, . . . , aK}, and that encounters no such vertex internally. Because of the previously mentioned restrictions on the way in which a Hamiltonian circuit can pass through a cover-testing component, this portion of the circuit must pass through a set of cover-testing components corresponding to exactly those edges from E that are incident on some one particular vertex v E V. Each of the cover-testing components is traversed in one of the modes (a), (b), or (c) of Figure 3.5, and no vertex from any other cover-testing component is encountered. Thus the K vertices from {a1,a2, . . . , aK} divide the Hamiltonian circuit into K paths, each path corresponding to a distinct vertex v E V. Since the Hamiltonian circuit must include all vertices from every one of the cover-testing components, and since vertices from the cover-testing component for edge e E E can be traversed only by a path corresponding to an endpoint of e, every edge in E must have at least one endpoint among those K selected vertices. Therefore, this set of K vertices forms the desired vertex cover for G.
>
> Conversely, suppose V* ⊆ V is a vertex cover for G with |V*| <= K. We can assume that |V*| = K since additional vertices from V can always be added and we will still have a vertex cover. Let the elements of V* be labeled as v1,v2, . . . , vK. The following edges are chosen to be "in" the Hamiltonian circuit for G'. From the cover-testing component representing each edge e = {u,v} E E, choose the edges specified in Figure 3.5(a), (b), or (c) depending on whether {u,v} ∩ V* equals, respectively, {u}, {u,v}, or {v}. One of these three possibilities must hold since V* is a vertex cover for G. Next, choose all the edges in E'_{v_i} for 1 <= i <= K. Finally, choose the edges
>
> {a_i,(v_i,e_{v_i[1]},1)}, 1 <= i <= K
>
> {a_{i+1},(v_i,e_{v_i[deg(v_i)]},6)}, 1 <= i < K
>
> and
>
> {a_1,(v_K,e_{v_K[deg(v_K)]},6)}
>
> We leave to the reader the task of verifying that this set of edges actually corresponds to a Hamiltonian circuit for G'.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
