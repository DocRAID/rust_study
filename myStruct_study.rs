#[derive(PartialEq,Debug)]
struct Phone{
    color:String,
    size:u32,
    screen:Display,
    security:bool,
    useage:(Useage,u32)
}
#[derive(PartialEq,Debug)]
enum Display{Lcd,Pdp,Led,Oled}
#[derive(PartialEq,Debug)]
enum Useage{New,Used}

fn phone_status(used_day:u32)-> (Useage,u32){
    let quality=(Useage::New,used_day);
    quality
}
fn phone_factory(color:String, size:u32, screen:Display, security:bool, used_day:u32)-> Phone{
    Phone{
        color:color,
        size:size,
        screen:screen,
        security:security,
        useage:phone_status(used_day)
    }

}
fn main(){
    let color =["Red","Yellow","Green","Black"];
    let mut phone:Phone;
    let screen = Display::Lcd;

    phone = phone_factory(color[0].to_string(),16,screen,true,22);
    println!("1번 폰: color: {}, size: {}, screen of type: {:?}, security: {}, status: {:?} and used day:{:?}",
    phone.color, phone.size, phone.screen, phone.security, phone.useage.0,phone.useage.1);
}