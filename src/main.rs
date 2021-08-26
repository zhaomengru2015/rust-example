use std::collections::HashMap;

struct Cacher<T>
where
  T: Fn(u32)->u32,
{
  calculation: T,
  value: Option<u32>,
}
impl<T> Cacher<T>
where
  T: Fn(u32)->u32,
{
  fn new(calculation: T) -> Cacher<T>{
    Cacher{
      calculation,
      value: None,
    }
  }
  fn value(&mut self, args: u32) ->HashMap<u32,u32>{
    match self.value {
      Some(v) => v,
      None =>{
        let v = (self.calculation)(args);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main() {
    // generate_workout(simulated_user_specified_value, simulated_random_number);
  let mut x= Cacher::new(|x| x);
  let v1 = x.value(1);
  let v2 = x.value(2);
  println!("v1:{},v2:{}",v1,v2);
  // assert_eq!(v1,v2);
}
