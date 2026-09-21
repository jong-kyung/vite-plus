use semver::Version;
use vp_pm_cli_macros::pm_args;

use crate::resolution::{
    Bun, CommandBuilder, CommandResolution, DiagnosticKind, Diagnostics, Npm,
    PackageManagerDialect, Pnpm, Resolve, Yarn,
};

/// Configuration subcommands.
#[pm_args]
#[derive(clap::Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum ConfigCommand {
    /// List all configuration
    List {
        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Use global config
        #[arg(short = 'g', long)]
        global: bool,

        /// Config location: project, user, or global
        #[arg(long, value_name = "LOCATION")]
        location: Option<String>,
    },

    /// Get configuration value
    Get {
        /// Config key
        key: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Use global config
        #[arg(short = 'g', long)]
        global: bool,

        /// Config location
        #[arg(long, value_name = "LOCATION")]
        location: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Config key
        key: String,

        /// Config value
        value: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Use global config
        #[arg(short = 'g', long)]
        global: bool,

        /// Config location
        #[arg(long, value_name = "LOCATION")]
        location: Option<String>,
    },

    /// Delete configuration key
    Delete {
        /// Config key
        key: String,

        /// Use global config
        #[arg(short = 'g', long)]
        global: bool,

        /// Config location
        #[arg(long, value_name = "LOCATION")]
        location: Option<String>,
    },
}

impl Resolve<ConfigCommand> for Pnpm {
    fn resolve(&self, args: &ConfigCommand, _diag: &mut Diagnostics) -> CommandResolution {
        resolve_npm_like_config("pnpm", args)
    }
}

impl Resolve<ConfigCommand> for Npm {
    fn resolve(&self, args: &ConfigCommand, _diag: &mut Diagnostics) -> CommandResolution {
        resolve_npm_like_config("npm", args)
    }
}

impl Resolve<ConfigCommand> for Yarn {
    fn diagnose(&self, args: &ConfigCommand, diag: &mut Diagnostics) {
        let Some(location) = args.effective_location() else {
            return;
        };
        // Classic's set/delete use saveHomeConfig, so user scope needs no flag.
        // https://github.com/yarnpkg/yarn/blob/v1.22.22/src/cli/commands/config.js#L50-L80
        let supported =
            matches!(location, "user" | "global") || (self.is_berry() && location == "project");
        if !supported {
            let manager = if self.is_berry() { "yarn >= 2" } else { "Yarn Classic" };
            diag.warn(
                DiagnosticKind::UnsupportedOption,
                vt_str::format!("{manager} does not support --location {location}."),
            );
        }
        // Yarn 2.2 added config set --home.
        // https://github.com/yarnpkg/berry/blob/01586a88806a2bebd7edb28d1bee3581b1fd3762/CHANGELOG.md#220
        if self.is_berry()
            && location == "user"
            && matches!(args, ConfigCommand::Set { .. })
            && self.version().is_some_and(|version| version < &Version::new(2, 2, 0))
        {
            diag.warn(
                DiagnosticKind::UnsupportedOption,
                "yarn < 2.2 does not support --location user for config set.",
            );
        }
        // Berry introduced config unset in Yarn 3.
        // https://github.com/yarnpkg/berry/blob/01586a88806a2bebd7edb28d1bee3581b1fd3762/CHANGELOG.md#300
        if self.is_berry()
            && location == "user"
            && matches!(args, ConfigCommand::Delete { .. })
            && self.version().is_some_and(|version| version < &Version::new(3, 0, 0))
        {
            diag.warn(
                DiagnosticKind::UnsupportedOption,
                "yarn < 3 does not support --location user for config delete.",
            );
        }
    }

    fn resolve(&self, args: &ConfigCommand, _diag: &mut Diagnostics) -> CommandResolution {
        resolve_yarn_config(args, self.is_berry())
    }
}

impl Resolve<ConfigCommand> for Bun {
    fn resolve(&self, args: &ConfigCommand, diag: &mut Diagnostics) -> CommandResolution {
        diag.warn(
            DiagnosticKind::FallbackCommand,
            "bun uses bunfig.toml for configuration, not a config command. Falling back to npm config.",
        );
        resolve_npm_like_config("bun", args)
    }
}

fn resolve_npm_like_config(program: &str, args: &ConfigCommand) -> CommandResolution {
    let mut cmd = CommandBuilder::new(program);
    cmd.arg("config").arg(args.subcommand_name());
    append_key_value(&mut cmd, args);
    cmd.arg_if("--json", args.json());
    if let Some(location) = args.effective_location() {
        cmd.arg("--location").arg(location);
    }
    cmd.into()
}

fn resolve_yarn_config(args: &ConfigCommand, is_berry: bool) -> CommandResolution {
    let mut cmd = CommandBuilder::new("yarn");
    cmd.arg("config");
    match (args, is_berry) {
        (ConfigCommand::Delete { .. }, true) => {
            cmd.arg("unset");
        }
        (ConfigCommand::List { .. }, true) => {}
        _ => {
            cmd.arg(args.subcommand_name());
        }
    }
    append_key_value(&mut cmd, args);
    cmd.arg_if("--json", args.json());
    // Reads keep the merged configuration, as npm and pnpm do for user scope.
    let user_write = is_berry
        && args.effective_location() == Some("user")
        && matches!(args, ConfigCommand::Set { .. } | ConfigCommand::Delete { .. });
    if args.effective_location() == Some("global") || user_write {
        cmd.arg(if is_berry { "--home" } else { "--global" });
    }
    cmd.into()
}

fn append_key_value(cmd: &mut CommandBuilder, command: &ConfigCommand) {
    if let Some(key) = command.key() {
        cmd.arg(key);
    }
    if let Some(value) = command.value() {
        cmd.arg(value);
    }
}

impl ConfigCommand {
    fn subcommand_name(&self) -> &'static str {
        match self {
            Self::List { .. } => "list",
            Self::Get { .. } => "get",
            Self::Set { .. } => "set",
            Self::Delete { .. } => "delete",
        }
    }

    fn key(&self) -> Option<&str> {
        match self {
            Self::List { .. } => None,
            Self::Get { key, .. } | Self::Set { key, .. } | Self::Delete { key, .. } => Some(key),
        }
    }

    fn value(&self) -> Option<&str> {
        match self {
            Self::Set { value, .. } => Some(value),
            Self::List { .. } | Self::Get { .. } | Self::Delete { .. } => None,
        }
    }

    fn json(&self) -> bool {
        match self {
            Self::List { json, .. } | Self::Get { json, .. } | Self::Set { json, .. } => *json,
            Self::Delete { .. } => false,
        }
    }

    fn effective_location(&self) -> Option<&str> {
        match self {
            Self::List { global, location, .. }
            | Self::Get { global, location, .. }
            | Self::Set { global, location, .. }
            | Self::Delete { global, location, .. } => {
                if *global {
                    Some("global")
                } else {
                    location.as_deref()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolution::{
        Resolution, resolve,
        test_utils::{bun, expect_run, expect_unsupported, npm, parse_subcommand, pnpm, yarn},
    };

    fn set_config(location: Option<&str>) -> ConfigCommand {
        ConfigCommand::Set {
            key: "registry".to_string(),
            value: "https://registry.npmjs.org".to_string(),
            json: false,
            global: false,
            location: location.map(ToString::to_string),
        }
    }

    #[test]
    fn test_parser_accepts_global_short_flag() {
        let args = parse_subcommand::<ConfigCommand>(["get", "registry", "-g"]).unwrap();

        assert_eq!(
            args,
            ConfigCommand::Get {
                key: "registry".to_string(),
                json: false,
                global: true,
                location: None,
            }
        );
    }

    #[test]
    fn test_pnpm_config_set() {
        let command = expect_run(resolve(&pnpm("10.0.0"), set_config(None)).outcome);

        assert_eq!(command.program, "pnpm");
        assert_eq!(command.args, vec!["config", "set", "registry", "https://registry.npmjs.org"]);
    }

    #[test]
    fn test_npm_config_set() {
        let command = expect_run(resolve(&npm("11.0.0"), set_config(None)).outcome);

        assert_eq!(command.program, "npm");
        assert_eq!(command.args, vec!["config", "set", "registry", "https://registry.npmjs.org"]);
    }

    #[test]
    fn test_config_set_with_json() {
        let command = expect_run(
            resolve(
                &pnpm("10.0.0"),
                ConfigCommand::Set {
                    key: "registry".to_string(),
                    value: "https://registry.npmjs.org".to_string(),
                    json: true,
                    global: false,
                    location: None,
                },
            )
            .outcome,
        );

        assert_eq!(command.program, "pnpm");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--json"]
        );
    }

    #[test]
    fn test_config_set_with_location_global() {
        let command = expect_run(resolve(&pnpm("10.0.0"), set_config(Some("global"))).outcome);

        assert_eq!(command.program, "pnpm");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--location", "global"]
        );
    }

    #[test]
    fn test_yarn2_config_set_location_global() {
        let command = expect_run(resolve(&yarn("4.0.0"), set_config(Some("global"))).outcome);

        assert_eq!(command.program, "yarn");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--home"]
        );
    }

    #[test]
    fn test_yarn1_config_set() {
        let command = expect_run(resolve(&yarn("1.22.0"), set_config(None)).outcome);

        assert_eq!(command.program, "yarn");
        assert_eq!(command.args, vec!["config", "set", "registry", "https://registry.npmjs.org"]);
    }

    #[test]
    fn test_pnpm_config_set_global() {
        let command = expect_run(
            resolve(
                &pnpm("10.0.0"),
                ConfigCommand::Set {
                    key: "registry".to_string(),
                    value: "https://registry.npmjs.org".to_string(),
                    json: false,
                    global: true,
                    location: None,
                },
            )
            .outcome,
        );

        assert_eq!(command.program, "pnpm");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--location", "global"]
        );
    }

    #[test]
    fn test_npm_config_set_global() {
        let command = expect_run(resolve(&npm("11.0.0"), set_config(Some("global"))).outcome);

        assert_eq!(command.program, "npm");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--location", "global"]
        );
    }

    #[test]
    fn test_yarn1_config_set_global() {
        let command = expect_run(resolve(&yarn("1.22.0"), set_config(Some("global"))).outcome);

        assert_eq!(command.program, "yarn");
        assert_eq!(
            command.args,
            vec!["config", "set", "registry", "https://registry.npmjs.org", "--global"]
        );
    }

    #[test]
    fn test_pnpm_config_get() {
        let command = expect_run(
            resolve(
                &pnpm("10.0.0"),
                ConfigCommand::Get {
                    key: "registry".to_string(),
                    json: false,
                    global: false,
                    location: None,
                },
            )
            .outcome,
        );

        assert_eq!(command.program, "pnpm");
        assert_eq!(command.args, vec!["config", "get", "registry"]);
    }

    #[test]
    fn test_npm_config_delete() {
        let command = expect_run(
            resolve(
                &npm("11.0.0"),
                ConfigCommand::Delete {
                    key: "registry".to_string(),
                    global: false,
                    location: None,
                },
            )
            .outcome,
        );

        assert_eq!(command.program, "npm");
        assert_eq!(command.args, vec!["config", "delete", "registry"]);
    }

    #[test]
    fn test_yarn2_config_delete() {
        let command = expect_run(
            resolve(
                &yarn("4.0.0"),
                ConfigCommand::Delete {
                    key: "registry".to_string(),
                    global: false,
                    location: None,
                },
            )
            .outcome,
        );

        assert_eq!(command.program, "yarn");
        assert_eq!(command.args, vec!["config", "unset", "registry"]);
    }

    #[test]
    fn test_yarn2_config_list() {
        let command = expect_run(
            resolve(
                &yarn("4.0.0"),
                ConfigCommand::List { json: false, global: false, location: None },
            )
            .outcome,
        );

        assert_eq!(command.program, "yarn");
        assert_eq!(command.args, vec!["config"]);
    }

    #[test]
    fn test_yarn1_rejects_unsupported_locations() {
        for version in ["1.22.19", "1.22.22"] {
            for location in ["project", "unknown", ""] {
                for command in [
                    vec!["list"],
                    vec!["get", "registry"],
                    vec!["set", "registry", "https://registry.npmjs.org"],
                    vec!["delete", "registry"],
                ] {
                    let args = parse_subcommand::<ConfigCommand>(
                        command.into_iter().chain(["--location", location]),
                    )
                    .unwrap();
                    expect_unsupported(
                        resolve(&yarn(version), args),
                        &[&vt_str::format!("Yarn Classic does not support --location {location}.")],
                    );
                }
            }
        }
    }

    #[test]
    fn test_yarn1_location_user_matches_default_without_warning() {
        for command in [
            vec!["list"],
            vec!["get", "registry"],
            vec!["set", "registry", "https://registry.npmjs.org"],
            vec!["delete", "registry"],
        ] {
            let default_args = parse_subcommand::<ConfigCommand>(command.clone()).unwrap();
            let user_args = parse_subcommand::<ConfigCommand>(
                command.into_iter().chain(["--location", "user"]),
            )
            .unwrap();
            let resolution = resolve(&yarn("1.22.22"), user_args);
            assert_eq!(resolution.outcome, resolve(&yarn("1.22.22"), default_args).outcome);
            assert!(resolution.diagnostics.is_empty());
        }
    }

    #[test]
    fn test_yarn1_global_keeps_precedence_over_location() {
        let args = parse_subcommand::<ConfigCommand>([
            "set",
            "registry",
            "https://registry.npmjs.org",
            "--global",
            "--location",
            "project",
        ])
        .unwrap();
        let resolution = resolve(&yarn("1.22.22"), args);
        assert_eq!(
            resolution.outcome,
            resolve(&yarn("1.22.22"), set_config(Some("global"))).outcome
        );
        assert!(resolution.diagnostics.is_empty());
    }

    #[test]
    fn test_yarn2_rejects_unsupported_locations() {
        for version in ["2.4.2", "3.6.0", "4.18.0"] {
            for location in ["unknown", ""] {
                for command in [
                    vec!["list"],
                    vec!["get", "npmRegistryServer"],
                    vec!["set", "npmRegistryServer", "https://registry.example.com"],
                    vec!["delete", "npmRegistryServer"],
                ] {
                    let args = parse_subcommand::<ConfigCommand>(
                        command.into_iter().chain(["--location", location]),
                    )
                    .unwrap();
                    expect_unsupported(
                        resolve(&yarn(version), args),
                        &[&vt_str::format!("yarn >= 2 does not support --location {location}.")],
                    );
                }
            }
        }
    }

    #[test]
    fn test_berry_user_location_writes_home_and_reads_effective_config() {
        for version in ["2.2.0", "2.4.2", "3.0.0", "3.6.0", "4.18.0"] {
            for command in [
                vec!["list"],
                vec!["get", "npmRegistryServer"],
                vec!["set", "npmRegistryServer", "https://registry.example.com"],
            ] {
                let mut expected = expect_run(
                    resolve(
                        &yarn(version),
                        parse_subcommand::<ConfigCommand>(command.clone()).unwrap(),
                    )
                    .outcome,
                );
                if command[0] == "set" {
                    expected.args.push("--home".to_string());
                }
                let args = parse_subcommand::<ConfigCommand>(
                    command.into_iter().chain(["--location", "user"]),
                )
                .unwrap();
                let resolution = resolve(&yarn(version), args);
                assert_eq!(expect_run(resolution.outcome), expected);
                assert!(resolution.diagnostics.is_empty());
            }
        }
        for version in ["3.0.0", "3.6.0", "4.18.0"] {
            let args = parse_subcommand::<ConfigCommand>([
                "delete",
                "npmRegistryServer",
                "--location",
                "user",
            ])
            .unwrap();
            let resolution = resolve(&yarn(version), args);
            assert_eq!(
                expect_run(resolution.outcome).args,
                vec!["config", "unset", "npmRegistryServer", "--home"]
            );
            assert!(resolution.diagnostics.is_empty());
        }
    }

    #[test]
    fn test_old_berry_rejects_user_writes_without_home_support() {
        for version in ["2.0.0", "2.1.1", "2.2.0-rc.0"] {
            expect_unsupported(
                resolve(&yarn(version), set_config(Some("user"))),
                &["yarn < 2.2 does not support --location user for config set."],
            );
            for location in [None, Some("project"), Some("global")] {
                let resolution = resolve(&yarn(version), set_config(location));
                assert!(resolution.diagnostics.is_empty());
                expect_run(resolution.outcome);
            }
            for command in [vec!["list"], vec!["get", "npmRegistryServer"]] {
                let args = parse_subcommand::<ConfigCommand>(
                    command.into_iter().chain(["--location", "user"]),
                )
                .unwrap();
                let resolution = resolve(&yarn(version), args);
                assert!(resolution.diagnostics.is_empty());
                expect_run(resolution.outcome);
            }
        }
    }

    #[test]
    fn test_yarn2_rejects_user_deletion_without_native_unset() {
        let args = parse_subcommand::<ConfigCommand>([
            "delete",
            "npmRegistryServer",
            "--location",
            "user",
        ])
        .unwrap();
        expect_unsupported(
            resolve(&yarn("2.4.2"), args),
            &["yarn < 3 does not support --location user for config delete."],
        );
    }

    #[test]
    fn test_yarn2_location_project_matches_default_without_warning() {
        for command in [
            vec!["list"],
            vec!["get", "npmRegistryServer"],
            vec!["set", "npmRegistryServer", "https://registry.example.com"],
            vec!["delete", "npmRegistryServer"],
        ] {
            let default_args = parse_subcommand::<ConfigCommand>(command.clone()).unwrap();
            let project_args = parse_subcommand::<ConfigCommand>(
                command.into_iter().chain(["--location", "project"]),
            )
            .unwrap();
            let resolution = resolve(&yarn("4.18.0"), project_args);
            assert_eq!(resolution.outcome, resolve(&yarn("4.18.0"), default_args).outcome);
            assert!(resolution.diagnostics.is_empty());
        }
    }

    #[test]
    fn test_yarn2_global_keeps_precedence_over_location() {
        for command in [
            vec!["list", "--global"],
            vec!["get", "npmRegistryServer", "--global"],
            vec!["set", "npmRegistryServer", "https://registry.example.com", "--global"],
            vec!["delete", "npmRegistryServer", "--global"],
        ] {
            let global_args = parse_subcommand::<ConfigCommand>(command.clone()).unwrap();
            let user_args = parse_subcommand::<ConfigCommand>(
                command.into_iter().chain(["--location", "user"]),
            )
            .unwrap();
            let resolution = resolve(&yarn("4.18.0"), user_args);
            assert_eq!(resolution.outcome, resolve(&yarn("4.18.0"), global_args).outcome);
            assert!(resolution.diagnostics.is_empty());
        }
    }

    #[test]
    fn test_bun_config_fallback_keeps_bun_program() {
        let Resolution { outcome, diagnostics } = resolve(&bun("1.3.11"), set_config(None));
        let command = expect_run(outcome);

        assert_eq!(command.program, "bun");
        assert_eq!(command.args, vec!["config", "set", "registry", "https://registry.npmjs.org"]);
        assert_eq!(
            diagnostics[0].message,
            "bun uses bunfig.toml for configuration, not a config command. Falling back to npm config."
        );
    }
}
