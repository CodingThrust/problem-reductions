use clap::parser::ValueSource;
use clap::{Arg, ArgAction, ArgMatches, Args, Command, Error, FromArgMatches};
use problemreductions::registry::{variant_entries, ProblemType};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::str::FromStr;

use crate::cli::ExampleSide;

const EXAMPLE: &str = "example";
const EXAMPLE_TARGET: &str = "to";
const EXAMPLE_SIDE: &str = "example-side";

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
            .allow_external_subcommands(true)
            .subcommand_value_name("PROBLEM_SPEC")
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

fn add_selected_problem_args(
    mut command: Command,
    canonical: &str,
    variant: &BTreeMap<String, String>,
) -> Command {
    let inputs = crate::commands::create::create_inputs_for(canonical, variant);

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

pub(crate) fn command_for_selected_problem(
    mut command: Command,
    selected: &str,
) -> Result<Command, Error> {
    let spec = crate::problem_name::parse_problem_spec(selected)
        .map_err(|error| invalid_problem_spec(&command, error.to_string()))?;
    let problem = problemreductions::registry::find_problem_type(&spec.name).ok_or_else(|| {
        invalid_problem_spec(
            &command,
            crate::problem_name::unknown_problem_error(&spec.name),
        )
    })?;
    let problem_ref =
        problemreductions::registry::ProblemRef::from_values(&problem, &spec.variant_values)
            .map_err(|error| invalid_problem_spec(&command, error))?;
    if problemreductions::registry::find_variant_entry(problem_ref.name(), problem_ref.variant())
        .is_none()
    {
        return Err(invalid_problem_spec(
            &command,
            format!(
                "No concrete variant is registered for {} with {:?}",
                problem_ref.name(),
                problem_ref.variant()
            ),
        ));
    }

    let canonical_spec = canonical_problem_spec(&problem, problem_ref.variant());
    let mut selected_command = Command::new(canonical_spec.clone())
        .about(problem.description)
        .long_about(format!(
            "Create a {} instance ({canonical_spec})",
            problem.canonical_name
        ))
        .disable_help_subcommand(true);
    if selected != canonical_spec {
        selected_command = selected_command.alias(selected.to_string());
    }
    selected_command = add_selected_problem_args(
        selected_command,
        problem.canonical_name,
        problem_ref.variant(),
    );

    let create = command
        .find_subcommand_mut("create")
        .expect("Cli has a create subcommand");
    *create = std::mem::take(create)
        .allow_external_subcommands(false)
        .subcommand(selected_command);
    Ok(command)
}

fn invalid_problem_spec(command: &Command, message: String) -> Error {
    command
        .clone()
        .error(clap::error::ErrorKind::InvalidSubcommand, message)
}

fn canonical_problem_spec(problem: &ProblemType, variant: &BTreeMap<String, String>) -> String {
    let values = problem
        .dimensions
        .iter()
        .filter_map(|dimension| {
            let value = dimension_value(variant, dimension.key, dimension.default_value);
            (value != dimension.default_value).then_some(value)
        })
        .collect::<Vec<_>>();
    join_spec(problem.canonical_name, &values)
}

pub(crate) fn resolve_registered_create_variant(
    selected: &str,
) -> (&'static str, BTreeMap<String, String>) {
    let mut parts = selected.split('/');
    let canonical = parts.next().expect("registered command has a name");
    let problem = problemreductions::registry::find_problem_type(canonical)
        .unwrap_or_else(|| panic!("missing schema for registered create command `{selected}`"));
    let values = parts.collect::<Vec<_>>();

    if values.is_empty() {
        return variant_entries()
            .into_iter()
            .find(|entry| entry.name == canonical && entry.is_default)
            .map(|entry| (problem.canonical_name, entry.variant_map()))
            .unwrap_or_else(|| panic!("missing default variant for `{canonical}`"));
    }

    let problem_ref = problemreductions::registry::ProblemRef::from_values(&problem, values)
        .unwrap_or_else(|error| panic!("invalid registered create command `{selected}`: {error}"));
    (problem.canonical_name, problem_ref.variant().clone())
}

fn add_value_parser(arg: Arg, kind: crate::commands::create::InputValueKind) -> Arg {
    use crate::commands::create::InputValueKind;
    match kind {
        InputValueKind::Usize => arg.value_parser(clap::value_parser!(usize)),
        InputValueKind::I64 => arg.value_parser(clap::value_parser!(i64)),
        InputValueKind::F64 => arg.value_parser(clap::value_parser!(f64)),
        InputValueKind::Text => arg,
        InputValueKind::Bool => unreachable!("boolean inputs use SetTrue"),
    }
}
