use std::io;


fn main() {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 0;
    let _nterms = loop{
        println!("Please enter how many terms u want in fibonacci sequence");
        let mut nterms = String::new();

        io::stdin()
        .read_line(&mut nterms)
        .expect("Failed to read line");

        let nterms: u32 = match nterms.trim().parse(){
            Ok(num) => {
                if num <= 0 {
                    println!("Enter a number which is not 0!");
                    continue
                }
                else if num >= 46 {
                    println!("Enter a number which is not higher then 45!!! due to overflow error");
                    continue
                }
                else
                {
                    num
                }
            },
            Err(_) => {
                println!("Enter a positive integer number please");
                continue
            }
        };

        break nterms;
    };

    if _nterms == 1{
        println!("Fibonacci sequence upto {},: {}",_nterms,n1);
    }
    else{
        println!("Your fibonacci sequence: ");
        while count < _nterms{
            println!("{}",n1);
            let nth = n1 + n2;
            n1 = n2;
            n2 = nth;
            count += 1;
        }
    }
}
