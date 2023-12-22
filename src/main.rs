use std::collections::HashMap;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
mod restaurant;
use crate::restaurant::order_food;
fn main() {
    println!("Hello, world!");


    //HASHMAP
    let mut heroes = HashMap::new();
    heroes.insert("superman", "clark kent");
    heroes.insert("batman", "bruce wayne");
    heroes.insert("flash", "barry allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}",k,v);
    }

    println!("Length {}", heroes.len());

    if heroes.contains_key(&"batman"){
        let the_batman = heroes.get(&"batman");
        match the_batman {
            Some(_) => println!("batman is a superhero "),
            None => println!("batman is no hero")
        }
    }


    //Struct
    struct Customer {
        name: String,
        address: String,
        balance: f32
    }

    let mut bob = Customer {
        name:String::from("bob"),
        address:String::from("elm street, ebby"),
        balance: 10.25
    };

    bob.address = String::from("new changed addereds");

    println!("{} from {} has to give me {} today",bob.name, bob.address, bob.balance);

     //with Generic datatypes
      struct Rectangle<W,H> {
        width : W,
        height: H
      } 

      let rect = Rectangle{width:10, height:10.25};
      println!("rectangle with is {} and height is {}", rect.width, rect.height);


      //traits

      trait Shape {
          fn new(height: f32, width: f32) -> Self;
          fn area(&self) -> f32;
      }

      impl Shape for Rectangle<f32, f32> {
          fn new(height: f32, width: f32) -> Self {
              Rectangle{height,width}
          }
          fn area(&self) -> f32 {
              &self.height * &self.width
          }
      }

      let recta: Rectangle<f32,f32> = Shape::new(12.35, 52.6);
      println!("the area of the recatngel is {}", recta.area());

      //PACKAGES , MODEULS

      order_food();


      //File I/O

      let path = "lines.txt";
      let output = File::create(path);
      let mut op = match output {
          Ok(file) => file,
          Err(error) => {
            panic!("Problem cresting file : {:?}", error);
          }
      };
      write!(op,"Just some \nrandom workds").expect("failed to write to file");

      let input = File::open(path).unwrap();

      let bufread = BufReader::new(input);
      for line in bufread.lines(){
        println!("{}", line.unwrap());
      }



      
}
