use super::*;

impl Default for TinyConfig {
    fn default() -> Self {
        Self { writable: false, log_level: LevelFilter::Info, database: Default::default() }
    }
}

impl TinyConfig {
    pub fn with_writable(mut self, on: bool) -> Self {
        self.writable = on;
        self
    }
    pub fn with_database(mut self, on: bool) -> Self {
        self.database = on;
        self
    }
    pub fn with_log_level(mut self, level: LevelFilter) -> Self {
        self.log_level = level;
        self
    }
    pub fn database() -> TinyResult<PathBuf> {
        let dir = find_directory_or_create(&current_exe()?, "target")?;
        Ok(dir.join("tiny-png.db"))
    }
}

impl TinyConfig {
    pub fn initialize(&mut self, workspace: PathBuf) -> TinyResult<TinyWorkspace> {
        logger(self.log_level);
        log::info!("Workspace initialized\n{}", workspace.display());
        let mut out =
            TinyWorkspace { workspace, writable: false, database: TinyConfig::database()?, files: Default::default() };
        out.load_database()?;
        Ok(out)
    }
}

impl TinyWorkspace {
    fn load_database(&mut self) -> TinyResult {
        if self.database.to_string_lossy().is_empty() {
            return Ok(());
        }
        if !self.database.exists() {
            return Ok(());
        }
        let _ = read(&self.database)?;
        // self.files =
        log::info!("Database initialized\n{}", self.database.display());
        Ok(())
    }
    fn drop_database(&self) -> TinyResult {
        // self.database
        Ok(())
    }
}

impl Drop for TinyWorkspace {
    fn drop(&mut self) {
        if let Err(e) = self.drop_database() {
            eprintln!("{}", e)
        }
    }
}
