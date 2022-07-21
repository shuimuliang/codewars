// https://www.codewars.com/kata/529a92d9aba78c356b000353/train/rust
// https://gist.github.com/QuadFlask/86f9b957f7e269a238f8b26411e0a9af

#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T>
    {
        // provide a convenient method to convert any iterable to an algebraic list.
        // let a1 = Cons::new(1, a2);
        // let a2 = Cons::new(2, a3);
        // let a3 = Cons::Null;
        // a1
        let mut iter = it.into_iter();
        if let Some(t) = iter.next() {
            Cons::new(t, Cons::from_iter(iter))
        } else {
            Cons::Null
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool
    {
        // TODO: return a new algebraic list containing only elements that satisfy the predicate function.
        // let a1 = filter( Cons::new(1, a2) );
        // let a2 = filter( Cons::new(2, a3) );
        // let a3 = filter( Cons::new(2, a4) );
        // let a4 = Cons::Null;

        if let Cons::Cons(head, tail) = self {
            if fun(head) {
                return Cons::new(head.clone(), tail.filter(fun));
            } else{
                return tail.filter(fun);
            }
        }

        Cons::Null
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S,
              S: Clone
    {
        //TODO: return a new algebraic list containing all elements resulting from applying the mapper function to them.
        if let Cons::Cons(head, tail) = self {
                return Cons::new(fun(head.clone()), tail.map(fun));
        }

        Cons::Null

        // match self {
        //     &Cons::Null => Cons::Null,
        //     &Cons::Cons(ref head, ref tail) =>
        //         Cons::new(fun(head.clone()), tail.map(fun))
        // }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_from_vec() {
        assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

        assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
                   vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_filter() {
        assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                       .filter(|&n| n > 3)
                       .to_vec(),
                   vec![4, 5]);

        assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                       .filter(|&n| n > 5),
                   Cons::Null);
    }

    #[test]
    fn should_map() {
        assert_eq!(Cons::from_iter(vec!["1", "2", "3", "4", "5"])
                       .map(str::parse::<i32>)
                       .map(Result::unwrap)
                       .to_vec(),
                   vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_filter_map() {
        assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                       .filter(|n| n % 2 == 0)
                       .map(|x| x.to_string())
                       .to_vec(),
                   ["2", "4"]);
    }
}