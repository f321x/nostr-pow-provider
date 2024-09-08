use super::*;

pub struct Hasher {
    queue: Mutex<Vec<HashTask>>,
    finished: Mutex<HashMap<EventId, nostr_sdk::UnsignedEvent>>,
}

struct HashTask {
    pub event: nostr_sdk::UnsignedEvent,
    pub leading_zeros: u8,
}

pub async fn pow_scheduler(hasher: Arc<Hasher>) {
    loop {
        let task = {
            let mut queue = hasher.queue.lock().expect("Failed to lock queue");
            queue.pop()
        };

        if let Some(task) = task {
            let (send, recv) = tokio::sync::oneshot::channel();

            // Spawn a task on rayon.
            rayon::spawn(move || {
                let result =
                    hash_event(task.event, task.leading_zeros).expect("Failed to find PoW");
                // Send the result back to Tokio.
                let _ = send.send(result);
            });

            // Wait for the rayon task.
            let result_event = recv.await.expect("Panic in rayon::spawn");
            let mut finished = hasher.finished.lock().expect("Failed to lock finished");
            finished.insert(
                result_event
                    .id
                    .clone()
                    .expect("No event ID present in PoW event"),
                result_event,
            );
        } else {
            tokio::time::sleep(Duration::from_secs(1));
        }
    }
}

fn hash_event(event: nostr_sdk::UnsignedEvent, difficulty: u8) -> Result<UnsignedEvent> {
    let tags = event.tags;
    let kind = event.kind;
    let content = event.content;
    let created_at = event.created_at;
    let pubkey = event.pubkey;

    let now = Instant::now();

    let result = (1u128..).into_iter().par_bridge().find_map_any(|nonce| {
        let mut tags = tags.clone();
        tags.push(Tag::pow(nonce, difficulty));

        let id: EventId = EventId::new(&pubkey, &created_at, &kind, &tags, &content);

        if id.check_pow(difficulty) {
            Some((nonce, id, tags))
        } else {
            None
        }
    });

    if let Some((nonce, id, tags)) = result {
        debug!(
            "{} iterations in {} ms. Avg rate {} hashes/second",
            nonce,
            now.elapsed().as_millis(),
            nonce * 1000 / std::cmp::max(1, now.elapsed().as_millis())
        );

        Ok(UnsignedEvent {
            id: Some(id),
            pubkey,
            created_at,
            kind,
            tags,
            content,
        })
    } else {
        Err(anyhow!("Failed to find valid PoW"))
    }
}

impl Hasher {
    pub fn new() -> Mutex<Arc<Self>> {
        let hasher = Arc::new(Hasher {
            queue: Mutex::new(Vec::new()),
            finished: Mutex::new(HashMap::new()),
        });
        tokio::task::spawn(async { pow_scheduler(hasher.clone()).await });
        Mutex::new(hasher)
    }

    pub fn add_task(&mut self, event: nostr_sdk::UnsignedEvent, leading_zeros: u8) {
        let mut queue = self
            .queue
            .lock()
            .expect("Failed to lock queue mutex on add_task");

        queue.push(HashTask {
            event,
            leading_zeros,
        })
    }

    pub fn fetch_event(&mut self, event_id: &EventId) -> Option<nostr_sdk::UnsignedEvent> {
        let mut finished = self
            .finished
            .lock()
            .expect("Failed to lock finished in fetch_event");

        match finished.get(event_id) {
            Some(pow_event) => {
                let event = *pow_event;
                finished.remove(&event_id);
                Some(event)
            }
            None => None,
        }
    }
}
