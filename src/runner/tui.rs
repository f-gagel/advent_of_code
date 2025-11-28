use std::io::BufReader;
use std::{fmt::Display, io::stderr, path::PathBuf};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    widgets::{Block, BorderType, List, ListItem, ListState},
};

use tui::widgets::{Borders, StatefulWidget, Widget};

#[derive(Debug, clap_derive::Parser)]
pub struct Args;

struct StatefulList<'a, T> {
    title: &'a str,
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<'_, T> {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<T: Display> Widget for &mut StatefulList<'_, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self
            .items
            .iter()
            .map(|t| ListItem::new(format!("{}", t)))
            .collect::<Vec<_>>();
        let list = List::new(items)
            .block(
                Block::default()
                    .title(self.title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_symbol(" >> ");
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

fn handle_list_events<T>(list: &mut StatefulList<T>) -> Result<Option<usize>, std::io::Error> {
    let key = match crossterm::event::read()? {
        Event::Key(key) => key,
        _ => return Ok(None),
    };
    if key.kind != KeyEventKind::Press {
        return Ok(None);
    }
    match key.code {
        KeyCode::Up => list.previous(),
        KeyCode::Down => list.next(),
        KeyCode::Enter => return Ok(list.state.selected()),
        _ => {}
    }
    Ok(None)
}

fn select<B, T, I>(title: &str, term: &mut Terminal<B>, iter: I) -> Result<T, std::io::Error>
where
    B: Backend,
    T: Display,
    I: IntoIterator<Item = T>,
{
    let mut list = StatefulList {
        title,
        items: iter.into_iter().collect::<Vec<_>>(),
        state: ListState::default(),
    };

    let value = loop {
        term.draw(|f| {
            let area = f.size();
            f.render_widget(&mut list, area);
        })?;

        if let Some(i) = handle_list_events(&mut list)? {
            break list.items.drain(i..=i).next().unwrap();
        }
    };
    Ok(value)
}

pub fn select_file<B>(term: &mut Terminal<B>) -> Result<PathBuf, std::io::Error>
where
    B: Backend,
{
    let mut current_dir = std::env::current_dir()?;
    current_dir.push("inputs");
    let backtrack = "..".to_string();

    loop {
        let dir = std::fs::read_dir(current_dir.clone())?;
        let names = dir.filter_map(|r| r.ok().map(|d| d.file_name()));
        let entries = names.map(|e| e.to_str().unwrap().to_owned());
        let entries = [backtrack.clone()].into_iter().chain(entries);
        let selected = select("Select input", term, entries)?;

        current_dir.push(selected);

        if current_dir.is_file() {
            break;
        }
    }

    Ok(current_dir)
}

pub fn run(_: Args) -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;

    let year = select("Select year", &mut terminal, crate::YEARS)?;
    let day = select("Select day", &mut terminal, year.days)?;
    let task = select("Select task", &mut terminal, day.tasks)?;

    let input = select_file(&mut terminal)?;

    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    let file = std::fs::File::open(&input).unwrap();
    let mut buf = BufReader::new(file);
    let result = task.run(&mut buf);

    println!("{}", crate::format_simple(result));

    println!("Press enter to exit...");
    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
