fn main() {
    println!("-------*Class Difference Shower*--------");

    let mut numCustomer = 5;  

    'class_diff: loop {
       let mut bankBalance = 10000; 
        println!("Customer ID {numCustomer}");

        'bal: loop {
            println!("Bank Balance: {bankBalance}");

            if bankBalance < 3000 {
                break 'bal;
            }

            if numCustomer < 0 {
                break 'class_diff;
            }

            bankBalance -= 1508;
        }
        numCustomer -= 1;
    }
}
