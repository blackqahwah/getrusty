use std::io;

// fn main() {
//     let mut count = 0;
  
//     'counting_up: loop {
//         println!("count = {count}");
      
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }


fn main() {

  println!("what do you want to convert?");
  println!("type 1 for fahrenheit to celsius. type 2 for celsius to fahrenheits");

  let mut response = String::new();

   io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

  
    let response: u32 = response.trim()
                   .parse()
                   .expect("please type a number");


  if response == 1{

    println!("type in your fahrenheit value, preferably a value below 32 please.");

    let mut farh = String::new();
    
    io::stdin()
        .read_line(&mut farh)
        .expect("Failed to read line");

    let farh: u32 = farh.trim()
                   .parse()
                   .expect("please type a number");

  let celsh = (farh-32) * 5;

      println!(" {farh} fahreneits is {} celsius", celsh / 9 );
  } else {

     println!("type in your celsius value");

     let mut celsh = String::new();
    
    io::stdin()
        .read_line(&mut celsh)
        .expect("Failed to read line");

    let celsh: u32 = celsh.trim()
                   .parse()
                   .expect("please type a number");

  let xer: f64 = 1.8;
  let farh = (celsh as f64) * xer;
 

        println!("{celsh} celsius is {} fahrenheits", (farh as u32) + 32);
  }
  
  
}

