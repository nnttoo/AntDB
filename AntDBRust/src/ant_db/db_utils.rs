use std::sync::Arc;

use crate::{BoxError, ant_db::db::AntDB};

pub struct AntDButils {
    db: Arc<AntDB>,
}

impl AntDButils {
    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }

    fn is_pattern_match(pattern: &str, text: &str) -> bool {
        let p_bytes = pattern.as_bytes();
        let t_bytes = text.as_bytes();
        let (mut p, mut t) = (0, 0);
        let (mut star_idx, mut match_idx) = (None, 0);

        while t < t_bytes.len() {
            if p < p_bytes.len() && (p_bytes[p] == b'?' || p_bytes[p] == t_bytes[t]) {
                p += 1;
                t += 1;
            } else if p < p_bytes.len() && p_bytes[p] == b'*' {
                star_idx = Some(p);
                match_idx = t;
                p += 1;
            } else if let Some(s_idx) = star_idx {
                p = s_idx + 1;
                match_idx += 1;
                t = match_idx;
            } else {
                return false;
            }
        }

        while p < p_bytes.len() && p_bytes[p] == b'*' {
            p += 1;
        }

        p == p_bytes.len()
    }

    pub fn keys(&self, pattern: &str) -> Result<Vec<String>, BoxError> {
        let Ok(hmap_lock) = self.db.hash_map.read() else {
            return Err(Box::from("error lock"));
        };

        let mut result: Vec<String> = Vec::new();

        for (key, item) in hmap_lock.iter() {
            // Skip expired items
            if self.db.expire_delete(key, item) {
                continue;
            }

            // Filter key by pattern matching
            if Self::is_pattern_match(pattern, key) {
                result.push(key.clone());
            }
        }

        Ok(result)
    }
}
