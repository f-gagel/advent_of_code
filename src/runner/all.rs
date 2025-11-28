use rayon::prelude::*;
use std::convert::Infallible;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, clap_derive::Parser)]
pub struct Args;

pub fn run(_: Args) -> Result<(), Infallible> {
    let tasks = crate::YEARS
        .iter()
        .flat_map(move |y| {
            y.days.iter().flat_map(move |d| {
                let mut path = PathBuf::from_iter([y.name, "inputs", d.name]);
                path.set_extension("txt");
                d.tasks.iter().map(move |t| (y, d, t, path.clone()))
            })
        })
        .collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let total_time = tasks
        .into_par_iter()
        .map(move |t| {
            let (year, day, task, path) = t;

            let result;
            let elapsed;
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let mut buf = BufReader::new(file);

                    let time = std::time::Instant::now();
                    result = task.run(&mut buf);
                    elapsed = time.elapsed();
                }
                Err(err) => {
                    result = Err(format!("{err}"));
                    elapsed = Duration::ZERO;
                }
            };

            let _ = stdout.lock().write_fmt(format_args!(
                "{}\r\n",
                crate::format_detailed(result, year, day, task, elapsed)
            ));

            elapsed
        })
        .sum::<Duration>();

    println!("Finished!\ntotal time: {:?}", total_time);
    Ok(())
}
