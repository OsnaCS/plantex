use super::{Chunk, ChunkIndex};

/// A type that can load a game world, specifically single chunks of it. This
/// could mean loading a saved world from a file, generating a world
/// procedurally or loading a world from a server.
pub trait Provider {
    /// Attempt to load a chunk from the world. This may fail (e.g. when
    /// loading from a file and the chunk is not yet saved in the file).
    ///
    /// The given position is not a world position, but a chunk index. See the
    /// documentation of `ChunkIndex` for more information.
    ///
    /// # Warning
    ///
    /// This function may take some time to complete! To check whether a chunk
    /// can be loaded at all, prefer `is_chunk_loadable()`.
    fn load_chunk(&self, pos: ChunkIndex) -> Option<Chunk>;

    /// Determines whether or not the chunk at the given position can be
    /// loaded. This function is expected to return quickly.
    fn is_chunk_loadable(&self, pos: ChunkIndex) -> bool;
}

/// A dummy provider that always fails to provide a chunk.
#[derive(Clone, Copy, Debug)]
pub struct NullProvider;

impl Provider for NullProvider {
    fn load_chunk(&self, _: ChunkIndex) -> Option<Chunk> {
        None
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        false
    }
}

/// A fallback provider that holds two chunk providers with one being primary
/// and one fallback. If the chunk load from the primary fails the fallback
/// is being called.
#[derive(Clone, Debug, Copy)]
pub struct FallbackProvider<P, F> {
    primary: P,
    fallback: F,
}

impl<P: Provider, F: Provider> Provider for FallbackProvider<P, F> {
    fn load_chunk(&self, pos: ChunkIndex) -> Option<Chunk> {
        if self.primary.is_chunk_loadable(pos) {
            self.primary.load_chunk(pos)
        } else {
            self.fallback.load_chunk(pos)
        }
    }

    fn is_chunk_loadable(&self, pos: ChunkIndex) -> bool {
        self.primary.is_chunk_loadable(pos) || self.fallback.is_chunk_loadable(pos)
    }
}
