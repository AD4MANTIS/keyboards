pub(crate) mod advanced_setup;
mod draw;
pub(crate) mod logging;
pub(crate) mod models;
mod objective;
pub mod prelude;

use advanced_setup::SEED;
use draw::draw_keyboard;
use logging::{append_to_file, append_updates};
use models::genome::{create_genome, shuffle_genome};
use models::layout::ILayout;
use objective::determine_keypress;
use rand::prelude::*;

pub struct SaSetup {
    pub temperature: f64,
    pub epoch: f64,
    pub cooling_rate: f64,
    pub num_iterations: usize,
}

// simulated annealing
pub fn run_sa<L: ILayout<N>, const N: usize>(
    text: &str,
    setup: SaSetup,
    logging: logging::LoggingOptions,
) -> [char; N] {
    let SaSetup {
        mut temperature,
        epoch,
        cooling_rate,
        num_iterations,
    } = setup;

    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

    let genome = L::get_genome();
    let layout_map = L::get_layout_map();

    let mut letter_list = *L::get_letter_list();

    if logging.verbosity.is_normal() {
        println!("Calculating raw baseline: ");
    }

    let key_presses: Vec<_> = text
        .chars()
        .filter_map(determine_keypress::<L, N>)
        .collect();

    let layout_score = objective::baseline_objective_function::<L, N>(
        &key_presses,
        genome,
        &layout_map,
        &letter_list,
    );

    if logging.verbosity.is_normal() {
        println!("{}", layout_score);

        println!(
            "From here everything is relative with + % worse and - % better than this baseline \n
            Note that best layout is being saved as a png at each step. Kill program when satisfied."
        );

        println!("Temperature \t Iteration \t Best Score \t New Score");
    }

    let mut current_genome = create_genome(&mut letter_list, &mut rng);
    let mut current_objective = objective::objective_function::<L, N>(
        &key_presses,
        &current_genome,
        &layout_map,
        Some(layout_score),
        &letter_list,
    );

    let mut best_genome = current_genome;
    let mut best_objective = current_objective;

    match logging.image {
        logging::SaveImageOption::FirstAndLast | logging::SaveImageOption::AllBest => {
            draw_keyboard(&best_genome, "0", &layout_map)
        }
        _ => {}
    };

    if logging.text {
        append_updates("\nStarting new Run");
        append_updates("temperature | iteration | bestObjective | newObjective");
    }

    // run SA
    let mut static_count = 0.0;

    for iteration in 1..=num_iterations {
        if temperature <= 1.0 {
            break;
        }

        // ~ create new genome ~
        let new_genome = shuffle_genome(&current_genome, 2., &mut rng);

        // ~ asses ~
        let new_objective = objective::objective_function::<L, N>(
            &key_presses,
            &new_genome,
            &layout_map,
            Some(layout_score),
            &letter_list,
        );
        let delta = new_objective - current_objective;

        if logging.verbosity.is_normal() {
            println!("{temperature:.2}\t{iteration}\t{best_objective:.2}\t{new_objective:.2}");
        }

        if delta < 0.0 {
            current_genome = new_genome;
            current_objective = new_objective;

            if logging.text {
                let update_line = format!(
                    "{temperature:.2}, {iteration}, {best_objective:.5}, {new_objective:.5}"
                );
                append_updates(&update_line);
            }

            if new_objective < best_objective {
                best_genome = new_genome;
                best_objective = new_objective;

                if logging::SaveImageOption::AllBest == logging.image {
                    println!("(new best, png being saved)");
                    draw_keyboard(&best_genome, iteration.to_string().as_str(), &layout_map);
                }

                if logging.text {
                    append_to_file(
                        "results/bestGenomes.txt",
                        &format!("{iteration}: {best_genome:#?}\n"),
                    );
                }

                if logging.image == logging::SaveImageOption::None && logging.verbosity.is_normal()
                {
                    println!("(new best)");
                }
            }
        } else if (-delta / temperature).exp() > rand::random() {
            current_genome = new_genome;
            current_objective = new_objective;
        }

        static_count += 1.0;

        if static_count > epoch {
            static_count = 0.0;
            temperature *= cooling_rate;

            if rand::random::<f64>() < 0.5 {
                current_genome = best_genome;
                current_objective = best_objective;
            }
        }
    }

    match logging.image {
        logging::SaveImageOption::AllBest
        | logging::SaveImageOption::Last
        | logging::SaveImageOption::FirstAndLast => {
            draw_keyboard(&best_genome, "final", &layout_map);
        }
        _ => {}
    }

    best_genome
}

#[cfg(test)]
mod tests {
    use super::*;

    use prelude::*;

    fn get_setup() -> SaSetup {
        SaSetup {
            temperature: 100.,
            epoch: 20.,
            cooling_rate: 0.9,
            num_iterations: 10,
        }
    }

    #[test]
    fn test_run_sa_qwerty_does_not_panic() {
        run_sa::<QwertyEnUs, 46>(
            &std::fs::read_to_string("resources/myBook.txt").expect("should read file"),
            get_setup(),
            LoggingOptions::silent(),
        );
    }

    #[test]
    fn test_run_sa_qwerty_with_german_umlaute_ignored() {
        run_sa::<QwertyEnUs, 46>(
            &std::fs::read_to_string("resources/meinBuch.txt").expect("should read file"),
            get_setup(),
            LoggingOptions::silent(),
        );
    }

    #[test]
    fn test_run_sa_qwertz_does_not_panic() {
        run_sa::<QwertzDeDe, 48>(
            &std::fs::read_to_string("resources/meinBuch.txt").expect("should read file"),
            get_setup(),
            LoggingOptions::silent(),
        );
    }
}
