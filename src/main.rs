fn main() {
    println!("=============================================================");
    println!("Run the examples. Don't forget to move assets to target/debug/assets.");
    println!("=> components_schedule - For using components and schedule approach");
    println!(
        "=> components - For using components without schedule. This approach has frame delays (1 frame per level) but is also the most simple one."
    );
    println!("=> resources - For using resources in order to communicate between functions. This approach has no frame delays but is too complex!");
}
