fn process(input:String){}
fn caller(){
    let s=String::from("hello owner!");
    process(s);
    // process(s); 함수 소유권에 의해 이미 소유권이 이전되어버림 (못부름)
}

fn copyable_process(input:u32){}
fn copyable_caller(){
    let n = 1;
    copyable_process(n);
    copyable_process(n);
    //int형 같이 단순한 형식은 복사가 가능하다.
}

fn copyable2_process(input:String){}
fn copyable2_caller(){
    let s=String::from("hello owner!");
    process(s.clone());
    process(s); //clone 을 쓰면 복사를 하기 때문에 부를수 있음.
}

fn main(){
    caller();
    copyable_caller();
    copyable2_caller();
    let mascot = String::from("Ferris");
    let ferris = mascot;

    // println!("{}",mascot); 이미 소유권이 이전되어 호출불가
}