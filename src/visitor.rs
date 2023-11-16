use crate::{app::PATHS, cli::Args};
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

pub struct VisitorBuilder {
    config: Args,
}
impl VisitorBuilder {
    pub fn new(config: Args) -> Self {
        Self { config }
    }
}
impl ParallelVisitorBuilder<'_> for VisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor> {
        let visitor = Visitor::new(self.config.clone());
        Box::new(visitor)
    }
}

pub struct Visitor {
    pub paths: Vec<String>,
    config: Args,
}
impl Visitor {
    pub fn new(config: Args) -> Self {
        Self {
            paths: Vec::new(),
            config,
        }
    }
}
impl ParallelVisitor for Visitor {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        if let Ok(path) = entry {
            if path.file_name().to_string_lossy() == self.config.stopper {
                if let Some(parent) = path.path().parent() {
                    self.paths.push(parent.to_string_lossy().into_owned());
                    // println!("{:?}", path);
                }
            }
            WalkState::Continue
        } else {
            WalkState::Skip
        }
    }
}
impl Drop for Visitor {
    fn drop(&mut self) {
        PATHS.lock().unwrap().append(&mut self.paths);
    }
}
