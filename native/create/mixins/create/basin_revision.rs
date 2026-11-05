// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// Private generated-build compatibility with upstream's entity identity check.
// This child module is installed in ModRevision's defining crate/file.
impl super::ModRevision {
    pub fn same_entity(self, other: Self) -> bool {
        self.born == other.born
    }
}
