struct Point<T> {
    x:T,
    y:T,
}
struct DoublePoint<T,U>{
    x:T,
    y:U,
}
fn main(){
    let boolean = Point{ x:true, y:false };
    // let boolean = Point{ x:true, y:1 }; 불일치 하기때문에 안됨.
    let intager = Point{ x:1, y:10 };
    let float = Point{ x:1.7, y:4.3};
    let string_slice = Point{ x:"high", y:"low"};


    let float_and_intager = DoublePoint{ x:1.4, y:10 };
    let string_and_boolean = DoublePoint{ x:"String", y:true };
    
}