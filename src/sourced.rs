use std::path::{
    Path,
    PathBuf,
};

#[derive(Debug, Clone)]
pub struct Sourced<E> {
    src: PathBuf,
    entity: E,
}
impl<E> Sourced<E> {
    pub fn new(
        entity: E,
        src: PathBuf,
    ) -> Self {
        Self { src, entity }
    }

    /// Return the source
    pub fn src(&self) -> &Path {
        &self.src
    }

    /// Return a reference to the wrapped entity
    pub fn entity(&self) -> &E {
        &self.entity
    }

    /// Return a mutable reference to the wrapped entity
    pub fn entity_mut(&mut self) -> &mut E {
        &mut self.entity
    }

    /// Return the entity, dropping the item
    pub fn take_entity(self) -> E {
        self.entity
    }

    /// Destructure the ided into the wrapped path and entity
    pub fn dismantle(self) -> (PathBuf, E) {
        (self.src, self.entity)
    }
}

impl<E> AsRef<E> for Sourced<E> {
    fn as_ref(&self) -> &E {
        &self.entity
    }
}

impl<E> std::ops::Deref for Sourced<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
