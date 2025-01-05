fn main() {
    // Dataset 1: Names of commissioners
    let commissioners = vec![
        "Aigboogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical zones
    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merge datasets into a single output
    println!("S/N\tName of Commissioner\t\tMinistry\t\tGeopolitical Zone");
    for i in 0..commissioners.len() {
        println!(
            "{}\t{}\t\t{}\t\t{}",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }
}
