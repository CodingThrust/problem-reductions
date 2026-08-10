use clap::parser::ValueSource;
use clap::{Arg, ArgAction, ArgMatches, Args, Command, Error, FromArgMatches};
use problemreductions::registry::{problem_types, variant_entries, ProblemType};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::str::FromStr;

use crate::cli::ExampleSide;

const EXAMPLE: &str = "example";
const EXAMPLE_TARGET: &str = "to";
const EXAMPLE_SIDE: &str = "example-side";

thread_local! {
    static STATIC_COMPLETION_BUILD: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn with_static_completion_schema<T>(build: impl FnOnce() -> T) -> T {
    STATIC_COMPLETION_BUILD.set(true);
    let result = build();
    STATIC_COMPLETION_BUILD.set(false);
    result
}

#[derive(Debug, Clone)]
pub struct CreateArgs {
    pub problem: Option<String>,
    pub example: Option<String>,
    pub example_target: Option<String>,
    pub example_side: ExampleSide,
    values: BTreeMap<String, String>,
}

impl CreateArgs {
    pub fn raw(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    pub fn has(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn value<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        self.raw(name).map(|raw| {
            raw.parse::<T>()
                .unwrap_or_else(|error| panic!("invalid value for --{name}: {error}"))
        })
    }

    #[cfg(test)]
    pub fn for_test(problem: &str) -> Self {
        Self {
            problem: Some(problem.to_string()),
            example: None,
            example_target: None,
            example_side: ExampleSide::Source,
            values: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub fn insert(&mut self, name: &str, value: impl ToString) {
        self.values.insert(name.to_string(), value.to_string());
    }
}

impl Args for CreateArgs {
    fn augment_args(command: Command) -> Command {
        command
            .subcommand_required(false)
            .arg(Arg::new(EXAMPLE).long(EXAMPLE).value_name("PROBLEM_SPEC"))
            .arg(
                Arg::new(EXAMPLE_TARGET)
                    .long(EXAMPLE_TARGET)
                    .value_name("TARGET_SPEC")
                    .requires(EXAMPLE),
            )
            .arg(
                Arg::new(EXAMPLE_SIDE)
                    .long(EXAMPLE_SIDE)
                    .value_parser(clap::builder::EnumValueParser::<ExampleSide>::new())
                    .default_value("source"),
            )
            .defer(add_problem_subcommands)
    }

    fn augment_args_for_update(command: Command) -> Command {
        Self::augment_args(command)
    }
}

impl FromArgMatches for CreateArgs {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let example = matches.get_one::<String>(EXAMPLE).cloned();
        let example_target = matches.get_one::<String>(EXAMPLE_TARGET).cloned();
        let example_side = matches
            .get_one::<ExampleSide>(EXAMPLE_SIDE)
            .expect("--example-side has a default value")
            .clone();

        let (problem, values) = if let Some((problem, problem_matches)) = matches.subcommand() {
            let values = problem_matches
                .ids()
                .filter_map(|id| {
                    if problem_matches.value_source(id.as_str()) != Some(ValueSource::CommandLine) {
                        return None;
                    }
                    problem_matches
                        .get_raw(id.as_str())
                        .and_then(|mut values| values.next())
                        .map(|value| (id.to_string(), os_value(value)))
                })
                .collect();
            (Some(problem.to_string()), values)
        } else {
            (None, BTreeMap::new())
        };

        Ok(Self {
            problem,
            example,
            example_target,
            example_side,
            values,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

fn os_value(value: &OsStr) -> String {
    value
        .to_str()
        .expect("Clap accepted a non-UTF-8 create argument")
        .to_string()
}

fn add_problem_subcommands(mut command: Command) -> Command {
    let include_problem_flags = !STATIC_COMPLETION_BUILD.get();
    let problems = problem_types();
    let entries = variant_entries();
    let canonical_names = problems
        .iter()
        .map(|problem| problem.canonical_name)
        .collect::<BTreeSet<_>>();
    for problem in problems {
        for variant in variants_for(&problem, &entries) {
            let names = names_for_variant(&problem, &variant, &entries, &canonical_names);
            let Some((name, aliases)) = names.split_first() else {
                continue;
            };
            let canonical = problem.canonical_name.to_string();
            let variant_spec = name.clone();
            let mut subcommand = Command::new(name.clone())
                .about(problem.description)
                .aliases(aliases.iter().cloned())
                .arg_required_else_help(true)
                .disable_help_subcommand(true);
            if include_problem_flags {
                subcommand = subcommand.defer(add_selected_problem_args);
            }
            command = command.subcommand(
                subcommand.long_about(format!("Create a {canonical} instance ({variant_spec})")),
            );
        }
    }
    command
}

fn variants_for(
    problem: &ProblemType,
    entries: &[&problemreductions::registry::VariantEntry],
) -> Vec<BTreeMap<String, String>> {
    let mut variants = entries
        .iter()
        .filter(|entry| entry.name == problem.canonical_name)
        .map(|entry| entry.variant_map())
        .collect::<Vec<_>>();
    variants.sort();
    variants.dedup();
    variants
}

fn names_for_variant(
    problem: &ProblemType,
    variant: &BTreeMap<String, String>,
    entries: &[&problemreductions::registry::VariantEntry],
    canonical_names: &BTreeSet<&str>,
) -> Vec<String> {
    let mut prefixes = vec![problem.canonical_name];
    prefixes.extend(
        problem
            .aliases
            .iter()
            .copied()
            .filter(|alias| !is_other_problem_name(problem.canonical_name, alias, canonical_names)),
    );

    let non_default = problem
        .dimensions
        .iter()
        .filter(|dimension| {
            dimension_value(variant, dimension.key, dimension.default_value)
                != dimension.default_value
        })
        .collect::<Vec<_>>();
    let mut names = BTreeSet::new();
    for prefix in prefixes {
        let suffix = non_default
            .iter()
            .map(|dimension| dimension_value(variant, dimension.key, dimension.default_value))
            .collect::<Vec<_>>();
        names.insert(join_spec(prefix, &suffix));

        let full = problem
            .dimensions
            .iter()
            .map(|dimension| dimension_value(variant, dimension.key, dimension.default_value))
            .collect::<Vec<_>>();
        names.insert(join_spec(prefix, &full));
    }

    for entry in entries
        .iter()
        .filter(|entry| entry.name == problem.canonical_name && entry.variant_map() == *variant)
    {
        names.extend(
            entry
                .aliases
                .iter()
                .filter(|alias| {
                    !is_other_problem_name(problem.canonical_name, alias, canonical_names)
                })
                .map(|alias| (*alias).to_string()),
        );
    }

    let canonical = join_spec(
        problem.canonical_name,
        &non_default
            .iter()
            .map(|dimension| dimension_value(variant, dimension.key, dimension.default_value))
            .collect::<Vec<_>>(),
    );
    let mut names = names.into_iter().collect::<Vec<_>>();
    names.sort();
    let position = names
        .iter()
        .position(|name| name == &canonical)
        .expect("canonical create command name");
    names.swap(0, position);
    names
}

fn is_other_problem_name(
    canonical: &str,
    candidate: &str,
    canonical_names: &BTreeSet<&str>,
) -> bool {
    canonical_names
        .iter()
        .any(|name| *name != canonical && name.eq_ignore_ascii_case(candidate))
}

fn dimension_value<'a>(
    variant: &'a BTreeMap<String, String>,
    key: &str,
    default: &'a str,
) -> &'a str {
    variant.get(key).map(String::as_str).unwrap_or(default)
}

fn join_spec(prefix: &str, values: &[&str]) -> String {
    if values.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}/{}", values.join("/"))
    }
}

fn add_selected_problem_args(mut command: Command) -> Command {
    let selected = command.get_name().to_string();
    let problem_ref = problemreductions::registry::parse_catalog_problem_ref(&selected)
        .unwrap_or_else(|error| panic!("invalid registered create command `{selected}`: {error}"));
    let inputs =
        crate::commands::create::create_inputs_for(problem_ref.name(), problem_ref.variant());

    for input in inputs {
        let mut arg = Arg::new(input.name.clone()).long(input.name.clone());
        if input.kind == crate::commands::create::InputValueKind::Bool {
            arg = arg.action(ArgAction::SetTrue);
        } else {
            arg = arg
                .action(ArgAction::Set)
                .value_name("VALUE")
                .allow_hyphen_values(true);
            arg = add_value_parser(arg, input.kind);
        }
        command = command.arg(arg);
    }
    command
}

fn add_value_parser(arg: Arg, kind: crate::commands::create::InputValueKind) -> Arg {
    use crate::commands::create::InputValueKind;
    match kind {
        InputValueKind::Usize => arg.value_parser(clap::value_parser!(usize)),
        InputValueKind::U64 => arg.value_parser(clap::value_parser!(u64)),
        InputValueKind::I32 => arg.value_parser(clap::value_parser!(i32)),
        InputValueKind::I64 => arg.value_parser(clap::value_parser!(i64)),
        InputValueKind::F64 => arg.value_parser(clap::value_parser!(f64)),
        InputValueKind::Text => arg,
        InputValueKind::Bool => unreachable!("boolean inputs use SetTrue"),
    }
}
