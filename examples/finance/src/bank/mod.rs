pub struct Bank {
    pub name: String
}

pub struct SwiftCode {
    pub bank_code: [char; 4], // BRAS for Banco do Brasil
    pub country_code: [char; 2], // BR for Brazil
    pub location_code: [char; 2], // RJ for Rio de Janeiro
    pub branch_code: [char; 3] // IBR for branch code
}

pub struct CreditCard {
    pub number: u32
}
