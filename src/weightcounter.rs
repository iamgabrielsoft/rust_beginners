
//determin the cointype after performing the weight count
pub enum CoinType {
    Penny, 
    Nickel, 
    Dim, 
    Quarter
}


//perform the operation here
pub fn perform(coin_type: CoinType, weight: f64) -> (i32, i32) {
    match coin_type {
        CoinType::Penny => {
            //perform some simple calculation 
            let amount = (weight  /2.50).ceil(); 
            let rolls = (amount /50.0).ceil() as i32; 
            (amount as i32, rolls)
        }

        CoinType::Nickel => {
            let amount = (weight / 5.0).ceil(); 
            let rolls = (amount / 40.0).ceil() as i32; 
            (amount as i32, rolls)
        }

        CoinType::Dim => {
            let amount = (weight / 2.268).ceil();
            let rolls = (amount / 50.0).ceil() as i32;
            (amount as i32, rolls)
        }

        CoinType::Quarter => {
            let amount = (weight / 5.67).ceil();
            let rolls = (amount / 40.0).ceil() as i32;
            (amount as i32, rolls)
        }
    }
}