extern crate postgres;

use postgres::{Connection,TlsMode};

struct Player {
    id: i32,
    name: String
}

fn main() {
    println!("stride client!");
    let conn = Connection::connect("postgresql://strideclient:foon@127.0.0.1/stride", TlsMode::None).unwrap();
    let stmt = match conn.prepare("select id, name, role from player where name=$1") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("query prepare failed: {:?}", e);
            return;
        }
    };
    let name = "admin";
    let mut rows = stmt.query(&[&name]);
    let row = rows.iter().next().unwrap();
    println!("row: {:?}", row);
}
