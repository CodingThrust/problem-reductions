use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "pred",
    about = "Explore NP-hard problem reductions",
    version,
    after_help = "\
Typical workflow:
  pred create MIS --graph 0-1,1-2,2-3 -o problem.json
  pred solve problem.json
  pred evaluate problem.json --config 1,0,1,0

Piping (use - to read from stdin):
  pred create MIS --graph 0-1,1-2 | pred solve -                    # when an ILP reduction path exists
  pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2 | pred solve - --solver brute-force
  pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1
  pred create MIS --graph 0-1,1-2 | pred reduce - --to QUBO

JSON output (any command):
  pred list --json                 # JSON to stdout
  pred show MIS --json | jq '.'   # pipe to jq

Use `pred <command> --help` for detailed usage of each command.
Use `pred list` to see all available problem types.

Enable tab completion:
  eval \"$(pred completions)\"     # add to ~/.bashrc or ~/.zshrc"
)]
pub struct Cli {
    /// Output file path (implies JSON output)
    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,

    /// Suppress informational messages on stderr
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// Output JSON to stdout instead of human-readable text
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all registered problem types (or reduction rules with --rules)
    #[command(after_help = "\
Examples:
  pred list                   # list problem types
  pred list --rules           # list all reduction rules
  pred list -o problems.json  # save as JSON")]
    List {
        /// List reduction rules instead of problem types
        #[arg(long)]
        rules: bool,
    },

    /// Show details for a problem type or variant (fields, reductions, complexity)
    #[command(after_help = "\
Examples:
  pred show MIS                   # all variants for MIS
  pred show MIS/UnitDiskGraph     # specific variant
  pred show MIS/UnitDiskGraph/i32 # fully qualified variant
  pred show KSAT/K3               # KSatisfiability with K=3

Use `pred list` to see all available problem types and variants.")]
    Show {
        /// Problem name or variant (e.g., MIS, MIS/UnitDiskGraph, KSAT/K3)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
    },

    /// Explore problems that reduce TO this one (incoming neighbors)
    #[command(after_help = "\
Examples:
  pred to MIS              # what reduces to MIS? (1 hop)
  pred to MIS --hops 2     # 2-hop incoming neighbors
  pred to MIS -o out.json  # save as JSON

Use `pred from <problem>` for outgoing neighbors (what this reduces to).")]
    To {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Explore problems this reduces to, starting FROM it (outgoing neighbors)
    #[command(after_help = "\
Examples:
  pred from MIS              # what does MIS reduce to? (1 hop)
  pred from MIS --hops 2     # 2-hop outgoing neighbors
  pred from MIS -o out.json  # save as JSON

Use `pred to <problem>` for incoming neighbors (what reduces to this).")]
    From {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Find the cheapest reduction path between two problems
    #[command(after_help = "\
Examples:
  pred path MIS QUBO                              # cheapest path
  pred path MIS QUBO --all                        # all paths
  pred path MIS QUBO -o path.json                 # save for `pred reduce --via`
  pred path MIS QUBO --all -o paths/              # save all paths to a folder
  pred path MIS QUBO --cost minimize:num_variables

Use `pred list` to see available problems.")]
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        source: String,
        /// Target problem (e.g., QUBO)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        target: String,
        /// Cost function [default: minimize-steps]
        #[arg(long, default_value = "minimize-steps")]
        cost: String,
        /// Show all paths instead of just the cheapest
        #[arg(long)]
        all: bool,
        /// Maximum paths to return in --all mode
        #[arg(long, default_value_t = 20)]
        max_paths: usize,
    },

    /// Export the reduction graph to JSON
    #[command(after_help = "\
Examples:
  pred export-graph                           # print to stdout
  pred export-graph -o reduction_graph.json   # save to file")]
    ExportGraph,

    /// Create a problem instance and save as JSON
    Create(Box<CreateArgs>),
    /// Evaluate a configuration against a problem instance JSON file
    Evaluate(EvaluateArgs),
    /// Reduce a problem instance to a target type
    Reduce(ReduceArgs),
    /// Inspect a problem JSON or reduction bundle
    #[command(after_help = "\
Examples:
  pred inspect problem.json
  pred inspect bundle.json
  pred create MIS --graph 0-1,1-2 | pred inspect -")]
    Inspect(InspectArgs),
    /// Solve a problem instance
    Solve(SolveArgs),
    /// Start MCP (Model Context Protocol) server for AI assistant integration
    #[cfg(feature = "mcp")]
    #[command(after_help = "\
Start a stdio-based MCP server that exposes problem reduction tools
to any MCP-compatible AI assistant.

Configuration:

  Claude Code / Claude Desktop (.mcp.json or ~/.claude/mcp.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  Cursor (.cursor/mcp.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  Windsurf (~/.codeium/windsurf/mcp_config.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  OpenCode (opencode.json):
    { \"mcp\": { \"problemreductions\": {
        \"type\": \"local\", \"command\": [\"pred\", \"mcp\"] } } }

Test with MCP Inspector:
  npx @modelcontextprotocol/inspector pred mcp")]
    Mcp,
    /// Print shell completions to stdout (auto-detects shell)
    #[command(after_help = "\
Setup: add one line to your shell rc file:

  # bash (~/.bashrc)
  eval \"$(pred completions bash)\"

  # zsh (~/.zshrc)
  eval \"$(pred completions zsh)\"

  # fish (~/.config/fish/config.fish)
  pred completions fish | source")]
    Completions {
        /// Shell type (bash, zsh, fish, etc.). Auto-detected if omitted.
        shell: Option<clap_complete::Shell>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ExampleSide {
    Source,
    Target,
}

#[derive(clap::Args)]
#[command(after_help = "\
TIP: Run `pred create <PROBLEM>` (no other flags) to see problem-specific help.
     Not every flag applies to every problem — the above list shows ALL flags.

Flags by problem type:
  MIS, MVC, MaxClique, MinDomSet  --graph, --weights
  MaxCut, MaxMatching, TSP        --graph, --edge-weights
  LongestPath                     --graph, --edge-lengths, --source-vertex, --target-vertex
  ShortestWeightConstrainedPath   --graph, --edge-lengths, --edge-weights, --source-vertex, --target-vertex, --length-bound, --weight-bound
  MaximalIS                       --graph, --weights
  SAT, NAESAT                     --num-vars, --clauses
  KSAT                            --num-vars, --clauses [--k]
  QUBO                            --matrix
  SpinGlass                       --graph, --couplings, --fields
  KColoring                       --graph, --k
  KClique                         --graph, --k
  MinimumMultiwayCut              --graph, --terminals, --edge-weights
  PartitionIntoTriangles          --graph
  GraphPartitioning               --graph
  GeneralizedHex                  --graph, --source, --sink
  MinimumCutIntoBoundedSets       --graph, --edge-weights, --source, --sink, --size-bound, --cut-bound
  HamiltonianCircuit, HC          --graph
  BoundedComponentSpanningForest  --graph, --weights, --k, --bound
  UndirectedTwoCommodityIntegralFlow --graph, --capacities, --source-1, --sink-1, --source-2, --sink-2, --requirement-1, --requirement-2
  IsomorphicSpanningTree          --graph, --tree
  KthBestSpanningTree             --graph, --edge-weights, --k, --bound
  LengthBoundedDisjointPaths      --graph, --source, --sink, --num-paths-required, --bound
  Factoring                       --target, --m, --n
  BinPacking                      --sizes, --capacity
  SubsetSum                       --sizes, --target
  SumOfSquaresPartition           --sizes, --num-groups, --bound
  PaintShop                       --sequence
  MaximumSetPacking               --sets [--weights]
  MinimumHittingSet               --universe, --sets
  MinimumSetCovering              --universe, --sets [--weights]
  EnsembleComputation             --universe, --sets, --budget
  ComparativeContainment          --universe, --r-sets, --s-sets [--r-weights] [--s-weights]
  X3C (ExactCoverBy3Sets)         --universe, --sets (3 elements each)
  SetBasis                        --universe, --sets, --k
  MinimumCardinalityKey           --num-attributes, --dependencies, --k
  PrimeAttributeName              --universe, --deps, --query
  TwoDimensionalConsecutiveSets   --alphabet-size, --sets
  BicliqueCover                   --left, --right, --biedges, --k
  BalancedCompleteBipartiteSubgraph --left, --right, --biedges, --k
  BiconnectivityAugmentation      --graph, --potential-edges, --budget [--num-vertices]
  BMF                             --matrix (0/1), --rank
  ConsecutiveBlockMinimization    --matrix (JSON 2D bool), --bound-k
  ConsecutiveOnesSubmatrix        --matrix (0/1), --k
  SteinerTree                     --graph, --edge-weights, --terminals
  MultipleCopyFileAllocation      --graph, --usage, --storage, --bound
  AcyclicPartition                --arcs [--weights] [--arc-costs] --weight-bound --cost-bound [--num-vertices]
  CVP                             --basis, --target-vec [--bounds]
  MultiprocessorScheduling        --lengths, --num-processors, --deadline
  SequencingWithinIntervals       --release-times, --deadlines, --lengths
  OptimalLinearArrangement        --graph, --bound
  MinMaxMulticenter (pCenter)     --graph, --weights, --edge-weights, --k, --bound
  MixedChinesePostman (MCPP)      --graph, --arcs, --edge-weights, --arc-costs, --bound [--num-vertices]
  RuralPostman (RPP)              --graph, --edge-weights, --required-edges, --bound
  StackerCrane                    --arcs, --graph, --arc-costs, --edge-lengths, --bound [--num-vertices]
  MultipleChoiceBranching         --arcs [--weights] --partition --bound [--num-vertices]
  AdditionalKey                   --num-attributes, --dependencies, --relation-attrs [--known-keys]
  ConsistencyOfDatabaseFrequencyTables --num-objects, --attribute-domains, --frequency-tables [--known-values]
  SubgraphIsomorphism             --graph (host), --pattern (pattern)
  LCS                             --strings, --bound [--alphabet-size]
  FAS                             --arcs [--weights] [--num-vertices]
  FVS                             --arcs [--weights] [--num-vertices]
  QBF                             --num-vars, --clauses, --quantifiers
  SteinerTreeInGraphs             --graph, --edge-weights, --terminals
  PartitionIntoPathsOfLength2     --graph
  ResourceConstrainedScheduling   --num-processors, --resource-bounds, --resource-requirements, --deadline
  PartiallyOrderedKnapsack        --sizes, --values, --capacity, --precedences
  QAP                             --matrix (cost), --distance-matrix
  StrongConnectivityAugmentation  --arcs, --candidate-arcs, --bound [--num-vertices]
  FlowShopScheduling              --task-lengths, --deadline [--num-processors]
  StaffScheduling                 --schedules, --requirements, --num-workers, --k
  TimetableDesign                 --num-periods, --num-craftsmen, --num-tasks, --craftsman-avail, --task-avail, --requirements
  MinimumTardinessSequencing      --n, --deadlines [--precedence-pairs]
  RectilinearPictureCompression   --matrix (0/1), --k
  SchedulingWithIndividualDeadlines --n, --num-processors/--m, --deadlines [--precedence-pairs]
  SequencingToMinimizeMaximumCumulativeCost --costs, --bound [--precedence-pairs]
  SequencingToMinimizeWeightedCompletionTime --lengths, --weights [--precedence-pairs]
  SequencingToMinimizeWeightedTardiness --sizes, --weights, --deadlines, --bound
  SCS                             --strings, --bound [--alphabet-size]
  StringToStringCorrection         --source-string, --target-string, --bound [--alphabet-size]
  D2CIF                           --arcs, --capacities, --source-1, --sink-1, --source-2, --sink-2, --requirement-1, --requirement-2
  CBQ                              --domain-size, --relations, --conjuncts-spec
  ILP, CircuitSAT                 (via reduction only)

Geometry graph variants (use slash notation, e.g., MIS/KingsSubgraph):
  KingsSubgraph, TriangularSubgraph   --positions (integer x,y pairs)
  UnitDiskGraph                        --positions (float x,y pairs) [--radius]

Random generation:
  --random --num-vertices N [--edge-prob 0.5] [--seed 42]

Examples:
  pred create --example MIS/SimpleGraph/i32
  pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32
  pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 --example-side target
  pred create MIS --graph 0-1,1-2,2-3 --weights 1,1,1
  pred create SAT --num-vars 3 --clauses \"1,2;-1,3\"
  pred create QUBO --matrix \"1,0.5;0.5,2\"
  pred create GeneralizedHex --graph 0-1,0-2,0-3,1-4,2-4,3-4,4-5 --source 0 --sink 5
  pred create MultipleChoiceBranching/i32 --arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --bound 10
  pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2 | pred solve - --solver brute-force
  pred create MIS/KingsSubgraph --positions \"0,0;1,0;1,1;0,1\"
  pred create MIS/UnitDiskGraph --positions \"0,0;1,0;0.5,0.8\" --radius 1.5
  pred create MIS --random --num-vertices 10 --edge-prob 0.3
  pred create MultiprocessorScheduling --lengths 4,5,3,2,6 --num-processors 2 --deadline 10
  pred create ConsistencyOfDatabaseFrequencyTables --num-objects 6 --attribute-domains \"2,3,2\" --frequency-tables \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\" --known-values \"0,0,0;3,0,1;1,2,1\"
  pred create BiconnectivityAugmentation --graph 0-1,1-2,2-3 --potential-edges 0-2:3,0-3:4,1-3:2 --budget 5
  pred create FVS --arcs \"0>1,1>2,2>0\" --weights 1,1,1
  pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1
  pred create X3C --universe 9 --sets \"0,1,2;0,2,4;3,4,5;3,5,7;6,7,8;1,4,6;2,5,8\"
  pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3
  pred create MinimumCardinalityKey --num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\" --k 2
  pred create PrimeAttributeName --universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3
  pred create TwoDimensionalConsecutiveSets --alphabet-size 6 --sets \"0,1,2;3,4,5;1,3;2,4;0,5\"")]
pub struct CreateArgs {
    /// Problem type (e.g., MIS, QUBO, SAT). Omit when using --example.
    #[arg(value_parser = crate::problem_name::ProblemNameParser)]
    pub problem: Option<String>,
    /// Build a problem from the canonical example database using a structural problem spec.
    #[arg(long, value_parser = crate::problem_name::ProblemNameParser)]
    pub example: Option<String>,
    /// Target problem spec for canonical rule example lookup.
    #[arg(long = "to", value_parser = crate::problem_name::ProblemNameParser)]
    pub example_target: Option<String>,
    /// Which side of a rule example to emit [default: source].
    #[arg(long, value_enum, default_value = "source")]
    pub example_side: ExampleSide,
    /// Graph edge list (e.g., 0-1,1-2,2-3)
    #[arg(long)]
    pub graph: Option<String>,
    /// Vertex weights (e.g., 1,1,1,1) [default: all 1s]
    #[arg(long)]
    pub weights: Option<String>,
    /// Edge weights (e.g., 2,3,1) [default: all 1s]
    #[arg(long)]
    pub edge_weights: Option<String>,
    /// Edge lengths (e.g., 2,3,1) [default: all 1s]
    #[arg(long)]
    pub edge_lengths: Option<String>,
    /// Edge capacities for multicommodity flow problems (e.g., 1,1,2)
    #[arg(long)]
    pub capacities: Option<String>,
    /// Source vertex for path-based graph problems and MinimumCutIntoBoundedSets
    #[arg(long)]
    pub source: Option<usize>,
    /// Sink vertex for path-based graph problems and MinimumCutIntoBoundedSets
    #[arg(long)]
    pub sink: Option<usize>,
    /// Required number of paths for LengthBoundedDisjointPaths
    #[arg(long)]
    pub num_paths_required: Option<usize>,
    /// Pairwise couplings J_ij for SpinGlass (e.g., 1,-1,1) [default: all 1s]
    #[arg(long)]
    pub couplings: Option<String>,
    /// On-site fields h_i for SpinGlass (e.g., 0,0,1) [default: all 0s]
    #[arg(long)]
    pub fields: Option<String>,
    /// Clauses for SAT problems (semicolon-separated, e.g., "1,2;-1,3")
    #[arg(long)]
    pub clauses: Option<String>,
    /// Number of variables (for SAT/KSAT)
    #[arg(long)]
    pub num_vars: Option<usize>,
    /// Matrix input. QUBO uses semicolon-separated numeric rows ("1,0.5;0.5,2");
    /// ConsecutiveBlockMinimization uses a JSON 2D bool array ('[[true,false],[false,true]]')
    #[arg(long)]
    pub matrix: Option<String>,
    /// Shared integer parameter (use `pred create <PROBLEM>` for the problem-specific meaning)
    #[arg(long)]
    pub k: Option<usize>,
    /// Generate a random instance (graph-based problems only)
    #[arg(long)]
    pub random: bool,
    /// Number of vertices for random graph generation
    #[arg(long)]
    pub num_vertices: Option<usize>,
    /// Source vertex for path problems
    #[arg(long)]
    pub source_vertex: Option<usize>,
    /// Target vertex for path problems
    #[arg(long)]
    pub target_vertex: Option<usize>,
    /// Edge probability for random graph generation (0.0 to 1.0) [default: 0.5]
    #[arg(long)]
    pub edge_prob: Option<f64>,
    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,
    /// Target value (for Factoring and SubsetSum)
    #[arg(long)]
    pub target: Option<String>,
    /// Bits for first factor (for Factoring); also accepted as a processor-count alias for scheduling create commands
    #[arg(long)]
    pub m: Option<usize>,
    /// Bits for second factor (for Factoring)
    #[arg(long)]
    pub n: Option<usize>,
    /// Vertex positions for geometry-based graphs (semicolon-separated x,y pairs, e.g., "0,0;1,0;1,1")
    #[arg(long)]
    pub positions: Option<String>,
    /// Radius for UnitDiskGraph [default: 1.0]
    #[arg(long)]
    pub radius: Option<f64>,
    /// Source vertex s_1 for commodity 1
    #[arg(long)]
    pub source_1: Option<usize>,
    /// Sink vertex t_1 for commodity 1
    #[arg(long)]
    pub sink_1: Option<usize>,
    /// Source vertex s_2 for commodity 2
    #[arg(long)]
    pub source_2: Option<usize>,
    /// Sink vertex t_2 for commodity 2
    #[arg(long)]
    pub sink_2: Option<usize>,
    /// Required flow R_1 for commodity 1
    #[arg(long)]
    pub requirement_1: Option<u64>,
    /// Required flow R_2 for commodity 2
    #[arg(long)]
    pub requirement_2: Option<u64>,
    /// Item sizes for BinPacking (comma-separated, e.g., "3,3,2,2")
    #[arg(long)]
    pub sizes: Option<String>,
    /// Bin capacity for BinPacking
    #[arg(long)]
    pub capacity: Option<String>,
    /// Car paint sequence for PaintShop (comma-separated, each label appears exactly twice, e.g., "a,b,a,c,c,b")
    #[arg(long)]
    pub sequence: Option<String>,
    /// Sets for set-system problems such as SetPacking, MinimumHittingSet, and SetCovering (semicolon-separated, e.g., "0,1;1,2;0,2")
    #[arg(long)]
    pub sets: Option<String>,
    /// R-family sets for ComparativeContainment (semicolon-separated, e.g., "0,1;1,2")
    #[arg(long)]
    pub r_sets: Option<String>,
    /// S-family sets for ComparativeContainment (semicolon-separated, e.g., "0,1;1,2")
    #[arg(long)]
    pub s_sets: Option<String>,
    /// R-family weights for ComparativeContainment (comma-separated, e.g., "2,5")
    #[arg(long)]
    pub r_weights: Option<String>,
    /// S-family weights for ComparativeContainment (comma-separated, e.g., "3,6")
    #[arg(long)]
    pub s_weights: Option<String>,
    /// Partition groups for arc-index partitions (semicolon-separated, e.g., "0,1;2,3")
    #[arg(long)]
    pub partition: Option<String>,
    /// Universe size for set-system problems such as MinimumHittingSet, MinimumSetCovering, and ComparativeContainment
    #[arg(long)]
    pub universe: Option<usize>,
    /// Bipartite graph edges for BicliqueCover / BalancedCompleteBipartiteSubgraph (e.g., "0-0,0-1,1-2" for left-right pairs)
    #[arg(long)]
    pub biedges: Option<String>,
    /// Left partition size for BicliqueCover / BalancedCompleteBipartiteSubgraph
    #[arg(long)]
    pub left: Option<usize>,
    /// Right partition size for BicliqueCover / BalancedCompleteBipartiteSubgraph
    #[arg(long)]
    pub right: Option<usize>,
    /// Rank for BMF
    #[arg(long)]
    pub rank: Option<usize>,
    /// Lattice basis for CVP (semicolon-separated column vectors, e.g., "1,0;0,1")
    #[arg(long)]
    pub basis: Option<String>,
    /// Target vector for CVP (comma-separated, e.g., "0.5,0.5")
    #[arg(long)]
    pub target_vec: Option<String>,
    /// Variable bounds for CVP as "lower,upper" (e.g., "-10,10") [default: -10,10]
    #[arg(long, allow_hyphen_values = true)]
    pub bounds: Option<String>,
    /// Release times for SequencingWithinIntervals (comma-separated, e.g., "0,0,5")
    #[arg(long)]
    pub release_times: Option<String>,
    /// Processing lengths (comma-separated, e.g., "4,5,3,2,6")
    #[arg(long)]
    pub lengths: Option<String>,
    /// Terminal vertices for SteinerTree or MinimumMultiwayCut (comma-separated indices, e.g., "0,2,4")
    #[arg(long)]
    pub terminals: Option<String>,
    /// Tree edge list for IsomorphicSpanningTree (e.g., 0-1,1-2,2-3)
    #[arg(long)]
    pub tree: Option<String>,
    /// Required edge indices for RuralPostman (comma-separated, e.g., "0,2,4")
    #[arg(long)]
    pub required_edges: Option<String>,
    /// Upper bound or length bound (for BoundedComponentSpanningForest, LengthBoundedDisjointPaths, LongestCommonSubsequence, MultipleCopyFileAllocation, MultipleChoiceBranching, OptimalLinearArrangement, RuralPostman, ShortestCommonSupersequence, or StringToStringCorrection)
    #[arg(long, allow_hyphen_values = true)]
    pub bound: Option<i64>,
    /// Upper bound on total path length
    #[arg(long)]
    pub length_bound: Option<i32>,
    /// Upper bound on total path weight
    #[arg(long)]
    pub weight_bound: Option<i32>,
    /// Upper bound on total inter-partition arc cost
    #[arg(long)]
    pub cost_bound: Option<i32>,
    /// Pattern graph edge list for SubgraphIsomorphism (e.g., 0-1,1-2,2-0)
    #[arg(long)]
    pub pattern: Option<String>,
    /// Input strings for LCS (e.g., "ABAC;BACA" or "0,1,0;1,0,1") or SCS (e.g., "0,1,2;1,2,0")
    #[arg(long)]
    pub strings: Option<String>,
    /// Task costs for SequencingToMinimizeMaximumCumulativeCost (comma-separated, e.g., "2,-1,3,-2,1,-3")
    #[arg(long, allow_hyphen_values = true)]
    pub costs: Option<String>,
    /// Arc costs for directed graph problems with per-arc costs (comma-separated, e.g., "1,1,2,3")
    #[arg(long)]
    pub arc_costs: Option<String>,
    /// Directed arcs for directed graph problems (e.g., 0>1,1>2,2>0)
    #[arg(long)]
    pub arcs: Option<String>,
    /// Quantifiers for QBF (comma-separated, E=Exists, A=ForAll, e.g., "E,A,E")
    #[arg(long)]
    pub quantifiers: Option<String>,
    /// Size bound for partition sets (for MinimumCutIntoBoundedSets)
    #[arg(long)]
    pub size_bound: Option<usize>,
    /// Cut weight bound (for MinimumCutIntoBoundedSets)
    #[arg(long)]
    pub cut_bound: Option<i32>,
    /// Item values (e.g., 3,4,5,7) for PartiallyOrderedKnapsack
    #[arg(long)]
    pub values: Option<String>,
    /// Precedence pairs (e.g., "0>2,0>3,1>4") for PartiallyOrderedKnapsack
    #[arg(long, alias = "item-precedences")]
    pub precedences: Option<String>,
    /// Distance matrix for QuadraticAssignment (semicolon-separated rows, e.g., "0,1,2;1,0,1;2,1,0")
    #[arg(long)]
    pub distance_matrix: Option<String>,
    /// Weighted potential augmentation edges (e.g., 0-2:3,1-3:5)
    #[arg(long)]
    pub potential_edges: Option<String>,
    /// Total budget for selected potential edges
    #[arg(long)]
    pub budget: Option<String>,
    /// Candidate weighted arcs for StrongConnectivityAugmentation (e.g., 2>0:1,2>1:3)
    #[arg(long)]
    pub candidate_arcs: Option<String>,
    /// Usage frequencies for MultipleCopyFileAllocation (comma-separated, e.g., "5,4,3,2")
    #[arg(long)]
    pub usage: Option<String>,
    /// Storage costs for MultipleCopyFileAllocation (comma-separated, e.g., "1,1,1,1")
    #[arg(long)]
    pub storage: Option<String>,
    /// Deadlines for MinimumTardinessSequencing or SchedulingWithIndividualDeadlines (comma-separated, e.g., "5,5,5,3,3")
    #[arg(long)]
    pub deadlines: Option<String>,
    /// Precedence pairs for MinimumTardinessSequencing, SchedulingWithIndividualDeadlines, or SequencingToMinimizeWeightedCompletionTime (e.g., "0>3,1>3,1>4,2>4")
    #[arg(long)]
    pub precedence_pairs: Option<String>,
    /// Resource bounds for ResourceConstrainedScheduling (comma-separated, e.g., "20,15")
    #[arg(long)]
    pub resource_bounds: Option<String>,
    /// Resource requirements for ResourceConstrainedScheduling (semicolon-separated rows, each row comma-separated, e.g., "6,3;7,4;5,2")
    #[arg(long)]
    pub resource_requirements: Option<String>,
    /// Task lengths for FlowShopScheduling (semicolon-separated rows: "3,4,2;2,3,5;4,1,3")
    #[arg(long)]
    pub task_lengths: Option<String>,
    /// Deadline for FlowShopScheduling, MultiprocessorScheduling, or ResourceConstrainedScheduling
    #[arg(long)]
    pub deadline: Option<u64>,
    /// Number of processors/machines for FlowShopScheduling, MultiprocessorScheduling, ResourceConstrainedScheduling, or SchedulingWithIndividualDeadlines
    #[arg(long)]
    pub num_processors: Option<usize>,
    /// Binary schedule patterns for StaffScheduling (semicolon-separated rows, e.g., "1,1,0;0,1,1")
    #[arg(long)]
    pub schedules: Option<String>,
    /// Requirements for StaffScheduling (comma-separated) or TimetableDesign (semicolon-separated rows)
    #[arg(long)]
    pub requirements: Option<String>,
    /// Number of available workers for StaffScheduling
    #[arg(long)]
    pub num_workers: Option<u64>,
    /// Number of work periods for TimetableDesign
    #[arg(long)]
    pub num_periods: Option<usize>,
    /// Number of craftsmen for TimetableDesign
    #[arg(long)]
    pub num_craftsmen: Option<usize>,
    /// Number of tasks for TimetableDesign
    #[arg(long)]
    pub num_tasks: Option<usize>,
    /// Craftsman availability rows for TimetableDesign (semicolon-separated 0/1 rows)
    #[arg(long)]
    pub craftsman_avail: Option<String>,
    /// Task availability rows for TimetableDesign (semicolon-separated 0/1 rows)
    #[arg(long)]
    pub task_avail: Option<String>,
    /// Alphabet size for LCS, SCS, StringToStringCorrection, or TwoDimensionalConsecutiveSets (optional; inferred from the input strings if omitted)
    #[arg(long)]
    pub alphabet_size: Option<usize>,

    /// Number of attributes for AdditionalKey or MinimumCardinalityKey
    #[arg(long)]
    pub num_attributes: Option<usize>,
    /// Functional dependencies for AdditionalKey (e.g., "0,1:2,3;2,3:4,5") or MinimumCardinalityKey (semicolon-separated "lhs>rhs" pairs, e.g., "0,1>2;0,2>3")
    #[arg(long)]
    pub dependencies: Option<String>,
    /// Relation scheme attributes for AdditionalKey (comma-separated, e.g., "0,1,2,3,4,5")
    #[arg(long)]
    pub relation_attrs: Option<String>,
    /// Known candidate keys for AdditionalKey (e.g., "0,1;2,3")
    #[arg(long)]
    pub known_keys: Option<String>,
    /// Number of objects for ConsistencyOfDatabaseFrequencyTables
    #[arg(long)]
    pub num_objects: Option<usize>,
    /// Attribute-domain sizes for ConsistencyOfDatabaseFrequencyTables (comma-separated, e.g., "2,3,2")
    #[arg(long)]
    pub attribute_domains: Option<String>,
    /// Pairwise frequency tables for ConsistencyOfDatabaseFrequencyTables (e.g., "0,1:1,1|0,1;1,2:1,0|0,1")
    #[arg(long)]
    pub frequency_tables: Option<String>,
    /// Known value triples for ConsistencyOfDatabaseFrequencyTables (e.g., "0,0,0;3,1,2")
    #[arg(long)]
    pub known_values: Option<String>,
    /// Domain size for ConjunctiveBooleanQuery
    #[arg(long)]
    pub domain_size: Option<usize>,
    /// Relations for ConjunctiveBooleanQuery (format: "arity:tuple1|tuple2;arity:tuple1|tuple2")
    #[arg(long)]
    pub relations: Option<String>,
    /// Conjuncts for ConjunctiveBooleanQuery (format: "rel:args;rel:args" where args use v0,v1 for variables, c0,c1 for constants)
    #[arg(long)]
    pub conjuncts_spec: Option<String>,
    /// Functional dependencies (semicolon-separated, each dep is lhs>rhs with comma-separated indices, e.g., "0,1>2,3;2,3>0,1")
    #[arg(long)]
    pub deps: Option<String>,
    /// Query attribute index for PrimeAttributeName
    #[arg(long)]
    pub query: Option<usize>,
    /// Number of groups for SumOfSquaresPartition
    #[arg(long)]
    pub num_groups: Option<usize>,
    /// Source string for StringToStringCorrection (comma-separated symbol indices, e.g., "0,1,2,3")
    #[arg(long)]
    pub source_string: Option<String>,
    /// Target string for StringToStringCorrection (comma-separated symbol indices, e.g., "0,1,3,2")
    #[arg(long)]
    pub target_string: Option<String>,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred solve problem.json                        # ILP solver (default, auto-reduces to ILP)
  pred solve problem.json --solver brute-force   # brute-force (exhaustive search)
  pred solve reduced.json                        # solve a reduction bundle
  pred solve reduced.json -o solution.json       # save result to file
  pred create MIS --graph 0-1,1-2 | pred solve - # read from stdin when an ILP path exists
  pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2 | pred solve - --solver brute-force
  pred create TwoDimensionalConsecutiveSets --alphabet-size 6 --sets \"0,1,2;3,4,5;1,3;2,4;0,5\" | pred solve - --solver brute-force
  pred solve problem.json --timeout 10           # abort after 10 seconds

Typical workflow:
  pred create MIS --graph 0-1,1-2,2-3 -o problem.json
  pred solve problem.json

Solve via explicit reduction:
  pred reduce problem.json --to QUBO -o reduced.json
  pred solve reduced.json

Input: a problem JSON from `pred create`, or a reduction bundle from `pred reduce`.
When given a bundle, the target is solved and the solution is mapped back to the source.
The ILP solver auto-reduces non-ILP problems before solving.
Problems without an ILP reduction path, such as `LengthBoundedDisjointPaths`,
`MinMaxMulticenter`, and `StringToStringCorrection`, currently need `--solver brute-force`.

ILP backend (default: HiGHS). To use a different backend:
  cargo install problemreductions-cli --features coin-cbc
  cargo install problemreductions-cli --features scip
  cargo install problemreductions-cli --no-default-features --features clarabel")]
pub struct SolveArgs {
    /// Problem JSON file (from `pred create`) or reduction bundle (from `pred reduce`). Use - for stdin.
    pub input: PathBuf,
    /// Solver: ilp (default) or brute-force
    #[arg(long, default_value = "ilp")]
    pub solver: String,
    /// Timeout in seconds (0 = no limit)
    #[arg(long, default_value = "0")]
    pub timeout: u64,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred reduce problem.json --to QUBO -o reduced.json
  pred reduce problem.json --to ILP -o reduced.json
  pred reduce problem.json --via path.json -o reduced.json
  pred create MIS --graph 0-1,1-2 | pred reduce - --to QUBO  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.
The --via path file is from `pred path <SRC> <DST> -o path.json`.
When --via is given, --to is inferred from the path file.
Output is a reduction bundle with source, target, and path.
Use `pred solve reduced.json` to solve and map the solution back.")]
pub struct ReduceArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Target problem type (e.g., QUBO, SpinGlass). Inferred from --via if omitted.
    #[arg(long, value_parser = crate::problem_name::ProblemNameParser)]
    pub to: Option<String>,
    /// Reduction route file (from `pred path ... -o`)
    #[arg(long)]
    pub via: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct InspectArgs {
    /// Problem JSON file or reduction bundle. Use - for stdin.
    pub input: PathBuf,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred evaluate problem.json --config 1,0,1,0
  pred evaluate problem.json --config 1,0,1,0 -o result.json
  pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.")]
pub struct EvaluateArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1,0)
    #[arg(long)]
    pub config: String,
}

/// Print the after_help text for a subcommand on parse error.
///
/// Only matches the first line of the error message. Without this,
/// bare `pred` (no subcommand) would match "pred solve" in the
/// top-level workflow examples and incorrectly append the solve
/// subcommand's help text.
pub fn print_subcommand_help_hint(error_msg: &str) {
    let first_line = error_msg.lines().next().unwrap_or("");
    let subcmds = [
        ("pred solve", "solve"),
        ("pred reduce", "reduce"),
        ("pred create", "create"),
        ("pred evaluate", "evaluate"),
        ("pred inspect", "inspect"),
        ("pred path", "path"),
        ("pred show", "show"),
        ("pred to", "to"),
        ("pred from", "from"),
        ("pred export-graph", "export-graph"),
    ];
    let cmd = Cli::command();
    for (pattern, name) in subcmds {
        if first_line.contains(pattern) {
            if let Some(sub) = cmd.find_subcommand(name) {
                if let Some(help) = sub.get_after_help() {
                    eprintln!("\n{help}");
                }
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_create_help_mentions_scheduling_with_individual_deadlines_shared_flags() {
        let mut cmd = Cli::command();
        let create = cmd
            .find_subcommand_mut("create")
            .expect("create subcommand");
        let mut help = Vec::new();
        create
            .write_long_help(&mut help)
            .expect("render create help");
        let help = String::from_utf8(help).expect("utf8 help");

        assert!(help.contains(
            "Deadlines for MinimumTardinessSequencing or SchedulingWithIndividualDeadlines"
        ));
        assert!(help.contains(
            "Precedence pairs for MinimumTardinessSequencing, SchedulingWithIndividualDeadlines, or SequencingToMinimizeWeightedCompletionTime"
        ));
        assert!(
            help.contains(
                "Number of processors/machines for FlowShopScheduling, MultiprocessorScheduling, ResourceConstrainedScheduling, or SchedulingWithIndividualDeadlines"
            ),
            "create help should describe --num-processors for both scheduling models"
        );
        assert!(help.contains(
            "SchedulingWithIndividualDeadlines --n, --num-processors/--m, --deadlines [--precedence-pairs]"
        ));
    }

    #[test]
    fn test_create_parses_biconnectivity_augmentation_flags() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "BiconnectivityAugmentation",
            "--graph",
            "0-1,1-2",
            "--potential-edges",
            "0-2:3,1-3:5",
            "--budget",
            "7",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        assert_eq!(args.problem.as_deref(), Some("BiconnectivityAugmentation"));
        assert_eq!(args.graph.as_deref(), Some("0-1,1-2"));
        assert_eq!(args.potential_edges.as_deref(), Some("0-2:3,1-3:5"));
        assert_eq!(args.budget.as_deref(), Some("7"));
    }

    #[test]
    fn test_create_help_mentions_biconnectivity_augmentation_flags() {
        let cmd = Cli::command();
        let create = cmd.find_subcommand("create").expect("create subcommand");
        let help = create
            .get_after_help()
            .expect("create after_help")
            .to_string();

        assert!(help.contains("BiconnectivityAugmentation"));
        assert!(help.contains("--potential-edges"));
        assert!(help.contains("--budget"));
    }

    #[test]
    fn test_create_help_mentions_stacker_crane_flags() {
        let cmd = Cli::command();
        let create = cmd.find_subcommand("create").expect("create subcommand");
        let help = create
            .get_after_help()
            .expect("create after_help")
            .to_string();

        assert!(help.contains("StackerCrane"));
        assert!(help.contains("--arcs"));
        assert!(help.contains("--graph"));
        assert!(help.contains("--arc-costs"));
        assert!(help.contains("--edge-lengths"));
        assert!(help.contains("--bound"));
        assert!(help.contains("--num-vertices"));
    }
}
