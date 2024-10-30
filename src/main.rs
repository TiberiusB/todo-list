mod cli; // "mod" veut dire module, est un keyword qui permet d'importer/exporter des modules, d'autres fishiers ou ensemble de fichiers (modules ou sous modules) qu'on cre dans le src, qui est le dossier source.

use cli::*; // "*" means export all. If not, we can write "use cli::Person" or "use cli::{PErson,Age}"

fn main() {
    let mut personne = Personne::new("Tibi", 20); // "mute" a cause que personne.birthday va changer la valeur quand elle s'execute
    println!("{} is {}", personne.name, personne.age);
    personne.birthday();
    println!("new age is {}", personne.age);
    println!("{:?}", personne);
}
