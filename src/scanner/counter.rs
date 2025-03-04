// src/scanner/counter.rs

#[derive(Debug, Default)]
pub struct ProcessCounter {
    total_files: usize,
    processed_files: usize,
    skipped_by_pattern: usize,
    skipped_binary: usize,
    skipped_size: usize,
    skipped_extension: usize,
}

impl ProcessCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_total_files(&mut self, count: usize) {
        self.total_files = count;
    }

    pub fn increment_processed(&mut self) {
        self.processed_files += 1;
    }

    pub fn increment_skipped_pattern(&mut self) {
        self.skipped_by_pattern += 1;
    }

    pub fn increment_skipped_binary(&mut self) {
        self.skipped_binary += 1;
    }

    pub fn increment_skipped_size(&mut self) {
        self.skipped_size += 1;
    }

    pub fn increment_skipped_extension(&mut self) {
        self.skipped_extension += 1;
    }

    pub fn print_summary(&self) {
        let total_skipped = self.skipped_by_pattern
            + self.skipped_binary
            + self.skipped_size
            + self.skipped_extension;

        eprintln!("\nProcessing summary:");
        eprintln!(
            "- Files processed: {}/{}",
            self.processed_files, self.total_files
        );

        if total_skipped > 0 {
            eprintln!("- Skipped files: {}", total_skipped);
            if self.skipped_by_pattern > 0 {
                eprintln!(
                    "  - Excluded by patterns: {} files",
                    self.skipped_by_pattern
                );
            }
            if self.skipped_binary > 0 {
                eprintln!("  - Binary files: {} files", self.skipped_binary);
            }
            if self.skipped_size > 0 {
                eprintln!("  - Size limit exceeded: {} files", self.skipped_size);
            }
            if self.skipped_extension > 0 {
                eprintln!(
                    "  - Not matching extensions: {} files",
                    self.skipped_extension
                );
            }
        }
    }
}
