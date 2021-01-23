/*
    Complete save() function
    next add render(),
    next add lighting,
    next add triangulation and mesh objects,
*/

mod vector;
mod sphere;
mod mesh;
mod renderer;


use renderer::{render, save};





fn main() -> Result<(),std::io::Error> {
    println!("Rendering scene...");
    let output = render();
    println!("Saving...");
    save(output)?;
    Ok(())
}
