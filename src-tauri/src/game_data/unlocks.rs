use super::unreal::parse_class_list;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UnlockKind {
    Milestone,
    Mam,
    Alternate,
}

/// One thing the player ticks: a milestone, a MAM research node, or a hard drive alternate.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unlock {
    pub id: String,
    pub name: String,
    pub kind: UnlockKind,
    /// milestone tier, or MAM tree position
    pub tier: u32,
    /// MAM tree, e.g. "Sulfur"; empty for the others
    pub tree: String,
    /// every factory recipe it gives, including through hidden unlocks it hands out
    pub recipes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UnlockBook {
    pub unlocks: Vec<Unlock>,
    /// recipes the player has from the start: HUB upgrades and starting blueprints
    pub always: HashSet<String>,
}

/// Reads schematics from the docs groups; hidden ("Custom") ones count for whoever hands them out.
pub fn read_unlocks(schematic_classes: &[&Value], factory_recipes: &HashSet<String>) -> UnlockBook {
    let raw: HashMap<String, RawSchematic> =
        schematic_classes.iter().filter_map(|c| read_schematic(c, factory_recipes)).map(|s| (s.id.clone(), s)).collect();
    let handed_out: HashSet<&str> = raw.values().flat_map(|s| s.children.iter().map(String::as_str)).collect();
    let mut book = UnlockBook::default();
    for schematic in raw.values() {
        let recipes = all_recipes(&schematic.id, &raw);
        match kind_of(schematic) {
            Some(kind) if !recipes.is_empty() => book.unlocks.push(unlock(schematic, kind, recipes)),
            None if starts_owned(schematic, &handed_out) => book.always.extend(recipes),
            _ => {}
        }
    }
    book.unlocks.sort_by(|a, b| (a.kind as u8, &a.tree, a.tier, &a.name).cmp(&(b.kind as u8, &b.tree, b.tier, &b.name)));
    book
}

struct RawSchematic {
    id: String,
    name: String,
    kind: String,
    tier: u32,
    recipes: Vec<String>,
    children: Vec<String>,
}

fn read_schematic(class: &Value, factory_recipes: &HashSet<String>) -> Option<RawSchematic> {
    let text = |key: &str| class.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let unlocks = class.get("mUnlocks").and_then(Value::as_array).cloned().unwrap_or_default();
    let listed = |key: &str| unlocks.iter().filter_map(|u| u.get(key).and_then(Value::as_str)).flat_map(parse_class_list).collect::<Vec<_>>();
    Some(RawSchematic {
        id: class.get("ClassName")?.as_str()?.to_string(),
        name: text("mDisplayName"),
        kind: text("mType"),
        tier: text("mTechTier").parse().unwrap_or(0),
        recipes: listed("mRecipes").into_iter().filter(|r| factory_recipes.contains(r)).collect(),
        children: listed("mSchematics"),
    })
}

/// Its own recipes plus those of every hidden unlock it hands out, however deep.
fn all_recipes(id: &str, raw: &HashMap<String, RawSchematic>) -> Vec<String> {
    let (mut seen, mut todo, mut recipes) = (HashSet::new(), vec![id], BTreeSet::new());
    while let Some(next) = todo.pop() {
        let Some(schematic) = raw.get(next).filter(|_| seen.insert(next)) else { continue };
        recipes.extend(schematic.recipes.iter().cloned());
        todo.extend(schematic.children.iter().map(String::as_str));
    }
    recipes.into_iter().collect()
}

/// The MAM tree from an id like Research_Sulfur_TurboFuel_C; FICSMAS research is left out.
fn mam_tree(id: &str) -> Option<String> {
    match id.split('_').nth(1)? {
        "XMas" => None,
        "PowerSlugs" => Some("Power Slugs".into()),
        "Alien" | "ACarapace" | "AOrgans" | "AO" | "AOrganisms" => Some("Alien Organisms".into()),
        tree => Some(tree.to_string()),
    }
}

fn kind_of(schematic: &RawSchematic) -> Option<UnlockKind> {
    match schematic.kind.as_str() {
        "EST_Milestone" => Some(UnlockKind::Milestone),
        "EST_MAM" if mam_tree(&schematic.id).is_some() => Some(UnlockKind::Mam),
        "EST_Alternate" => Some(UnlockKind::Alternate),
        _ => None,
    }
}

/// HUB upgrades, and hidden unlocks nothing hands out (the starting blueprints).
fn starts_owned(schematic: &RawSchematic, handed_out: &HashSet<&str>) -> bool {
    schematic.kind == "EST_Tutorial" || (schematic.kind == "EST_Custom" && !handed_out.contains(schematic.id.as_str()))
}

fn unlock(schematic: &RawSchematic, kind: UnlockKind, recipes: Vec<String>) -> Unlock {
    let tree = if kind == UnlockKind::Mam { mam_tree(&schematic.id).unwrap_or_default() } else { String::new() };
    Unlock { id: schematic.id.clone(), name: schematic.name.clone(), kind, tier: schematic.tier, tree, recipes }
}

impl UnlockBook {
    /// No schematics in the data (old or test docs): every standard recipe counts as owned.
    pub fn is_empty(&self) -> bool {
        self.unlocks.is_empty()
    }

    /// Recipes the player can use with these ticks.
    pub fn owned_recipes(&self, ticked: &BTreeSet<String>) -> HashSet<String> {
        let ticked_recipes = self.unlocks.iter().filter(|u| ticked.contains(&u.id)).flat_map(|u| u.recipes.iter().cloned());
        self.always.iter().cloned().chain(ticked_recipes).collect()
    }

    /// The unticked unlocks that would give one of `recipes`, milestones first.
    pub fn givers_of<'a>(&'a self, recipes: &[&str], ticked: &BTreeSet<String>) -> Vec<&'a Unlock> {
        self.unlocks
            .iter()
            .filter(|u| !ticked.contains(&u.id) && u.recipes.iter().any(|r| recipes.contains(&r.as_str())))
            .collect()
    }
}
