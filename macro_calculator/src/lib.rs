use json::*;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn round_to_precision(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let cals_per_portion: f64 = food.calories[1]
            .replace("kcal", "")
            .parse()
            .expect("Invalid calorie value");
        total_cals += cals_per_portion * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    JsonValue::from(
        object! {
            "cals" => round_to_precision(total_cals),
            "carbs" => round_to_precision(total_carbs),
            "proteins" => round_to_precision(total_proteins),
            "fats" => round_to_precision(total_fats),
        }
    )
}
