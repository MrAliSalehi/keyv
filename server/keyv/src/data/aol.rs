use crate::engine::parser;
use crate::instruction::RawInstruction;
use crate::prelude::Res;
use itertools::Itertools;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Aol {
    file_index: usize,
}
impl Aol {
    pub async fn new(path: &str) -> eyre::Result<Self> {
        let path = PathBuf::from_str(path)?;
        if !tokio::fs::try_exists(&path).await? {
            tokio::fs::DirBuilder::new()
                .recursive(true)
                .create(&path)
                .await?;
        }
        Ok(Self {
            file_index: 0,
        })
    }
    pub async fn load_commits(&self, path: impl Into<PathBuf>) -> Res {
        let mut files = tokio::fs::read_dir(path.into()).await?;

        while let Ok(Some(file)) = files.next_entry().await {
            let f = file.file_name();
            let Some(name) = f.to_str() else {
                continue;
            };

            let Some(index) = name.parse::<usize>().ok() else {
                continue;
            };

            let content = tokio::fs::read(file.path()).await?;
/*
            let mut instructions = content
                .lines()
                .into_iter()
                .filter_map(|e| parser::parse_instruction(e.as_bytes()))
                .collect_vec();
            self.initial_commits.append(&mut instructions);*/
            //todo continue aol loading
        }
        Ok(())
    }
}
