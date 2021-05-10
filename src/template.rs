use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use tempfile::NamedTempFile;

use crate::Paper;

const TEMPLATE: &'static str = r#"doi = ""
title = ""
description = """
"""
progress = 1

tags = [
]

[refs]
"#;


pub fn open_in_editor(workspace: &Path, path: Option<&Path>) {
    let content = if let Some(path) = path {
        let mut f = File::open(path).expect("Could not read meta for new paper");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Could not read to string");

        buf
    } else {
        TEMPLATE.to_string()
    };

    let mut editor_buf = NamedTempFile::new().expect("Could not create temporary file for EDITOR");
    editor_buf.write_all(content.as_bytes()).unwrap();
    let editor_buf_path = editor_buf.path();

    let mut prev_buf = content;

    loop {
        Command::new("vim")
            .arg(editor_buf_path)
            .status().expect("Could not open temporary file");

        let changed_buf = {
            let mut tmp = String::new();
            let mut f = File::open(&editor_buf_path).unwrap();

            f.read_to_string(&mut tmp).unwrap();

            tmp
        };

        if changed_buf == prev_buf {
            break;
        } else {
            prev_buf = changed_buf;
        }

        match toml::from_str(&prev_buf) {
            Ok(res) => {
                add_new_paper(workspace, res);

                break;
            }
            Err(err) => {
                eprintln!("Got error: {}", err);
                thread::sleep(Duration::from_millis(1000));

                continue;
            }
        }
    }
}

pub fn add_new_paper(workspace: &Path, paper: Paper) {
    paper.save_to(workspace).unwrap();
}
