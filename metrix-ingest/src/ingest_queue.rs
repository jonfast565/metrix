use crossbeam::queue::SegQueue;
use diesel::Connection;
use diesel::PgConnection;
use metrix_models::MetricInsert;
use metrix_utils::envs::get_database_url;

static INSERT_QUEUE: SegQueue<MetricInsert> = SegQueue::<MetricInsert>::new();

pub struct InsertQueueManager {}

impl InsertQueueManager {
    pub fn insert(m: MetricInsert) {
        INSERT_QUEUE.push(m);
    }

    pub fn read() -> Option<MetricInsert> {
        INSERT_QUEUE.pop()
    }

    pub fn process_queue_thread() {
        std::thread::spawn(move || {
            InsertQueueManager::process_queue();
        });
    }

    fn process_queue() {
        let connection =
            PgConnection::establish(&get_database_url()).expect("Couldn't connect to database.");
        loop {
            match InsertQueueManager::read() {
                Some(record) => {
                    info!("Read record from queue manager");
                    metrix_database::insert_metric(&record, &connection);
                }
                None => {}
            }
        }
    }
}
