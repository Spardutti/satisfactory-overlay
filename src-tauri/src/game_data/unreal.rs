use super::model::ItemAmount;

const ITEM_KEY: &str = "ItemClass=";
const AMOUNT_KEY: &str = ",Amount=";

/// Parses an Unreal struct list like `((ItemClass="...Desc_X.Desc_X_C'",Amount=1),(...))`.
pub fn parse_item_amounts(text: &str) -> Result<Vec<ItemAmount>, String> {
    text.split(ITEM_KEY).skip(1).map(parse_entry).collect()
}

/// Parses a class path list like `("/Game/.../Build_X.Build_X_C","/Script/FactoryGame.FGBuildGun")`.
pub fn parse_class_list(text: &str) -> Vec<String> {
    text.split(',')
        .map(last_class_segment)
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_entry(entry: &str) -> Result<ItemAmount, String> {
    let split_at = entry
        .find(AMOUNT_KEY)
        .ok_or_else(|| format!("Missing Amount in `{entry}`"))?;
    let item = last_class_segment(&entry[..split_at]);
    let amount_text: String = entry[split_at + AMOUNT_KEY.len()..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let amount = amount_text
        .parse()
        .map_err(|_| format!("Bad Amount in `{entry}`"))?;
    if item.is_empty() {
        return Err(format!("Missing item class in `{entry}`"));
    }
    Ok(ItemAmount { item, amount })
}

fn last_class_segment(path: &str) -> String {
    let trimmed = path.trim_matches(|c: char| "\"'\\() ".contains(c));
    let tail = trimmed.rsplit('.').next().unwrap_or(trimmed);
    tail.rsplit('/').next().unwrap_or(tail).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn amount(item: &str, amount: f64) -> ItemAmount {
        ItemAmount { item: item.into(), amount }
    }

    #[test]
    fn parse_item_amounts_single_entry_returns_item_and_amount() {
        let text = r#"((ItemClass="/Script/Engine.BlueprintGeneratedClass'/Game/FactoryGame/Resource/Parts/IronRod/Desc_IronRod.Desc_IronRod_C'",Amount=1))"#;
        assert_eq!(parse_item_amounts(text).unwrap(), vec![amount("Desc_IronRod_C", 1.0)]);
    }

    #[test]
    fn parse_item_amounts_several_entries_keeps_order() {
        let text = r#"((ItemClass="/Game/A/Desc_IronPlate.Desc_IronPlate_C'",Amount=6),(ItemClass="/Game/B/Desc_IronScrew.Desc_IronScrew_C'",Amount=12))"#;
        assert_eq!(
            parse_item_amounts(text).unwrap(),
            vec![amount("Desc_IronPlate_C", 6.0), amount("Desc_IronScrew_C", 12.0)]
        );
    }

    #[test]
    fn parse_item_amounts_escaped_quotes_returns_item() {
        let text = r#"((ItemClass=BlueprintGeneratedClass'\"/Game/W/Desc_Water.Desc_Water_C\"',Amount=1000))"#;
        assert_eq!(parse_item_amounts(text).unwrap(), vec![amount("Desc_Water_C", 1000.0)]);
    }

    #[test]
    fn parse_item_amounts_empty_text_returns_nothing() {
        assert_eq!(parse_item_amounts("").unwrap(), vec![]);
    }

    #[test]
    fn parse_item_amounts_missing_amount_is_an_error() {
        let err = parse_item_amounts(r#"((ItemClass="/Game/X/Desc_X.Desc_X_C'"))"#).unwrap_err();
        assert!(err.starts_with("Missing Amount in"), "{err}");
    }

    #[test]
    fn parse_class_list_returns_every_class_name() {
        let text = r#"("/Game/FactoryGame/Buildable/Factory/SmelterMk1/Build_SmelterMk1.Build_SmelterMk1_C","/Script/FactoryGame.FGBuildGun")"#;
        assert_eq!(parse_class_list(text), vec!["Build_SmelterMk1_C", "FGBuildGun"]);
    }

    #[test]
    fn parse_class_list_empty_text_returns_nothing() {
        assert_eq!(parse_class_list(""), Vec::<String>::new());
    }
}
