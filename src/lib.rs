use rand::Rng;

pub trait Bag<T> {
    fn push(&mut self,item:T); //Add an item to the bag.
    fn pop(&mut self, rng:impl Rng)->Option<T>; //Remove a random(determined by RNG) item from the bag
    fn new_empty()->Self; //New empty bag
    fn multipush(&mut self, iter:impl IntoIterator<Item=T>){
        for item in iter{self.push(item);}
    }
}


/* #[macro_export]
macro_rules! impl_fromiter {
    ($t:ty, $new_bag:ty) => {
        impl<$t>  FromIterator<$t> for $new_bag{
            fn from_iter(iter:Iterator<$t>){
                let nb = Self::new_empty();
                nb.multipush(iter);
                return nb;
            }
        }
    };
} */
/*
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

impl <T>FromIterator<T>for ArrayBag<T,R> where ArrayBag<T,R> : Bag<T>{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut nb = Self::new_empty();
        nb.multipush(iter);
        return nb;
    }
}
*/
impl<T> Bag<T> for Vec<T>{
    fn push(&mut self,item:T) {self.push(item)}

    fn pop(&mut self, mut rng:impl Rng)->Option<T> {
        if self.is_empty() { None }
        else { Some(self.swap_remove(rng.gen_range(0..self.len()))) }
    }

    fn new_empty()->Self {vec![]}
}


