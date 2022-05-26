fn main(){
    // let formal = false;
    // let greeting = if formal{
    //     "good"
    // } else{
    //     "bad"
    // };
    // println!("{}",greeting);

    let myScore = 100;
    let mut myGreade:char = 'F';
    if myScore > 90{
        myGreade='A';
    } else if myScore > 80{
        myGreade='B';
    } else if myScore > 70{
        myGreade='C';
    };
    println!("{}",myGreade);
}