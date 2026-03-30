//! Non-Liveness Free Petri Net problem implementation.
//!
//! Given a free-choice Petri net P = (S, T, F, M₀), determine whether P is
//! *not live*: does there exist a transition that can become permanently dead?
//! A transition t is *dead from marking M* if t never fires in any firing
//! sequence starting from M. The net is not live iff some transition is
//! globally dead from some reachable marking.
//!
//! The configuration is a binary vector over transitions, selecting which
//! transitions are claimed to be globally dead. The answer is YES (Or(true))
//! when at least one selected transition is indeed globally dead.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

inventory::submit! {
    ProblemSchemaEntry {
        name: "NonLivenessFreePetriNet",
        display_name: "Non-Liveness Free Petri Net",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a free-choice Petri net is not live (some transition can become permanently dead)",
        fields: &[
            FieldInfo { name: "num_places", type_name: "usize", description: "Number of places |S|" },
            FieldInfo { name: "num_transitions", type_name: "usize", description: "Number of transitions |T|" },
            FieldInfo { name: "place_to_transition", type_name: "Vec<(usize,usize)>", description: "Arcs from places to transitions" },
            FieldInfo { name: "transition_to_place", type_name: "Vec<(usize,usize)>", description: "Arcs from transitions to places" },
            FieldInfo { name: "initial_marking", type_name: "Vec<usize>", description: "Initial marking M₀ (tokens per place)" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "NonLivenessFreePetriNet",
        fields: &["num_places", "num_transitions", "num_arcs", "initial_token_sum"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NonLivenessFreePetriNet {
    num_places: usize,
    num_transitions: usize,
    place_to_transition: Vec<(usize, usize)>,
    transition_to_place: Vec<(usize, usize)>,
    initial_marking: Vec<usize>,
}

impl NonLivenessFreePetriNet {
    fn validate_inputs(
        num_places: usize,
        num_transitions: usize,
        place_to_transition: &[(usize, usize)],
        transition_to_place: &[(usize, usize)],
        initial_marking: &[usize],
    ) -> Result<(), String> {
        if num_places == 0 {
            return Err("NonLivenessFreePetriNet requires at least one place".to_string());
        }
        if num_transitions == 0 {
            return Err("NonLivenessFreePetriNet requires at least one transition".to_string());
        }
        if initial_marking.len() != num_places {
            return Err(format!(
                "initial_marking length {} does not match num_places {}",
                initial_marking.len(),
                num_places
            ));
        }
        for (i, &(p, t)) in place_to_transition.iter().enumerate() {
            if p >= num_places {
                return Err(format!(
                    "place_to_transition arc {} has place {} out of range 0..{}",
                    i, p, num_places
                ));
            }
            if t >= num_transitions {
                return Err(format!(
                    "place_to_transition arc {} has transition {} out of range 0..{}",
                    i, t, num_transitions
                ));
            }
        }
        for (i, &(t, p)) in transition_to_place.iter().enumerate() {
            if t >= num_transitions {
                return Err(format!(
                    "transition_to_place arc {} has transition {} out of range 0..{}",
                    i, t, num_transitions
                ));
            }
            if p >= num_places {
                return Err(format!(
                    "transition_to_place arc {} has place {} out of range 0..{}",
                    i, p, num_places
                ));
            }
        }

        // Validate free-choice property: for any two transitions sharing an
        // input place, they must share ALL input places (identical preset).
        let mut preset: HashMap<usize, HashSet<usize>> = HashMap::new();
        for &(p, t) in place_to_transition {
            preset.entry(t).or_default().insert(p);
        }
        // Group transitions by shared input places
        for &(p, _) in place_to_transition {
            let transitions_from_p: Vec<usize> = place_to_transition
                .iter()
                .filter(|&&(pp, _)| pp == p)
                .map(|&(_, t)| t)
                .collect();
            for i in 0..transitions_from_p.len() {
                for j in (i + 1)..transitions_from_p.len() {
                    let t1 = transitions_from_p[i];
                    let t2 = transitions_from_p[j];
                    let p1 = preset.get(&t1).cloned().unwrap_or_default();
                    let p2 = preset.get(&t2).cloned().unwrap_or_default();
                    if p1 != p2 {
                        return Err(format!(
                            "Free-choice violation: transitions {} and {} share input place {} but have different presets",
                            t1, t2, p
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Try to create a new `NonLivenessFreePetriNet` instance, returning an error
    /// if validation fails.
    pub fn try_new(
        num_places: usize,
        num_transitions: usize,
        place_to_transition: Vec<(usize, usize)>,
        transition_to_place: Vec<(usize, usize)>,
        initial_marking: Vec<usize>,
    ) -> Result<Self, String> {
        Self::validate_inputs(
            num_places,
            num_transitions,
            &place_to_transition,
            &transition_to_place,
            &initial_marking,
        )?;
        Ok(Self {
            num_places,
            num_transitions,
            place_to_transition,
            transition_to_place,
            initial_marking,
        })
    }

    /// Create a new `NonLivenessFreePetriNet` instance.
    ///
    /// # Panics
    ///
    /// Panics if validation fails (indices out of range, wrong marking length,
    /// or free-choice violation).
    pub fn new(
        num_places: usize,
        num_transitions: usize,
        place_to_transition: Vec<(usize, usize)>,
        transition_to_place: Vec<(usize, usize)>,
        initial_marking: Vec<usize>,
    ) -> Self {
        Self::try_new(
            num_places,
            num_transitions,
            place_to_transition,
            transition_to_place,
            initial_marking,
        )
        .unwrap_or_else(|message| panic!("{message}"))
    }

    /// Number of places |S|.
    pub fn num_places(&self) -> usize {
        self.num_places
    }

    /// Number of transitions |T|.
    pub fn num_transitions(&self) -> usize {
        self.num_transitions
    }

    /// Total number of arcs |F|.
    pub fn num_arcs(&self) -> usize {
        self.place_to_transition.len() + self.transition_to_place.len()
    }

    /// Sum of tokens in the initial marking.
    pub fn initial_token_sum(&self) -> usize {
        self.initial_marking.iter().sum()
    }

    /// Arcs from places to transitions.
    pub fn place_to_transition(&self) -> &[(usize, usize)] {
        &self.place_to_transition
    }

    /// Arcs from transitions to places.
    pub fn transition_to_place(&self) -> &[(usize, usize)] {
        &self.transition_to_place
    }

    /// Initial marking M₀.
    pub fn initial_marking(&self) -> &[usize] {
        &self.initial_marking
    }

    /// Determine which transitions are enabled at the given marking.
    fn enabled_transitions(&self, marking: &[usize]) -> Vec<bool> {
        let mut enabled = vec![true; self.num_transitions];
        // A transition t is enabled iff every input place has at least one token.
        // First, mark all transitions that have at least one input place.
        let mut has_input = vec![false; self.num_transitions];
        for &(p, t) in &self.place_to_transition {
            has_input[t] = true;
            if marking[p] == 0 {
                enabled[t] = false;
            }
        }
        // Transitions with no input places are always enabled (source transitions).
        // They remain true in the enabled vector.
        // But we need to handle the case where has_input is false: leave enabled as true.
        let _ = has_input; // used implicitly above
        enabled
    }

    /// Fire a transition, producing a new marking. Returns None if not enabled.
    fn fire(&self, marking: &[usize], transition: usize) -> Option<Vec<usize>> {
        let mut new_marking = marking.to_vec();
        // Remove tokens from input places
        for &(p, t) in &self.place_to_transition {
            if t == transition {
                if new_marking[p] == 0 {
                    return None;
                }
                new_marking[p] -= 1;
            }
        }
        // Add tokens to output places
        for &(t, p) in &self.transition_to_place {
            if t == transition {
                new_marking[p] += 1;
            }
        }
        Some(new_marking)
    }

    /// Build the bounded reachability graph and determine which transitions
    /// are globally dead (i.e., there exists a reachable marking from which
    /// the transition can never fire again).
    ///
    /// For boundedness, we cap exploration at markings where no place exceeds
    /// `initial_token_sum`. This is sound for free-choice nets under the
    /// NP-completeness assumption from Garey & Johnson.
    fn compute_globally_dead_transitions(&self) -> Vec<bool> {
        let token_cap = self.initial_token_sum();
        let num_t = self.num_transitions;

        // Build reachability graph: BFS from initial marking.
        let mut marking_index: HashMap<Vec<usize>, usize> = HashMap::new();
        let mut markings: Vec<Vec<usize>> = Vec::new();
        // successors[m_idx] = list of (transition, next_marking_idx)
        let mut successors: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        let initial = self.initial_marking.clone();
        marking_index.insert(initial.clone(), 0);
        markings.push(initial);
        successors.push(Vec::new());
        queue.push_back(0);

        while let Some(m_idx) = queue.pop_front() {
            let enabled = self.enabled_transitions(&markings[m_idx]);
            for (t, &is_enabled) in enabled.iter().enumerate() {
                if !is_enabled {
                    continue;
                }
                if let Some(new_marking) = self.fire(&markings[m_idx], t) {
                    // Check bound: no place exceeds token_cap
                    if new_marking.iter().any(|&tokens| tokens > token_cap) {
                        continue;
                    }
                    let next_idx = if let Some(&idx) = marking_index.get(&new_marking) {
                        idx
                    } else {
                        let idx = markings.len();
                        marking_index.insert(new_marking.clone(), idx);
                        markings.push(new_marking);
                        successors.push(Vec::new());
                        queue.push_back(idx);
                        idx
                    };
                    successors[m_idx].push((t, next_idx));
                }
            }
        }

        let num_markings = markings.len();

        // For each transition t, find the set of markings from which t can
        // eventually fire (via BFS on the reachability graph).
        // A transition is globally dead iff there exists a reachable marking
        // NOT in this set.
        //
        // We compute this by backward BFS: starting from markings where t fires,
        // propagate backward through all transitions.
        let mut globally_dead = vec![false; num_t];

        // Build reverse adjacency once (shared across all transitions).
        let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); num_markings];
        for (m_idx, succs) in successors.iter().enumerate() {
            for &(_tr, next_idx) in succs {
                predecessors[next_idx].push(m_idx);
            }
        }

        for (t, dead) in globally_dead.iter_mut().enumerate() {
            // Find markings where transition t is directly fired
            // (i.e., markings that have an outgoing edge for transition t)
            let mut can_reach_t = vec![false; num_markings];
            let mut bfs_queue: VecDeque<usize> = VecDeque::new();

            for (m_idx, succs) in successors.iter().enumerate() {
                if succs.iter().any(|&(tr, _)| tr == t) {
                    can_reach_t[m_idx] = true;
                    bfs_queue.push_back(m_idx);
                }
            }

            // Backward BFS: from which markings can we reach a marking where t fires?
            while let Some(m_idx) = bfs_queue.pop_front() {
                for &pred_idx in &predecessors[m_idx] {
                    if !can_reach_t[pred_idx] {
                        can_reach_t[pred_idx] = true;
                        bfs_queue.push_back(pred_idx);
                    }
                }
            }

            // t is globally dead iff some reachable marking cannot reach a firing of t
            if can_reach_t.iter().any(|&reached| !reached) {
                *dead = true;
            }
        }

        globally_dead
    }
}

#[derive(Deserialize)]
struct NonLivenessFreePetriNetData {
    num_places: usize,
    num_transitions: usize,
    place_to_transition: Vec<(usize, usize)>,
    transition_to_place: Vec<(usize, usize)>,
    initial_marking: Vec<usize>,
}

impl<'de> Deserialize<'de> for NonLivenessFreePetriNet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = NonLivenessFreePetriNetData::deserialize(deserializer)?;
        Self::try_new(
            data.num_places,
            data.num_transitions,
            data.place_to_transition,
            data.transition_to_place,
            data.initial_marking,
        )
        .map_err(D::Error::custom)
    }
}

impl Problem for NonLivenessFreePetriNet {
    const NAME: &'static str = "NonLivenessFreePetriNet";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_transitions]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != self.num_transitions {
            return Or(false);
        }

        let globally_dead = self.compute_globally_dead_transitions();

        // Config selects transitions claimed to be dead.
        // Return true iff at least one selected transition is indeed globally dead.
        for (t, &selected) in config.iter().enumerate() {
            if selected == 1 && globally_dead[t] {
                return Or(true);
            }
        }

        Or(false)
    }
}

crate::declare_variants! {
    default NonLivenessFreePetriNet => "(initial_token_sum + 1) ^ num_places * num_transitions",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "non_liveness_free_petri_net",
        instance: Box::new(NonLivenessFreePetriNet::new(
            4,
            3,
            vec![(0, 0), (1, 1), (2, 2)],
            vec![(0, 1), (1, 2), (2, 3)],
            vec![1, 0, 0, 0],
        )),
        optimal_config: vec![1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/non_liveness_free_petri_net.rs"]
mod tests;
