use time::{self, Tm, Duration};
use serde::{self, Deserialize, Deserializer};

// This format string necessitates the use of `time` rather than `chrono`
// as `chrono` does not support Timezone names.
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f %Z";

pub fn deserialize<D: Deserializer>(deserializer: D) -> Result<Tm, D::Error> {
    let s = String::deserialize(deserializer)?;
    // Hack to get around https://github.com/rust-lang-deprecated/time/issues/92
    time::strptime(&s, FORMAT)
        .map(|dt| dt + Duration::seconds(0))
        .map_err(serde::de::Error::custom)
}
