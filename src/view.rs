use std::fs;
use std::path::Path;
use std::collections::HashMap;
use strsim::jaro;
use crate::{Paper, Progress};

pub struct View {
    papers: HashMap<String, Paper>
}

impl View {
    pub fn complete_from(workspace: &Path) -> View {
        let paths = fs::read_dir(workspace).unwrap();

        // look into workspace for directories containing `index.toml`
        // convert to paper structure and collect into hashmap
        let papers = paths.into_iter()
            .filter_map(|x| x.ok())
            .filter(|x| x.file_type().unwrap().is_dir())
            .map(|x| x.path().join("index.toml"))
            .filter(|x| x.exists())
            .map(|x| Paper::from_path(&x))
            .map(|x| (x.hash(), x))
            .collect();

        View {
            papers
        }
    }

    pub fn view(self, pattern: &str, threshold: f64) -> Self {
        let papers = self.papers.into_iter()
            .filter(|(_, x)| jaro(pattern, &x.title) > threshold)
            .collect();

        View {
            papers
        }
    }

    pub fn print_sequential(&self) {
        let width = term_size::dimensions()
            .map(|x| x.0)
            .unwrap_or(80);

        println!("hash    I    II   III  IV   title");
        for _ in 0..width {
            print!("⎯");
        }
        println!("");

        let mut papers = self.papers.clone().into_iter().collect::<Vec<_>>();
        papers.sort_by_key(|x| x.1.last_changed);

        for (mut key, paper) in papers {
            key.truncate(7);

            print!("\x1b[33m{}\x1b[0m ", key);
            match paper.progress {
                Progress::I   => print!("\x1b[44m    \x1b[0m                "),
                Progress::II  => print!("\x1b[44m    \x1b[0m \x1b[44m    \x1b[0m           "),
                Progress::III => print!("\x1b[44m    \x1b[0m \x1b[44m    \x1b[0m \x1b[44m    \x1b[0m      "),
                Progress::IV  => print!("\x1b[44m    \x1b[0m \x1b[44m    \x1b[0m \x1b[44m    \x1b[0m \x1b[44m    \x1b[0m ")
            }

            println!("{}", paper.title((width - 28).max(15)));
        }
    }
}
