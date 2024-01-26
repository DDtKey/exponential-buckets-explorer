use strum::{EnumIter, EnumString};

#[derive(strum::Display, EnumString, EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Unit {
    Number,
    Bytes,
    Seconds,
}
