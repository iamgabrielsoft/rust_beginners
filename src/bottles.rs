






pub fn sing(bottles: u32)   {
    //loop through the bottles
    for i in (0..=bottles).rev() {
        println!("{}", output_bottles(i)); 
    }
}




pub fn output_bottles(bottles: u32) -> String{
        // Once there are 2 bottles left, when you "Take one down, pass it around"
        // you will have "1 bottle" left on the wall. (instead of "1 bottles")
    if bottles == 2 {
        return format!(
            "{} bottles of beer on the wall.\n\
             {} bottles of beer. Take one down, pass it around.\n\
             {} bottle of beer on the wall.",
            bottles,
            bottles,
            bottles - 1
        );

        // Once there is 1 bottle left, when you "Take one down, pass it around"
        // you will have "0 bottles" left on the wall
    }else if bottles == 1 {
        return format!(
            "{} bottle of beer on the wall.\n\
             {} bottle of beer. Take one down, pass it around.\n\
             {} bottles of beer on the wall!",
            bottles,
            bottles,
            bottles - 1
        );
    }
    else if bottles == 0 {
        return String::from("Lets do it again!");
    }
    
    else {
        return format!(
            "{} bottles of beer on the wall.\n\
            {} bottles of beer. Take one down, pass it around.\n\
            {} bottles of beer on the wall.",
            bottles,
            bottles,
            bottles - 1
        )
    }
   
}
