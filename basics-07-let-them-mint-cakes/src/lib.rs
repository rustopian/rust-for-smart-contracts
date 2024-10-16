// Define the BakedGoodType enum here
// TODO: Add variants for Cake, Bread, Cookie

// Define the BakedGood struct here
// TODO: Include fields: id, name, baked_good_type

// Implement the minting function
pub fn mint_baked_good(
    id: u32,
    name: String,
    baked_good_type: BakedGoodType,
    secret_recipe: Option<&str>,
) -> Result<BakedGood, String> {
    // TODO: Implement the logic to mint a baked good NFT
    // - If baked_good_type is Cake, check that secret_recipe is Some("TheCakeIsALie")
    //   - If the recipe is correct, mint the Cake baked good
    //   - Otherwise, return an Err with a message explaining the failure
    // - If baked_good_type is not Cake, mint the baked good without checking secret_recipe
    // - Return Ok(BakedGood) if successful
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_cake_with_correct_recipe() {
        let baked_good = mint_baked_good(
            1,
            "Chocolate Cake".to_string(),
            BakedGoodType::Cake,
            Some("TheCakeIsALie"),
        )
        .unwrap();
        assert_eq!(baked_good.id, 1);
        assert_eq!(baked_good.name, "Chocolate Cake");
        assert_eq!(baked_good.baked_good_type, BakedGoodType::Cake);
    }

    #[test]
    fn test_mint_cake_with_incorrect_recipe() {
        let result = mint_baked_good(
            2,
            "Vanilla Cake".to_string(),
            BakedGoodType::Cake,
            Some("WrongRecipe"),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid secret recipe. Cannot mint Cake."
        );
    }

    #[test]
    fn test_mint_cake_without_recipe() {
        let result = mint_baked_good(
            3,
            "Strawberry Cake".to_string(),
            BakedGoodType::Cake,
            None,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Secret recipe required to mint Cake."
        );
    }

    #[test]
    fn test_mint_bread() {
        let baked_good = mint_baked_good(
            4,
            "Sourdough Bread".to_string(),
            BakedGoodType::Bread,
            None,
        )
        .unwrap();
        assert_eq!(baked_good.id, 4);
        assert_eq!(baked_good.name, "Sourdough Bread");
        assert_eq!(baked_good.baked_good_type, BakedGoodType::Bread);
    }

    #[test]
    fn test_mint_cookie_with_recipe() {
        let baked_good = mint_baked_good(
            5,
            "Chocolate Chip Cookie".to_string(),
            BakedGoodType::Cookie,
            Some("AnyRecipe"),
        )
        .unwrap();
        assert_eq!(baked_good.id, 5);
        assert_eq!(baked_good.name, "Chocolate Chip Cookie");
        assert_eq!(baked_good.baked_good_type, BakedGoodType::Cookie);
    }
}