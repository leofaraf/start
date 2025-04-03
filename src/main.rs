use start::sysutils::find::find_collection::find_collection;

fn main() {
    let mut db = start::in_memory();
    let col = find_collection(&mut db.ss, "sys-master");
    println!("opt col: {:?}", col)
}
