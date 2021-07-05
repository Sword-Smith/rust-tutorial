#[derive(Debug)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

pub fn to_drop() {
    let x = ToDrop;
    println!("x: {:?}", x);
    println!("Made a ToDrop!");
}
