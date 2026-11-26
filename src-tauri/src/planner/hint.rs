use super::Planner;
use crate::game_data::{Unlock, UnlockKind};
use std::collections::BTreeSet;

/// Enough names to act on without a wall of text.
const MAX_NAMED: usize = 3;

impl Planner<'_> {
    /// " Tick one of these in My recipes: Turbofuel (MAM Sulfur)." — empty when nothing locked makes `item`.
    /// A giver of a recipe not owned is never ticked, so no tick list is needed.
    pub(super) fn tick_hint(&self, item: &str) -> String {
        let locked: Vec<&str> = self
            .data
            .recipes
            .iter()
            .filter(|r| r.products.iter().any(|p| p.item == item) && !self.enabled_alternates.contains(&r.id))
            .map(|r| r.id.as_str())
            .collect();
        let givers = self.data.unlocks.givers_of(&locked, &BTreeSet::new());
        if givers.is_empty() {
            return String::new();
        }
        let names: Vec<String> = givers.iter().take(MAX_NAMED).map(|u| label(u)).collect();
        format!(" Tick one of these in My recipes: {}.", names.join(", "))
    }
}

fn label(unlock: &Unlock) -> String {
    match unlock.kind {
        UnlockKind::Milestone => format!("{} (Tier {})", unlock.name, unlock.tier),
        UnlockKind::Mam => format!("{} (MAM {})", unlock.name, unlock.tree),
        UnlockKind::Alternate => unlock.name.clone(),
    }
}
