//在main中引用其他两个模块

mod second;
use second::Coin;
fn main() {
    let object=Coin::new(10000);
    object.test();
}
