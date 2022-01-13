// fn main() {
//     let x=5;
//     println!("value of x is {}",x);
// }

// fn main(){
//     let x: i32=5;
//     if x==5{
//         println!("x is 5");
//     }
//     else {
//         println!("x is not 5");
//     }
// }
// ------TUPLE------//
// fn main(){
//   let a=(5,"HELLO, world!",false);
//   println!("the first value is {}",a.0);
//   println!("the second value is {}",a.1);
//   println!("the third value is {}",a.2);
// }
//---------ARRAY------//
// fn main(){
//      let a=[1,2,3,4];
//      println!("{:?}",a);

//      let b:[i32:10]=[0;10];
//      println!("array is {:?}",b);

// }

//-------CONTROL FLOW------//
// fn main(){
//     // if else
//     let n=5;
//     if n==5{
//         println!("WON");
//     }
//     else{
//         println!("LOST");
//     }
// }

// ------LOOPS------//

// fn main(){
//     let mut x=2;
//     loop{
//         if x>1000 {
//         break;}
//         x*=2;
//         println!("{}",x);
//     }

//      let mut y=x;
//      while y>0{
//          println!("{}",y);
//          y/=2;
//      } 
//     // 0 to 9
//     for x in 0..10{  
//         println!("{}",x);
//     }
//     // 0 to 10
//     for x in 0..=10{  
//         println!("{}",x);
//     }
//     let y=[1,2,3,4];
//     for val in y{
//         println!("{}",val);
//     }
// }

//------MATCH-------//
fn main(){
    let x=2;
    match x {
        1 => println!("VALUE OF X IS {}",x),
        _ => println!("NO MATCH"),
    }

    let a=true;
    let b=false;
     match (a, b){
         (true,true) => println!("both true"),
         (false,false) => println!("both false"),
         (true,false) =>println!("a true"),
         _ => println!("NO CASE")
     }
}

