/*
    TODO: add named command line parameters (clap)
    TODO: add functionality by allowing to calculate iteratively one of loan, interest rate, installment or number of installments.


*/
use std::fmt::Write;
use std::str::FromStr;

type CurrencyType = f64;
type InterestType = f64;
type CountType = u32;

const REMAINING_INSTALLMENT_THRESHOLD : CurrencyType = 0.005;

enum UnknownLoanInfo {
    Loan,
    Installment,
    Interest,
    NumberOfPayments,
}

impl UnknownLoanInfo {
    fn get_operation_from_number(operation_number: u32) -> Result<UnknownLoanInfo, core::fmt::Error> {
        match operation_number {
            1 => Ok(UnknownLoanInfo::Installment),
            2 => Ok(UnknownLoanInfo::Interest),
            3 => Ok(UnknownLoanInfo::Loan),
            4 => Ok(UnknownLoanInfo::NumberOfPayments),
            _ => {
                let mut s = String::new();
                Err(write!(&mut s, "Cannot convert {} to UnknownLoanInfo. Valid values are in the integer range from 1 to 4.", operation_number))
            },
        }
    }
}
struct LoanInfo {
    unknown_loan_info: UnknownLoanInfo,
    loan: CurrencyType,
    installment: CurrencyType,
    interest: InterestType,
    number_of_payments: CountType,
}

impl Default for LoanInfo {
    fn default() -> Self {
        Self {
            unknown_loan_info: UnknownLoanInfo::NumberOfPayments,
            loan: CurrencyType::default(),
            installment: CurrencyType::default(),
            interest: CurrencyType::default(),
            number_of_payments: CurrencyType::default(),
        }
    }
}

fn help() {
    println!("Calculates loan values.");

    println!("Requires three numeric arguments.");
    println!("Accepts a fourth optional positive integer argument with values ranging from 1 (default) to 4.");

    println!("");
    println!("When fourth argument is 1 (or missing):");
    println!("\t1) This program finds the NUMBER OF PAYMENTS (times)");
    println!("\t   of INSTALLMENT  VALUE needed to pay the LOAN VALUE with INTEREST RATE.");
    println!("\t2) First argument is LOAN VALUE.");
    println!("\t3) Second argument is INSTALLMENT VALUE, and");
    println!("\t4) Third argument is INTEREST RATE.");

    println!("");
    println!("When fourth argument is 2:");
    println!("\t1) This program finds the INTEREST RATE");
    println!("\t   to pay the LOAN VALUE in NUMBER OF PAYMENTS (times) of INSTALLMENT VALUE.");
    println!("\t2) First argument is LOAN VALUE,");
    println!("\t3) Second argument is INSTALLMENT VALUE, and");
    println!("\t4) Third argument is NUMBER OF PAYMENTS.");

    println!("");
    println!("When fourth argument is 3:");
    println!("\t1) This program finds the LOAN VALUE");
    println!("\t   such that it is paid by NUMBER OF PAYMENTS (times) of INSTALLMENT VALUE with INTEREST RATE.");
    println!("\t2) First argument is INSTALLMENT VALUE,");
    println!("\t3) Second argument is INTEREST RATE, and");
    println!("\t4) Third argument is NUMBER OF PAYMENTS.");

    println!("");
    println!("When fourth argument is 4:");
    println!("\t1) This program finds the INSTALLMENT VALUE");
    println!("\t   such that the LOAN VALUE is paid In NUMBER OF PAYMENTS (times) and with INTEREST RATE.");
    println!("\t2) First argument is LOAN VALUE,");
    println!("\t3) Second argument is INTEREST RATE, and");
    println!("\t4) Third argument is NUMBER OF PAYMENTS.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        help();
        panic!("At least loan value, installment value and interest rate must be provided");
    }
    if args.len() > 4 {
        help();
        panic!("At most four values must be provided: ")
    }
    let first_arg_str = args.get(1).unwrap();
    let second_arg_str = args.get(2).unwrap();
    let third_arg_str = args.get(3).unwrap();
    let operation_type_str = args.get(4).unwrap();

    let loan_info = process_args(first_arg_str, second_arg_str, third_arg_str, operation_type_str).unwrap();
    match loan_info.unknown_loan_info {
        UnknownLoanInfo::Installment => find_installment(loan_info),
        UnknownLoanInfo::Interest => find_interest(loan_info),
        UnknownLoanInfo::Loan => find_loan(loan_info),
        UnknownLoanInfo::NUmberOfPayments => find_number_of_payments(loan_info),
    };
}

fn find_installment(loan_info: &LoanInfo) -> Result<CurrencyType, core::fmt::Error> {
    // First guess is the loan value divided by the expected number of payments.
    let mut installment: CurrencyType = loan_info.loan / (loan_info.number_of_payments as CurrencyType);
    // Check how far the guess is from reality.
    loop {
        let mut remaining_value = loan_info.loan;
        let mut n_paym_count_down = loan_info.number_of_payments;
        while n_paym_count_down > 0 {
            remaining_value *= (1.0 + loan_info.interest);
            remaining_value -= installment;
            n_paym_count_down -= 1;
        }
        // We are working with currency values, then less than an residual less tha half of a hundreth of the unity is the stop criteria
        if remaining_value < REMAINING_INSTALLMENT_THRESHOLD {
            installment
        }
        // Updates the guess for the next iteration.
        installment += remaining_value / (loan_info.number_of_payments as CurrencyType);
    }
}

fn find_interest(loan_info: &LoanInfo) -> Result<Option<CurrencyType>, &String> {
    Ok(Some(CurrencyType::default()))
}

fn find_loan(loan_info: &LoanInfo) -> Result<Option<CurrencyType>, &String> {
    Ok(Some(CurrencyType::default()))
}

fn find_number_of_payments(loan_info: &LoanInfo) -> Result<Option<CountType>, &String> {
    /*
        Formula:
            v(n+1) = v(n) * (1 + j) - p
            v = remaining debt (value)
            n = payment number
            j = interest rate
            p = installment
    */
    if loan_info.loan <= 0.0 {
        Err("Loan value must be greater than zero");
    }
    if loan_info.installment <= 0.0 {
        Err("Installment value must be greater than zero.");
    }
    if loan_info.interest < 0.0 {
        Err("Interest rate value must be positive");
    }
    if loan_info.loan * loan_info.interest / 100.0 >= loan_info.installment {
        Err("Debt unpayable (loan * interest / 100 >= installment)");
    }

    let mut remaining = loan_info.loan;
    println!("{}\t{:.2}\t{:.2}", 0, remaining, 0.0);
    let new_interest = 1.0 + loan_info.interest / 100.0;
    let mut num_payment: CountType = 1;
    while remaining > 0.0 {
        let mut new_remaining = remaining * new_interest - loan_info.installment;
        if new_remaining < 0.0 {
            new_remaining = 0.0;
        }
        println!("{}\t{:.2}\t{:.2}", num_payment, new_remaining, remaining - new_remaining); 
        remaining = new_remaining;
        num_payment += 1;
    }
    Ok(Some(num_payment))
}


fn process_args(first_arg_str: &String, second_arg_str: &String, third_arg_str: &String, operation_type_str: &String) -> Result<LoanInfo, core::fmt::Error> {
    let operation_number = u32::from_str(&operation_type_str).expect("Operation seems not to be an integer number.");

    match operation_number {
        1 => {
            Ok(LoanInfo {
                unknown_loan_info: UnknownLoanInfo::NumberOfPayments,
                loan: CurrencyType::from_str(first_arg_str).expect("Loan seems not to be a number."),
                installment:  CurrencyType::from_str(second_arg_str).expect("Installment seems not to be a number."),
                interest: CurrencyType::from_str(third_arg_str).expect("Interest seems not to be a number."),
                number_of_payments: CountType::default(),
            })
        },
        2 => {
            Ok(LoanInfo {
                unknown_loan_info: UnknownLoanInfo::Interest,
                loan: CurrencyType::from_str(first_arg_str).expect("Loan seems not to be a number."),
                installment:  CurrencyType::from_str(second_arg_str).expect("Installment seems not to be a number."),
                number_of_payments: CountType::from_str(third_arg_str).expect("Number of payments seems not to be a number."),
                interest: CurrencyType::default(),
            })
        },
        3 => {
            Ok(LoanInfo {
                unknown_loan_info: UnknownLoanInfo::Loan,
                installment: CurrencyType::from_str(first_arg_str).expect("Loan seems not to be a number."),
                interest:  CurrencyType::from_str(second_arg_str).expect("Installment seems not to be a number."),
                number_of_payments: CountType::from_str(third_arg_str).expect("Number of payments seems not to be a number."),
                loan: CurrencyType::default(),
            })
        },
    4 => {
            Ok(LoanInfo {
                unknown_loan_info: UnknownLoanInfo::Installment,
                loan: CurrencyType::from_str(first_arg_str).expect("Loan seems not to be a number."),
                interest:  CurrencyType::from_str(second_arg_str).expect("Installment seems not to be a number."),
                number_of_payments: CountType::from_str(third_arg_str).expect("Number of payments seems not to be a number."),
                installment: CurrencyType::default(),
            })
        },
        _ => Err("Operation number shall be an integer number between 1 and 4.")
    }
}

fn tabulate_installments(loan: f64, installment: f64, interest: f64) {
/*
    Formula:
        v(n+1) = v(n) * (1 + j) - p
        v = remaining debt (value)
        n = payment number
        j = interest rate
        p = installment
*/
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
