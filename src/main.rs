#![allow(non_snake_case)]

mod calcul;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let rayon_str = &args[1];
	let rayon = rayon_str.parse::<f64>().unwrap();

	let p = calcul::perimetre_cercle(rayon);
        println!(">>> The perimeter of the circle is : {}", p);

	let s = calcul::surface_cercle(rayon);
	println!(">>> The area of the circle is : {}", s);

	let s_sph = calcul::surface_sphere(rayon);
	println!(">>> The surface area of the sphere is : {}", s_sph);

	let v_sph = calcul::volume_sphere(rayon);
	println!(">>> Le volume of the sphere is : {}", v_sph);
}
