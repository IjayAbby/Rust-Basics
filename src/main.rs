//print even numbers from 0 to 10

// fn main() { 
//     let mut count = 1;
//     while count <= 10 {
//         if count % 2 == 0 {
//             println!("Even number: {}", count);
//         }
//         count += 1;
//     } 
// }


//create an array with numbers 
// fn main() {
//     let a:[i32, 10] = [1,2,3,4,5,6,7,8,9,10];

//     let mut b: usize = 0;

//     while b != 12 {
//         if a[b] % 2 == 0 {
//             println!("{}", a[b]);
//         }

//         b += 1;
//     }
// }

//create an array containing 5,10,15,20, loop through and if number is less than 10 print less than 10, print larger than 20

fn main() {
    let arr = [5, 10,15,20, 25];


    for count in arr {
        if count < 10 {
            println!("less than 10")
        }
        if count > 20 {
            println!("greater than 20")
        }
    }
}

