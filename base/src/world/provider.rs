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
    fn load_chunk(pos: ChunkIndex) -> Option<Chunk>;

    /// Determines whether or not the chunk at the given position can be
    /// loaded. This function is expected to return quickly.
    fn is_chunk_loadable(pos: ChunkIndex) -> bool;
}
