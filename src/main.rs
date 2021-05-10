use std::io::{Write, Read};
use std::collections::HashMap;
use std::str::FromStr;
use std::path::Path;
use std::fs::{self, File};

use clap::clap_app;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

mod template;
mod view;

use view::View;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Progress {
    I,
    II,
    III,
    IV
}

impl FromStr for Progress {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "I" => Ok(Self::I),
            "II" => Ok(Self::II),
            "III" => Ok(Self::III),
            "IV" => Ok(Self::IV),
            _ => Err(format!("Could not parse progress {}", input)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paper {
    title: String,
    description: String,
    doi: String,
    progress: Progress,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    refs: HashMap<String, Vec<String>>,
    #[serde(skip)]
    last_changed: Option<DateTime<Utc>>,
}

impl Paper {
    pub fn save_to(&self, workspace: &Path) -> Result<String, String> {
        let path = workspace.join(&self.hash());

        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }

        let paper_conf = toml::to_string(&self).unwrap();

        let mut f = File::create(path.join("index.toml")).unwrap();
        f.write_all(&paper_conf.as_bytes()).unwrap();

        dbg!(&self.hash(), &workspace);
        Ok(self.hash())
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.title);
        let out = hasher.finalize();

        let mut out = format!("{:x}", out);
        out.truncate(16);
        out
    }

    pub fn from_id(workspace: &Path, id: &str) -> Paper {
        let path = if id.len() < 16 {
            let candidates = fs::read_dir(workspace).unwrap()
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.file_name().to_str().unwrap().starts_with(id))
                .collect::<Vec<_>>();

            if candidates.len() > 1 {
                panic!("Found more than one candidate for {}", id);
            }

            candidates[0].path()
        } else {
            workspace.join(id)
        };

        Self::from_path(&path.join("index.toml"))
    }

    pub fn from_path(path: &Path) -> Paper {
        let mut f = File::open(path).unwrap();
        let mut buf = String::new();

        f.read_to_string(&mut buf).unwrap();

        let mut tmp: Paper = toml::from_str(&buf).unwrap();
        tmp.last_changed = Some(f.metadata().unwrap().modified().unwrap().into());

        tmp
    }

    pub fn mark(&mut self, progress: &str) {
        let progress = Progress::from_str(progress).unwrap();
        self.progress = progress;
    }

    pub fn title(&self, max_length: usize) -> String {
        let max_length = max_length - 3;
        let mut title = self.title.clone();
        if self.title.len() < max_length {
            for _ in 0..max_length - self.title.len() {
                title.push(' ');
            }
        } else {
            title.truncate(max_length);
            title.push_str("...");
        }

        title
    }
}

fn main() {
    let matches = clap_app!(pap =>
        (version: "0.1")
        (author: "Lorenz Schmidt (lorenz.schmidt@mailbox.org)")
        (about: "Paper management system")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand view =>
            (about: "print view of stored papers")
            (@arg PATTERN: "Search pattern to narrow down results")
            (@arg graph: -g --graph "Display the citation graph. Requires a single match.")
            (@arg seqs: -s --seqs "Display the status of matched papers in sequential order")
        )
        (@subcommand add =>
            (about: "add a paper template in an editor")
            (@arg PATH: "Load file from a path")
        )
        (@subcommand mark =>
            (about: "mark the progress of a paper")
            (@arg PAPER_ID: +required "Identifier of the paper to be marked")
            (@arg PROGRESS: +required "The progress of a paper can be I/II/III/IV\n\nI:\tadded, but need to read references before continuing\nII:\tread and compressed on paper or whiteboard\nIII:\trecalled or explained to somebody else, looked into open-review\nIV:\tfeed-back from publication venue, text results or reversed-citation graph")
        )
    ).get_matches();

    let workspace = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|x| x.file_name().eq(".pap.toml"))
        .map(|x| x.path().parent().unwrap().to_owned())
        .next().expect("Could not find any workspace in subdirectories");

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 | _ => println!("Don't be crazy"),
    }
    
    if let Some(ref matches) = matches.subcommand_matches("add") {
        let path = matches.value_of("PATH")
            .map(|x| Path::new(x));

        template::open_in_editor(&workspace, path);
    } else if let Some(ref matches) = matches.subcommand_matches("view") {
        let mut view = View::complete_from(&workspace);

        if let Some(pattern) = matches.value_of("PATTERN") {
            view = view.view(pattern, 0.6);
        }

        view.print_sequential();
    } else if let Some(ref matches) = matches.subcommand_matches("mark") {
        let mut paper = Paper::from_id(&workspace, matches.value_of("PAPER_ID").unwrap());

        paper.mark(matches.value_of("PROGRESS").unwrap());
        paper.save_to(&workspace).unwrap();
    }


}
