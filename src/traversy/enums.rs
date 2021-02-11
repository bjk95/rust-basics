enum Movement{
    Up,
    Down
}
 
fn move_item(m: Movement){
    match m {
        Movement::Up => print!("we going up"),
        Movement::Down => print!("we going down")
    }
}
pub fn run(){
    let yo_mumma = Movement::Down;
    let me = Movement::Up;

    move_item(yo_mumma);
    move_item(me);
} 