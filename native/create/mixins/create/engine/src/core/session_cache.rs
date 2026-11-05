// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Extracted unchanged from upstream transport.rs; no belt implementation.
/// Runtime ownership by world session and dimension. A host may swap several
/// dimensions through the same world object; visiting one must retain the others.
#[derive(Debug)]
pub struct SessionCache<T> {
    session: Option<usize>,
    dimensions: std::collections::BTreeMap<String, T>,
}
impl<T> Default for SessionCache<T> {
    fn default() -> Self {
        Self {
            session: None,
            dimensions: Default::default(),
        }
    }
}
impl<T: Default> SessionCache<T> {
    pub fn dimension(&mut self, session: usize, dimension: &str) -> &mut T {
        if self.session != Some(session) {
            self.session = Some(session);
            self.dimensions.clear();
        }
        self.dimensions.entry(dimension.to_owned()).or_default()
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
