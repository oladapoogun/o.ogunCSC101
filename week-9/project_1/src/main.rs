use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("nigerian_breweries_drinks.txt")?;

    let lager_list = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];
    let stout_list = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic_list = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    write_category_to_file(&mut file, "Lager", &lager_list)?;
    write_category_to_file(&mut file, "Stout", &stout_list)?;
    write_category_to_file(&mut file, "Non-Alcoholic", &non_alcoholic_list)?;

    Ok(())
}

fn write_category_to_file(
    file: &mut File,
    category_name: &str,
    drink_list: &[&str],
) -> std::io::Result<()> {
    writeln!(file, "{}", category_name)?;
    for drink in drink_list {
        writeln!(file, "  - {}", drink)?;
    }
    Ok(())
}