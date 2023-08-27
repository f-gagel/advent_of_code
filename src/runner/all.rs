use rayon::prelude::*;
use std::convert::Infallible;
use std::io::{Write, BufReader};
use std::path::PathBuf;
use std::time::Duration;

pub fn run() -> Result<(), Infallible> {
    let tasks = advent_of_code::get_years()
        .flat_map(|y| {
            y.days().iter().flat_map(|d| {
                let mut path = PathBuf::from_iter(["inputs", y.name(), d.name()]);
                path.set_extension("txt");
                d.tasks().iter().map(move |t| (t, path.clone()))
            })
        })
        .collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let total_time = tasks
        .into_par_iter()
        .map(|t| {
            let (task, path) = t;

            let result;
            let elapsed;
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let mut buf = BufReader::new(file);

                    let time = std::time::Instant::now();
                    result = task.run(&mut buf);
                    elapsed = time.elapsed();
                },
                Err(err) => {
                    result = Err(format!("{err}"));
                    elapsed = Duration::ZERO;
                },
            };

            let _ = stdout.lock().write_fmt(
                format_args!("{}\r\n",
                crate::format_detailed(result, task, elapsed))
            );

            elapsed
        })
        .sum::<Duration>();

    println!("Finished!\ntotal time: {:?}", total_time);
    Ok(())
}
