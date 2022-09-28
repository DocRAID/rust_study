struct Foo {
    x:i32,
}
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}
fn main() {
    let mut foo = Foo {x:43};
    let x = &mut foo.x; //빌려왔음.
    *x = 13 ; //역참조 해서 빌린걸 중간에 바꿔치기
    println!("{}",x); //이미 바뀜
    let y = do_something(&foo);
    println!("{}",y);
}
//명시적 생명 주기
//아무것도 아닌것을 가르키는 참조를 들고 다니는 구조체는 있을수 없음.