#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn add() -> &'static str {
    "hi, Please enter the any number in url as like (/10) to add-it "
}

#[get("/<num>")]
fn num(num:u32) -> String{
    format!("Number is:  {} after addition of:  {}",num,num+10)
}


fn main() {
    rocket::ignite().mount("/", routes![add,num]).launch();
}
