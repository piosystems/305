pub fn run() {
    // Variables can be type annotated.
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation

    println!(
        "Primitive scalar variables illustrated include:\n logical = {}\n float = {}",
        logical, a_float
    );
}