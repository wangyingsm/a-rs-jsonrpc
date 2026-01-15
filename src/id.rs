use serde::{Deserialize, Serialize};

static ATOMIC_U64_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Number(u64),
    String(String),
}

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        Id::Number(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Id::String(value)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Id::String(value.to_string())
    }
}

impl Id {
    pub fn next_number() -> Self {
        let id = ATOMIC_U64_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Id::Number(id)
    }

    pub fn next_string() -> Self {
        let id = ATOMIC_U64_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Id::String(format!("id-{}", id))
    }
}
