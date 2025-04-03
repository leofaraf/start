use start::sysutils::{find::find_collection::find_collection, insert::collection::insert_collection};

fn main() {
    let mut db = start::in_memory();
    println!("Find Trash");
    let col = find_collection(&mut db.ss, "sys-trash");
    println!("trash: {:?}", col);

    println!("Insert students");
    insert_collection(&mut db.ss, "students");
    println!("Find students");
    let students = find_collection(&mut db.ss, "students");
    println!("students col: {:?}", students);
}
