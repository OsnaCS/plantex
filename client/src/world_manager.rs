use base::world::{Chunk, ChunkProvider, World};
use super::GameContext;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::mem::replace;
use std::thread;
use std::cell::{Ref, RefCell};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use base::world::{CHUNK_SIZE, ChunkIndex};
use base::math::*;
use world::WorldView;

#[derive(Clone)]
pub struct WorldManager {
    shared: Rc<RefCell<Shared>>,
    chunk_requests: Sender<ChunkIndex>,
    context: Rc<GameContext>,
}

struct Shared {
    world: World,
    world_view: WorldView,
    provided_chunks: Receiver<(ChunkIndex, Chunk)>,
    sent_requests: HashSet<ChunkIndex>,
    load_distance: f32,
    player_chunk: ChunkIndex,
}

impl WorldManager {
    pub fn new(provider: Box<ChunkProvider>, game_context: Rc<GameContext>) -> Self {
        // Create two channels to send chunk positions and receive chunks.
        let (chunk_request_sender, chunk_request_recv) = channel();
        let (chunk_sender, chunk_recv) = channel();

        // Spawn worker thread, which will detach from the main thread. But
        // that is no problem: once the last sender is destroyed, the worker
        // thread will quit.
        thread::spawn(move || {
            worker_thread(provider, chunk_request_recv, chunk_sender);
        });

        let this = WorldManager {
            shared: Rc::new(RefCell::new(Shared {
                world: World::empty(),
                world_view: WorldView::from_world(&World::empty(), game_context.clone()),
                sent_requests: HashSet::new(),
                provided_chunks: chunk_recv,
                // TODO: load this from the config!
                load_distance: 10.0,
                player_chunk: ChunkIndex(AxialPoint::new(0, 0)),
            })),
            chunk_requests: chunk_request_sender,
            context: game_context,
        };

        this.update_player_chunk();
        this
    }

    /// Called when the player moves to a different chunk.
    ///
    /// This unloads all currently loaded chunks that are too far away from the
    /// player, and loads all chunks close enough to the player (if they aren't
    /// already requested).
    fn update_player_chunk(&self) {
        let mut shared = self.shared.borrow_mut();
        let player_chunk = shared.player_chunk.0;
        let radius = shared.load_distance as i32;

        // Load new range
        for qd in -radius..radius {
            for rd in -radius..radius {
                let chunk_pos = AxialPoint::new(player_chunk.q + qd, player_chunk.r + rd);
                if (chunk_pos - player_chunk).to_real().distance(Vector2::zero()) <
                   shared.load_distance {
                    let chunk_index = ChunkIndex(chunk_pos);

                    if !shared.world.chunks.contains_key(&chunk_index) {
                        if !shared.sent_requests.contains(&chunk_index) {
                            self.chunk_requests.send(chunk_index).unwrap();
                            shared.sent_requests.insert(chunk_index);
                        }
                    }
                }
            }
        }

        // Drop unneeded chunks from world
        let chunks = replace(&mut shared.world.chunks, HashMap::new());
        let mut new_chunks = HashMap::new();
        for (index, chunk) in chunks {
            let chunk_pos = index.0;
            if (chunk_pos - player_chunk).to_real().distance(Vector2::zero()) <
               shared.load_distance {
                // Still in range
                new_chunks.insert(index, chunk);
            } else {
                // Remove
                shared.world_view.remove_chunk(index);
            }
        }
        shared.world.chunks = new_chunks;
    }


    /// Returns an immutable reference to the world.
    ///
    /// *Note*: since the world manager uses a `RefCell` to save the world, a
    /// `Ref` is returned. But thanks to deref coercions you can use it like a
    /// standard reference.
    pub fn get_world(&self) -> Ref<World> {
        Ref::map(self.shared.borrow(), |shared| &shared.world)
    }

    /// Returns an immutable reference to the world view.
    pub fn get_view(&self) -> Ref<WorldView> {
        Ref::map(self.shared.borrow(), |shared| &shared.world_view)
    }

    /// Starts to generate all chunks within `load_distance` (config parameter)
    /// around `pos`.
    fn load_world_around(&self, pos: Point2f) {
        let axial_pos = AxialPoint::from_real(pos);
        let chunk_pos = AxialPoint::new(axial_pos.q / CHUNK_SIZE as i32,
                                        axial_pos.r / CHUNK_SIZE as i32);

        let mut shared = self.shared.borrow_mut();
        if shared.player_chunk.0 != chunk_pos {
            shared.player_chunk = ChunkIndex(chunk_pos);
            debug!("player moved to chunk {:?}", chunk_pos);
            drop(shared);

            self.update_player_chunk();
        }
    }

    /// Applies all queued updated to the actual world. Notably, all generated
    /// chunks are added.
    pub fn update_world(&self, player_pos: Point3f) {
        self.load_world_around(Point2f::new(player_pos.x, player_pos.y));

        let mut shared = self.shared.borrow_mut();

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

            shared.world_view.refresh_chunk(pos, &chunk, self.context.get_facade());

            shared.sent_requests.remove(&pos);
            let res = shared.world.add_chunk(pos, chunk);
            if res.is_err() {
                warn!("chunk at {:?} already exists!", pos);
            }
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
