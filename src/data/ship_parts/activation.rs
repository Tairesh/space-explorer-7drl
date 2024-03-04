use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub enum Activation {
    Open,
    Close,
    Lock,
    Unlock,
    UseTerminal,
}
