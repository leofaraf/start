use start::{systypes::document::RawDocument, sysutils::find::find_collection::find_collection, utils::insert::insert_one};

fn main() {
    let mut db = start::in_memory();

    insert_one(&mut db.ss, "students", "peter".as_bytes().to_vec());
    insert_one(&mut db.ss, "students", "gebert".as_bytes().to_vec());
    insert_one(&mut db.ss, "students", "leon".as_bytes().to_vec());
    let students = find_collection(&mut db.ss, "students");
    println!("students col: {:?}", students);
}
