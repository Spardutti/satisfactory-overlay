use super::machine_power::machine_power;
use super::model::{GameData, Item, ItemAmount, Recipe};
use super::power::{energy_values, power_recipes, POWER};
use super::unlocks::read_unlocks;
use super::unreal::{parse_class_list, parse_item_amounts};
use serde_json::Value;

/// Liquids and gases are stored in litres; the UI works in m³.
const FLUID_UNITS_PER_M3: f64 = 1000.0;

struct Group<'a> {
    native_class: &'a str,
    classes: &'a [Value],
}

/// Builds items, factory buildings and factory recipes from the docs JSON text.
pub fn parse_docs(text: &str) -> Result<GameData, String> {
    let root: Vec<Value> =
        serde_json::from_str(text).map_err(|e| format!("Docs file is not a JSON array: {e}"))?;
    let groups: Vec<Group> = root.iter().filter_map(as_group).collect();
    let mut data = GameData::default();
    for group in groups.iter().filter(|g| is_item_group(g.native_class)) {
        let resource = group.native_class.contains("FGResourceDescriptor");
        group.classes.iter().for_each(|c| read_item(c, resource, &mut data));
    }
    for group in groups.iter().filter(|g| g.native_class.contains("FGBuildableManufacturer")) {
        group.classes.iter().for_each(|c| read_building(c, &mut data));
    }
    for group in groups.iter().filter(|g| g.native_class.contains("FGRecipe")) {
        group.classes.iter().for_each(|c| read_recipe(c, &mut data));
    }
    let factory_recipes = data.recipes.iter().map(|r| r.id.clone()).collect();
    let schematics: Vec<&Value> =
        groups.iter().filter(|g| g.native_class.contains("FGSchematic")).flat_map(|g| g.classes.iter()).collect();
    data.unlocks = read_unlocks(&schematics, &factory_recipes);
    add_power(&groups, &mut data);
    data.machine_power = machine_power(&power_users(&groups));
    Ok(data)
}

/// Generator + fuel recipes that make power; generators need no unlock, their fuels do.
fn add_power(groups: &[Group], data: &mut GameData) {
    let classes = |pick: fn(&str) -> bool| groups.iter().filter(|g| pick(g.native_class)).flat_map(|g| g.classes.iter()).collect::<Vec<_>>();
    let generators = classes(|n| n.contains("FGBuildableGeneratorFuel"));
    let energy = energy_values(&classes(is_item_group));
    generators.iter().for_each(|g| read_building(g, data));
    data.items.insert(POWER.into(), Item { id: POWER.into(), name: "Power (MW)".into(), fluid: false, resource: false });
    let recipes = power_recipes(&generators, &energy, data);
    data.unlocks.always.extend(recipes.iter().map(|r| r.id.clone()));
    data.recipes.extend(recipes);
}

/// Factories and extractors; generators make power instead of drawing it.
fn power_users<'a>(groups: &[Group<'a>]) -> Vec<&'a Value> {
    let draws = |n: &str| ["FGBuildableManufacturer", "FGBuildableResourceExtractor", "FGBuildableWaterPump"].iter().any(|k| n.contains(k));
    groups.iter().filter(|g| draws(g.native_class)).flat_map(|g| g.classes.iter()).collect()
}

fn as_group(value: &Value) -> Option<Group<'_>> {
    Some(Group {
        native_class: value.get("NativeClass")?.as_str()?,
        classes: value.get("Classes")?.as_array()?,
    })
}

fn is_item_group(native_class: &str) -> bool {
    native_class.contains("Descriptor") || native_class.contains("FGAmmoType")
}

fn field<'a>(class: &'a Value, key: &str) -> Option<&'a str> {
    class.get(key)?.as_str()
}

fn read_item(class: &Value, resource: bool, data: &mut GameData) {
    let (Some(id), Some(name)) = (field(class, "ClassName"), field(class, "mDisplayName")) else {
        data.warnings.push("Skipped an item without ClassName or mDisplayName".into());
        return;
    };
    let form = field(class, "mForm").unwrap_or("RF_SOLID");
    let fluid = form == "RF_LIQUID" || form == "RF_GAS";
    let item = Item { id: id.into(), name: name.into(), fluid, resource };
    data.items.insert(id.into(), item);
}

fn read_building(class: &Value, data: &mut GameData) {
    match (field(class, "ClassName"), field(class, "mDisplayName")) {
        (Some(id), Some(name)) => {
            data.buildings.insert(id.into(), name.into());
        }
        _ => data.warnings.push("Skipped a building without ClassName or mDisplayName".into()),
    }
}

fn read_recipe(class: &Value, data: &mut GameData) {
    let id = field(class, "ClassName").unwrap_or("<no ClassName>");
    let Some(building) = factory_building(class, data) else {
        return;
    };
    match build_recipe(class, building, data) {
        Ok(recipe) => data.recipes.push(recipe),
        Err(reason) => data.warnings.push(format!("Skipped recipe {id}: {reason}")),
    }
}

fn factory_building(class: &Value, data: &GameData) -> Option<String> {
    parse_class_list(field(class, "mProducedIn")?)
        .into_iter()
        .find(|b| data.buildings.contains_key(b))
}

fn build_recipe(class: &Value, building: String, data: &GameData) -> Result<Recipe, String> {
    let get = |key: &str| field(class, key).ok_or(format!("missing {key}"));
    let id = get("ClassName")?;
    let name = get("mDisplayName")?;
    let duration_secs: f64 = get("mManufactoringDuration")?
        .parse()
        .map_err(|_| "bad mManufactoringDuration".to_string())?;
    if duration_secs <= 0.0 {
        return Err("mManufactoringDuration is not positive".into());
    }
    Ok(Recipe {
        id: id.into(),
        name: name.into(),
        alternate: name.starts_with("Alternate:") || id.starts_with("Recipe_Alternate_"),
        ingredients: to_display_units(parse_item_amounts(get("mIngredients")?)?, data),
        products: to_display_units(parse_item_amounts(get("mProduct")?)?, data),
        duration_secs,
        building,
    })
}

fn to_display_units(amounts: Vec<ItemAmount>, data: &GameData) -> Vec<ItemAmount> {
    amounts
        .into_iter()
        .map(|a| match data.is_fluid(&a.item) {
            true => ItemAmount { amount: a.amount / FLUID_UNITS_PER_M3, ..a },
            false => a,
        })
        .collect()
}

#[cfg(test)]
#[path = "docs_tests.rs"]
mod tests;
