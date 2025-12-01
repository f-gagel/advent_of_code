use std::{fmt::Debug, io::BufRead};

pub mod bit_grid;
pub mod bit_set;
pub mod debug;
pub mod geometry_2d;
pub mod input;
pub mod iter_ext;
pub mod macros;
pub mod num_enum;
pub mod pathfinding;

#[derive(Debug)]
pub struct Year {
    pub name: &'static str,
    pub days: &'static [Day],
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

#[derive(Debug)]
pub struct Day {
    pub name: &'static str,
    pub tasks: &'static [Task],
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

type TaskFn = dyn Sync + Fn(&mut dyn BufRead) -> Result<String, String>;
pub struct Task {
    pub module: &'static str,
    pub name: &'static str,
    pub func: &'static TaskFn,
}
impl Task {
    pub fn run(&self, input: &mut impl BufRead) -> Result<String, String> {
        (self.func)(input)
    }
}
impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("module", &self.module)
            .field("name", &self.name)
            .finish()
    }
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

#[macro_export]
macro_rules! decl_year {
    (
        $(
            $(#[doc($path:literal)])?
            $day:ident {
                $(
                    $task:ident;
                )*
            }
        )*
    ) => {
        $(
            $(#[doc = include_str!($path)])?
            pub mod $day;
        )*

        #[doc(hidden)]
        pub static YEAR: $crate::Year = $crate::Year {
            name: module_path!(),
            days: &[
                $($crate::Day {
                    name: stringify!($day),
                    tasks: &[
                        $($crate::Task {
                            module: module_path!(),
                            name: stringify!($task),
                            func: & |mut read| {
                                match $crate::input::Input::parse(&mut read) {
                                    Ok(input) => match $day :: $task (input) {
                                        Ok(res) => Ok(format!("{}", res)),
                                        Err(err) => Err(format!("{}", err)),
                                    },
                                    Err(err) => Err(format!("{}", err)),
                                }
                            }
                        },)*
                    ]
                },)*
            ]
        };
    };
}
