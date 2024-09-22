use rand::{Rng,SeedableRng};

pub trait Bag<T> {
    fn push(&mut self,item:T); //Add an item to the bag.
    fn pop(&mut self)->Option<T>; //Remove a random(determined by RNG) item from the bag
    fn new_empty()->Self; //New empty bag
    fn multipush(&mut self, iter:impl IntoIterator<Item=T>){
        for item in iter{self.push(item);}
    }
}


/*Perhaps todo
#[macro_export]
macro_rules! bag_f_iter {
    ($new_bag:ty) => {
        impl<T> FromIterator<T> for new_bag<T>{
            let nb = Self::new_empty();
            nb.multipush(iter);
            return nb;
        }
    };
}
*/

pub struct ArrayBag<T,R:SeedableRng>{
    items : Vec<T>,
    rng : R,
}

impl<T,R:SeedableRng+Rng> Bag<T> for ArrayBag<T,R>{
    fn push(&mut self,item:T) {
        self.items.push(item)
    }

    fn pop(&mut self)->Option<T> {
        if self.items.is_empty() { None }
        else { Some(self.items.swap_remove(self.rng.gen_range(0..self.items.len()))) }
    }

    fn new_empty()->Self {
        ArrayBag { items: vec![], rng: R::from_entropy() }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;

    use super::*;

    #[test]
    fn empty_pop(){
        let mut new_empty_bag:ArrayBag<u8,StdRng>=ArrayBag::new_empty();
        assert_eq!(new_empty_bag.pop(), None);
    }
    
}
