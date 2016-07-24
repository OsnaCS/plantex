use base::world::{Chunk, ChunkProvider, World};
use super::GameContext;
use std::rc::Rc;
use std::thread;
use std::cell::{Ref, RefCell};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use base::world::ChunkIndex;
use base::math::*;
use world::WorldView;

#[derive(Clone)]
pub struct WorldManager {
    shared: Rc<RefCell<Shared>>,
    chunk_requests: Sender<ChunkIndex>,
    context: GameContext,
}

struct Shared {
    world: World,
    world_view: WorldView,
    provided_chunks: Receiver<(ChunkIndex, Chunk)>,
    #[allow(dead_code)]
    load_distance: f32,
}

impl WorldManager {
    pub fn new(provider: Box<ChunkProvider>, game_context: GameContext) -> Self {
        // Create two channels to send chunk positions and receive chunks.
        let (chunk_request_sender, chunk_request_recv) = channel();
        let (chunk_sender, chunk_recv) = channel();

        // Spawn worker thread, which will detach from the main thread. But
        // that is no problem: once the last sender is destroyed, the worker
        // thread will quit.
        thread::spawn(move || {
            worker_thread(provider, chunk_request_recv, chunk_sender);
        });

        WorldManager {
            shared: Rc::new(RefCell::new(Shared {
                world: World::empty(),
                world_view: WorldView::from_world(&World::empty(), game_context.get_facade()),
                provided_chunks: chunk_recv,
                // TODO: load this from the config!
                load_distance: 20.0,
            })),
            chunk_requests: chunk_request_sender,
            context: game_context,
        }
    }

    // TODO: this is temporary
    pub fn pregenerate_world(&self) {
        for q in 0..5 {
            for r in 0..5 {
                self.chunk_requests.send(ChunkIndex(AxialPoint::new(q, r))).unwrap();
            }
        }
    }

    /// Returns an immutable reference to the world.
    ///
    /// *Note*: since the world manager uses a `RefCell` to save the world, a
    /// `Ref` is returned. But thanks to deref coercions you can use it like a
    /// standard reference.
    pub fn get_world<'a>(&'a self) -> Ref<'a, World> {
        Ref::map(self.shared.borrow(), |shared| &shared.world)
    }

    /// Returns an immutable reference to the world view.
    pub fn get_view<'a>(&'a self) -> Ref<'a, WorldView> {
        Ref::map(self.shared.borrow(), |shared| &shared.world_view)
    }

    /// Starts to generate all chunks within `load_distance` (config parameter)
    /// around `pos`. TODO
    pub fn load_world_around(&self, _pos: Point2f) -> bool {
        unimplemented!();
    }

    /// Applies all queued updated to the actual world. Notably, all generated
    /// chunks are added.
    pub fn update_world(&self) {
        let mut shared = self.shared.borrow_mut();
        let mut view_needs_update = false;

        loop {
            let (pos, chunk) = match shared.provided_chunks.try_recv() {
                Err(TryRecvError::Empty) => {
                    // No new chunks for us, we will check again next time ;-)
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    // The worker thread has shut down: this should never happen
                    // thus we can just panic, too.
                    error!("chunk providing worker thread shut down");
                    panic!("chunk providing worker thread shut down");
                }
                Ok(val) => val,
            };

            let res = shared.world.add_chunk(pos, chunk);
            if res.is_err() {
                warn!("chunk at {:?} already exists!", pos);
            }

            view_needs_update = true;
        }

        // TODO: this is temporary! We shouldn't recreate the whole view!
        if view_needs_update {
            shared.world_view = WorldView::from_world(&shared.world, self.context.get_facade());
        }
    }
}


fn worker_thread(provider: Box<ChunkProvider>,
                 commands: Receiver<ChunkIndex>,
                 chunks: Sender<(ChunkIndex, Chunk)>) {
    loop {
        let requested_chunk = match commands.recv() {
            Err(_) => {
                // The other side has hung up, so we can stop working, too.
                break;
            }
            Ok(index) => index,
        };

        debug!("chunk provider thread: received request to generate chunk {:?}",
               requested_chunk.0);

        match provider.load_chunk(requested_chunk) {
            Some(chunk) => {
                debug!("chunk provider thread: chunk at {:?} successfully loaded",
                       requested_chunk);
                chunks.send((requested_chunk, chunk)).expect("main thread has hung up");
            }
            None => {
                warn!("chunk provider thread: failed to load chunk at {:?}",
                      requested_chunk)
            }
        }
    }

    info!("chunk provider thread: now stopping ...")
}
