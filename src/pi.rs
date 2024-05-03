use rand::Rng;

/// Estime la valeur de pi en utilisant la méthode de Monte-Carlo.
///
/// Arguments:
///     num_points: Le nombre de points aléatoires à générer.
///
/// Returns:
///     Une estimation de la valeur de pi.
fn estimate_pi(num_points: usize) -> f64 {
    let mut circle_points = 0;
    let mut total_points = 0;

    let mut rng = rand::thread_rng();

    for _ in 0..num_points {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        // Vérifier si le point est à l'intérieur du cercle
        if x.powi(2) + y.powi(2) <= 1.0 {
            circle_points += 1;
        }
        total_points += 1;
    }

    // Estimer pi
    let pi_estimation = (circle_points as f64 / total_points as f64) * 4.0;

    pi_estimation
}

fn main() {
    // Exemple d'utilisation
    let num_points: usize = {
        use std::io::{self, Write};
        print!("Nombre de points: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let pi_estimate = estimate_pi(num_points);

    println!(
        "Pi Estimation with {} points: {}",
        num_points, pi_estimate
    );
}
