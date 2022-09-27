use std::{
    fmt::Display,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use crossbeam::channel::{bounded, Sender};
use flate2::write::GzEncoder;

#[derive(Debug)]
pub struct OutputFile {
    pub table: Option<String>,
    pub database: Option<String>,
}

impl Display for OutputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.database {
            Some(db) => {
                write!(f, "{db}/")?;
                match &self.table {
                    Some(table) => write!(f, "{table}.sql"),
                    None => f.write_str("00_create_database.sql"),
                }
            }
            None => match &self.table {
                Some(table) => write!(f, "{table}.sql"),
                None => f.write_str("00_header.sql"),
            },
        }
    }
}

pub struct Output {
    sender: Sender<Vec<u8>>,
}

impl Output {
    pub fn new<P: AsRef<Path>>(
        output_dir: P,
        output_file: &OutputFile,
        compress: bool,
    ) -> std::io::Result<Self> {
        let path = {
            let mut path = PathBuf::from(output_dir.as_ref());

            if compress {
                path.push(format!("{}.gz", output_file.to_string()));
            } else {
                path.push(output_file.to_string());
            }
            path
        };

        create_dir_all(path.parent().expect("Cannot access to parent path!"))?;

        println!("Opening {}", path.to_string_lossy());
        let mut output: Box<dyn Write + Send> = if compress {
            Box::new(GzEncoder::new(File::create(path)?, Default::default()))
        } else {
            Box::new(File::create(path)?)
        };

        let (sender, receiver) = bounded::<Vec<u8>>(1000);
        std::thread::spawn(move || {
            while let Ok(bytes) = receiver.recv() {
                if let Err(e) = output.write_all(bytes.as_slice()) {
                    eprintln!("An error occurred while writing: {e}");
                    break;
                }
            }
        });
        Ok(Self { sender })
    }

    pub fn write_bytes(&mut self, buf: &[u8]) -> color_eyre::Result<()> {
        self.sender.send(buf.to_vec())?;
        Ok(())
    }
}
