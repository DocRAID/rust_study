use std::fs::File;
use std::io::Read;
use std::io;
// use std::io::ErrorKind;
// fn main(){
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file)=> file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//                 Ok(fc) =>fc,
//                 Err(e) => {
//                     panic!(
//                         "Tried to create file but there was a problem: {:?}",
//                         e
//                     )
//                 },
//             }
//         },
//         Err(error) => {
//             panic!(
//                 "There was a problem opening the file: {:?}",
//                 error
//             )
//         }
//     };
// }

// fn main() {
//     // let f = File::open("hello.txt").unwrap();
//     let f = File::open("hello.txt").expect("Failed to open hello.txt..haha ");
// }

fn read_username_from_file() -> Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
    //? 는 함수에서만 호출이 가능하다. main에서 호출 불가 
    // 이유는 함수를 호출하는곳에 에러를 반환하기 때문.

}
fn main(){
    
}