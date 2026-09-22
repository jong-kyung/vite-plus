use semver::Version;

pub(crate) trait PackageManagerDialect {
    const COMMAND_NAME: &'static str;

    fn version(&self) -> Option<&Version>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Npm {
    version: Option<Version>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Pnpm {
    version: Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Yarn {
    version: Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Bun {
    version: Version,
}

macro_rules! impl_dialect {
    ($type:ident, $command_name:expr) => {
        impl $type {
            pub(crate) fn new(version: Version) -> Self {
                Self { version }
            }
        }

        impl PackageManagerDialect for $type {
            const COMMAND_NAME: &'static str = $command_name;

            fn version(&self) -> Option<&Version> {
                Some(&self.version)
            }
        }
    };
}

impl_dialect!(Yarn, "yarn");
impl_dialect!(Pnpm, "pnpm");
impl_dialect!(Bun, "bun");

impl Npm {
    pub(crate) fn new(version: Version) -> Self {
        Self { version: Some(version) }
    }

    pub(crate) fn unknown_version() -> Self {
        Self { version: None }
    }

    /// Script approval commands landed in npm 11.16.0; unknown versions are current.
    pub(crate) fn supports_script_approval(&self) -> bool {
        self.version.as_ref().is_none_or(|version| version >= &Version::new(11, 16, 0))
    }

    /// `patch add`, `patch commit`, and enforced script approvals landed in npm 12.
    /// Unknown versions are treated as current.
    pub(crate) fn supports_v12_commands(&self) -> bool {
        self.version.as_ref().is_none_or(|version| version >= &Version::new(12, 0, 0))
    }
}

impl PackageManagerDialect for Npm {
    const COMMAND_NAME: &'static str = "npm";

    fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }
}

impl Pnpm {
    /// Build approval deny syntax landed in pnpm 11.
    pub(crate) fn supports_build_denial(&self) -> bool {
        self.version >= Version::new(11, 0, 0)
    }
}

impl Yarn {
    pub(crate) fn is_berry(&self) -> bool {
        crate::package_manager::is_yarn_berry(&self.version)
    }

    /// Yarn 2.2 added config set --home.
    /// https://github.com/yarnpkg/berry/blob/01586a88806a2bebd7edb28d1bee3581b1fd3762/CHANGELOG.md#220
    pub(crate) fn supports_config_set_home(&self) -> bool {
        self.version >= Version::new(2, 2, 0)
    }

    /// `config unset` and range-preserving `up --recursive` landed in Yarn 3.
    /// https://github.com/yarnpkg/berry/blob/01586a88806a2bebd7edb28d1bee3581b1fd3762/CHANGELOG.md#300
    pub(crate) fn supports_v3_commands(&self) -> bool {
        self.version >= Version::new(3, 0, 0)
    }

    /// Staged publishing landed in Yarn 4.16.0.
    /// https://github.com/yarnpkg/berry/releases/tag/%40yarnpkg/cli/4.16.0
    pub(crate) fn supports_staged_publishing(&self) -> bool {
        self.version >= Version::new(4, 16, 0)
    }
}

impl Bun {
    /// `bun pm version` landed in Bun 1.2.18.
    pub(crate) fn supports_version_command(&self) -> bool {
        self.version >= Version::new(1, 2, 18)
    }

    /// `dedupe`, `prune`, and `audit fix` landed in bun 1.4.
    pub(crate) fn supports_v1_4_commands(&self) -> bool {
        self.version >= Version::new(1, 4, 0)
    }
}
