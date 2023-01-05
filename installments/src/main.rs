use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let loan_str = args.get(1).expect("Loan value not provided");
    let installment_str = args.get(2).expect("Installment value not provided");
    let interest_str = args.get(3).expect("Interest rate value not provided");

    let (loan, installment, interest) = check_inputs(loan_str, installment_str, interest_str);
    tabulate_installments(loan, installment, interest);
 
}

fn check_inputs(loan_str: &String, installment_str: &String, interest_str: &String) -> (f64, f64, f64) {
    let loan = f64::from_str(loan_str).expect("Loan seems not to be a number");
    let installment = f64::from_str(installment_str).expect("Installment seems not to be a number");
    let interest = f64::from_str(interest_str).expect("Interest seems not to be a number");

    if loan <= 0.0 {
        panic!("Loan value must be positive");
    }
    if installment <= 0.0 {
        panic!("Installment value must be positive");
    }
    if interest < 0.0 {
        panic!("Interest rate value must be positive");
    }
    if loan * interest / 100.0 >= installment {
        panic!("Debt unpayable (loan * interest / 100 >= installment)");
    }
    (loan, installment, interest)
}

fn tabulate_installments(loan: f64, installment: f64, interest: f64) {
    let mut remaining: f64 = loan;
    println!("{}\t{:.2}\t{:.2}", 0, remaining, 0.0);
    let new_interest: f64 = 1.0 + interest / 100.0;
    let mut num_payment: i32 = 1;
    while remaining > 0.0 {
        let mut new_remaining = remaining * new_interest - installment;
        if new_remaining < 0.0 {
            new_remaining = 0.0;
        }
        println!("{}\t{:.2}\t{:.2}", num_payment, new_remaining, remaining - new_remaining); 
        remaining = new_remaining;
        num_payment += 1;
    }
}
