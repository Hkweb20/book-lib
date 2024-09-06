use std::{io, marker::PhantomData, usize};
use reqwest::{Client, Error};

#[derive(Debug)]
struct Dataplan {
    plan_name: String,
    validity: String,
    price: u32,
    plan_id: u32,
    network: Network,
}

#[derive(Debug,PartialEq)]
enum Network {
    Mtn,
    Airtel,
}
struct Resbody{
    plan: u32,
    network: u32,
    mobile_number: u128,
    Ported_number: bool,
}
fn main() {
let mut plans :Vec<Dataplan>=Vec::new();
let _mtn1gb = Dataplan{
    plan_name: "MTN  500MB".to_string(),
    validity: "30 DAYS".to_string(),
    price: 200,
    plan_id: 157,
    network:Network::Mtn
};
let _mtn2gb= Dataplan{
    plan_name: "MTN 2GB".to_string(),
    validity: "30 DAYS".to_string(),
  price: 300,
  plan_id: 12,
  network: Network::Mtn,
};
let mtn3gb = Dataplan{
    plan_name: "MTN 3GB".to_string(),
    validity: "2 DAYS".to_string(),
    price: 500,
    plan_id: 23,
    network: Network::Mtn
};
plans.push(_mtn1gb);
plans.push(_mtn2gb);
plans.push(mtn3gb);

//view_mtn(&plans);

loop {
    println!("#MENU");
    println!("1: BUY MTN DATA");
    println!("2: BUY AIRTEL DATA");
    println!("3 EXIT ");
    let mut choise = String::new();
    io::stdin().read_line(&mut choise).expect("error");
    match choise.trim() {
     "1"=> view_mtn(&plans),
     "2"=> println!("buy airtel soons"),
     "3"=> break,
     _ => println!("hello"),
    }
}



}


fn view_mtn(plans: &Vec<Dataplan>){
  let mtn_plans:Vec<&Dataplan> = plans.iter().filter(|p| p.network == Network::Mtn).collect();
  for (i ,plan )in mtn_plans.iter().enumerate() {
    println!("{}. {} VALIDITY: {} PRICE: ₦{} PLAN ID: {}", i + 1,plan.plan_name, plan.validity, plan.price, plan.plan_id);

      
  }
   println!("Choose A Plan");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("error");

   match input.trim().parse::<usize>(){
       Ok(num) if num > 0 && num <= mtn_plans.len()=>{
        let select = &mtn_plans[num - 1];
        println!("Are you sure want buy {} for ₦{} validitiy {}",select.plan_name,select.price,select.validity);
       let phone_number:u128 =  get_phone_number();
       send_req(select.plan_id,phone_number, Resbody);
    
       }
       _=>{
        println!("invalid select ");
       }
   } 

}
fn get_phone_number() -> String {
    println!("Please enter your phone number:");
    let mut phone_number = String::new();
    std::io::stdin().read_line(&mut phone_number).expect("Failed to read line");
    phone_number.trim().to_string()
}
async  fn send_req(plan_id:u32, Phone:u128, res: &Resbody)-> Result<(),Error>{
    let phone = Phone;
 let rebody = Resbody{
    plan: plan_id,
    network: 1,
    Ported_number: true,
    mobile_number:phone

 };
 let url = "https://www.rigasasub.com.ng/api/data/";
 let apikey = "AICCtnsAD2qexHoA931k70mx1G2ddg3Af6CiBBw6b4ra5vzCAbCyA34CpChA";
 let client = Client::new();
 let res = client.post(url)
 .json(rebody)
 .header("Content-Type", "application/json")
 .header("Authorization", apikey)
 .send()
 .await?;

 Ok(())
}