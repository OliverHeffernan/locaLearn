use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::study_set::StudySetLayout;

/// Request sent to a generation provider.
#[derive(Debug, Clone)]
pub struct GenerationRequest {
    prompt: String,
    context: StudyContext,
}

impl GenerationRequest {
    /// Creates a provider request from a prompt and study context.
    pub fn new(prompt: impl Into<String>, context: StudyContext) -> Self {
        Self {
            prompt: prompt.into(),
            context,
        }
    }

    /// Returns the full provider prompt.
    pub fn prompt(&self) -> String {
        [
            self.prompt.as_str(),
            "\n\n# Local Study Resources\n",
            self.context.combined_resources().as_str(),
        ]
        .join("")
    }
}

/// Content returned by a generation provider.
#[derive(Debug, Clone)]
pub struct GenerationResponse {
    content: String,
}

impl GenerationResponse {
    /// Creates a response from generated content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns generated content.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Aggregated local resources available to generation providers.
#[derive(Debug, Clone)]
pub struct StudyContext {
    resources: Vec<ResourceDocument>,
}

impl StudyContext {
    /// Loads readable resources from a study set.
    pub fn from_layout(layout: &StudySetLayout) -> Self {
        let reader = DefaultResourceReader::default();
        let resources = WalkDir::new(layout.resources_dir())
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| reader.read(entry.path()).ok())
            .collect();

        Self { resources }
    }

    /// Combines loaded resources into one provider-friendly string.
    pub fn combined_resources(&self) -> String {
        self.resources
            .iter()
            .map(ResourceDocument::formatted)
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Returns the number of readable resources loaded into context.
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the total number of UTF-8 bytes loaded from resources.
    pub fn loaded_bytes(&self) -> usize {
        self.resources
            .iter()
            .map(|resource| resource.content.len())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct ResourceDocument {
    path: PathBuf,
    content: String,
}

impl ResourceDocument {
    fn formatted(&self) -> String {
        format!("## {}\n{}", self.path.display(), self.content)
    }
}

/// Reads one resource format into plain text for provider prompts.
trait ResourceReader {
    /// Returns whether this reader should attempt the path.
    fn supports(&self, path: &Path) -> bool;

    /// Reads a resource into a document.
    fn read(&self, path: &Path) -> std::io::Result<ResourceDocument>;
}

/// Selects the first reader that supports a resource path.
struct DefaultResourceReader {
    readers: Vec<Box<dyn ResourceReader>>,
}

impl Default for DefaultResourceReader {
    fn default() -> Self {
        Self {
            readers: vec![Box::new(PdfResourceReader), Box::new(TextResourceReader)],
        }
    }
}

impl DefaultResourceReader {
    fn read(&self, path: &Path) -> std::io::Result<ResourceDocument> {
        self.readers
            .iter()
            .map(Box::as_ref)
            .find(|reader| reader.supports(path))
            .map(|reader| reader.read(path))
            .unwrap_or_else(|| TextResourceReader.read(path))
    }
}

/// Reads UTF-8 text resources.
struct TextResourceReader;

impl ResourceReader for TextResourceReader {
    fn supports(&self, _path: &Path) -> bool {
        true
    }

    fn read(&self, path: &Path) -> std::io::Result<ResourceDocument> {
        fs::read_to_string(path).map(|content| ResourceDocument {
            path: path.to_path_buf(),
            content,
        })
    }
}

/// Extracts plain text from PDF resources.
struct PdfResourceReader;

impl ResourceReader for PdfResourceReader {
    fn supports(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .filter(|extension| extension == "pdf")
            .is_some()
    }

    fn read(&self, path: &Path) -> std::io::Result<ResourceDocument> {
        pdf_extract::extract_text(path)
            .map(|content| ResourceDocument {
                path: path.to_path_buf(),
                content,
            })
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    }
}
