use satisfactory_overlay_lib::game_data::{GameDocsSource, RecipeSource};
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let path = args.next().ok_or("usage: show_recipe <path to en-US.json> [recipe name]")?;
    let wanted = args.next().unwrap_or_else(|| "Screw".into());
    let data = GameDocsSource { path: PathBuf::from(path) }.load()?;
    println!("{} items, {} recipes, {} warnings", data.items.len(), data.recipes.len(), data.warnings.len());
    for recipe in data.recipes.iter().filter(|r| r.name.contains(&wanted)) {
        println!("\n{} ({}) in {}, {}s", recipe.name, recipe.id, data.building_name(&recipe.building), recipe.duration_secs);
        for input in &recipe.ingredients {
            println!("  in : {} × {}", input.amount, data.item_name(&input.item));
        }
        for output in &recipe.products {
            println!("  out: {} × {}", output.amount, data.item_name(&output.item));
        }
    }
    Ok(())
}
