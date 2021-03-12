// pub struct ClassName{
//   pub field:Type,
// }

// pub impl ClassName{
//     fn test(&self) ->  {
//      println!("test")
//     }
// }
//  pub enum EnumTest {
//    A,
//    B
//  }

// pub impl EnumTest{
//   fn enumtest(&self){
//     println!(enumtest)
//   }
// }
#[warn(dead_code)]
pub struct Coin{
  cointype:i32,
}


impl Coin {
    pub fn new(value: i32) -> Coin {
        Coin {
            cointype:value
        }
    }

    pub fn test(&self) {
        println!("from public method");
        self.testcoin();
    }

    fn testcoin(&self) {
        println!("from private method:testcoin");
    }

}


