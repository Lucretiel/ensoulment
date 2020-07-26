use chrono;
use std::{
    env,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Clone)]
enum EnsoulmentBuildError {
    EnvParseError(chrono::ParseError),
}

impl Display for EnsoulmentBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EnsoulmentBuildError::EnvParseError(err) => write!(
                f,
                "An $ENSOULMENT_TIMESTAMP variable was given, but it was not \
                a valid rfc3339 timestamp. Error: {}",
                err
            ),
        }
    }
}

impl Debug for EnsoulmentBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for EnsoulmentBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EnsoulmentBuildError::EnvParseError(err) => Some(err),
        }
    }
}

impl From<chrono::ParseError> for EnsoulmentBuildError {
    fn from(err: chrono::ParseError) -> Self {
        EnsoulmentBuildError::EnvParseError(err)
    }
}

fn main() -> Result<(), EnsoulmentBuildError> {
    // At the end of this build script, $OUT_DIR/moment.rs will have a valid
    // rust *expression* that resolves to a `chrono::DateTime<FixedOffset>`.
    // The lib.rs file takes care of the rest.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("moment.rs");
    let dest_file = fs::File::create(&dest_path).unwrap();
    let mut dest_file = io::BufWriter::new(dest_file);

    let date = match env::var("ENSOULMENT_TIMESTAMP").ok() {
        Some(timestamp) => chrono::DateTime::parse_from_rfc3339(&timestamp)?,
        None => {
            let now = chrono::Local::now();
            now.with_timezone(now.offset())
        }
    };

    let formatted = date.to_rfc3339();

    write!(
        dest_file,
        "::chrono::DateTime::parse_from_rfc3339(\"{}\").unwrap()",
        formatted
    )
    .unwrap();

    Ok(())
}
