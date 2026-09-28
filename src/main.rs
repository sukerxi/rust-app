
fn main() {
    println!("Bleach!");

    let x=10;
    println!("{:?}",x);
    let y: i32 = if x==5{
        100
    }else{
        200
    };
    println!("{:?}",y);

    let is_finished=true;
    if is_finished{
        println!("Game Over!");
    }else {
        println!("Game On!");
    }
    
}
