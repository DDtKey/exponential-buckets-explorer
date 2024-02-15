use strum::{EnumIter, EnumString};

#[derive(strum::Display, EnumString, EnumIter, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Unit {
    #[default]
    Number,
    Bytes,
    Seconds,
}
