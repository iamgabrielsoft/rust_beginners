

mod bottles; 
mod weightcounter;

use weightcounter::CoinType; 





fn main(){
    const weight: f64 = 45.3; 
    const TYPE: CoinType = CoinType::Dim;
    // rock_paper_scissors::play();
    let (amount, rolls ) = weightcounter::perform(TYPE, weight);
    println!("{} grams = {} with {} coin rolls ", weight, amount, rolls) 

}