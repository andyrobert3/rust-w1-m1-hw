// Define a User struct which contains 2 fields: 
    // name (string e.g "John")
    // balance (tuple e.g (100.00, "SGD"))

// Define a User method (using impl) 
    // called print_user_detail, 
        // which simply prints the name, balance and currency of the user.

// Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters. 
    // Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method. 

struct User {
    name: String,
    balance: (f32, String),
}

impl User {
    fn print_user_detail(&self) {
        println!("Name: {}, balance: {}, currency: {}", self.name, self.balance.0, self.balance.1);
    }
}

fn accrue_interest(user: &mut User, interest_percentage: f32) {
    let interest = user.balance.0 * interest_percentage / 100.00;

    // User cannot have negative balance 
    if interest < 0.0 && (interest.abs() > user.balance.0.abs()) {
        user.balance.0 = 0.0;
    } else {
        user.balance.0 += interest;
    }

    user.print_user_detail();
}

fn main() {
    // In the main function, create a user variable of type User, populating the field values of name, and balance and currency.
    let mut user = User {
        name: "Grizzly".to_owned(),
        balance: (100.00, "SOL".to_owned()),
    };

    let interest_rate: f32 = 3.0;

    // Then, call the accrue_interest function.
    accrue_interest(&mut user, interest_rate);
    
    // Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest.
    let num_years = 5;
    for _ in 1..=num_years {
        accrue_interest(&mut user, interest_rate);
    }
}
