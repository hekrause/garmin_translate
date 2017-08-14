
use rusqlite::Connection;

pub fn create_table(dst_filename: &str) {
    let conn = Connection::open("/home/henning/rust-programs/garmin_translator/src/db_interface/data_db.db").unwrap();



    let command = format!("CREATE TABLE IF NOT EXISTS {} (
                    timestamp           TEXT UNIQUE NOT NULL,
                    position_lat        TEXT,
                    position_long       TEXT,
                    distance            TEXT,
                    altitude            TEXT,
                    speed               TEXT,
                    heart_rate          TEXT,
                    enhanced_altitude   TEXT,
                    enhanced_speed      TEXT
                )", dst_filename);

    conn.execute(&command, &[]).unwrap();
}

pub fn insert_values(timestamp: String, position_lat: String, position_long: String, distance: String, altitude: String,
                    speed: String, heart_rate: String, enhanced_altitude: String, enhanced_speed: String, dst_filename: &str) {
    let conn = Connection::open("/home/henning/rust-programs/garmin_translator/src/db_interface/data_db.db").unwrap();

    let command = format!("INSERT INTO {} (timestamp, position_lat, position_long, distance, altitude, speed, heart_rate, enhanced_altitude, enhanced_speed)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", dst_filename);

    conn.execute(&command, &[&timestamp, &position_lat, &position_long, &distance, &altitude, &speed, &heart_rate, &enhanced_altitude, &enhanced_speed]).unwrap();
}
