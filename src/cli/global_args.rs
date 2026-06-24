use arbitrary::Arbitrary;
use facet::Facet;
use figue as args;

use crate::cli::json_log_behaviour::JsonLogBehaviour;

#[derive(Facet, Default, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct GlobalArgs {
    /// Enable debug logging
    #[facet(args::named, default)]
    pub debug: bool,

    /// Emit structured JSON logs alongside stderr output.
    /// Optionally specify a filename; if not provided, a timestamped filename will be generated.
    #[facet(args::named, args::label = "FILE")]
    json: Option<String>,
}

impl GlobalArgs {
    #[must_use]
    pub fn log_level(&self) -> tracing::Level {
        if self.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        }
    }

    /// Get the JSON log behaviour based on the --json argument.
    #[must_use]
    pub fn json_log_behaviour(&self) -> JsonLogBehaviour {
        match &self.json {
            None => JsonLogBehaviour::None,
            Some(s) if s.is_empty() => JsonLogBehaviour::SomeAutomaticPath,
            Some(s) => JsonLogBehaviour::Some(s.into()),
        }
    }
}
