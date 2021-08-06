use crossbeam::queue::SegQueue;
use metrix_models::MetricInsert;
use std::env;
use diesel::PgConnection;
use diesel::Connection;

static INSERT_QUEUE: SegQueue::<MetricInsert> = SegQueue::<MetricInsert>::new();

pub struct InsertQueueManager {}

impl InsertQueueManager {
    
    pub fn insert(&self, m: MetricInsert) {
        INSERT_QUEUE.push(m);
    }

    pub fn read(&self) -> Option<MetricInsert> {
        INSERT_QUEUE.pop()
    }

    pub fn new() -> InsertQueueManager {
        InsertQueueManager{}
    }
}

pub fn process_queue_thread() {
    std::thread::spawn(move || {
        process_queue();
    });
}


fn get_database_url() -> String {
    let database_url_key = "DATABASE_URL";
    match env::var(database_url_key) {
        Ok(val) => {
            return val;
        },
        Err(e) => panic!("DATABASE_URL not passed to the application. {}", e)
    }
}

fn process_queue() {
    let connection = PgConnection::establish(&get_database_url()).expect("Couldn't connect to database.");
    let queue_mgr = InsertQueueManager{}; 

    loop {
        match queue_mgr.read() {
            Some(record) => metrix_database::insert_metric(&record, &connection),
            None => {}
        }
    };
}