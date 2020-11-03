use std::env;
use std::cmp::{max,min};

fn main() {
    let (n,r)={
        let args: Vec<String> = env::args().collect();
        let b=args[1].parse::<usize>().unwrap();
        let c=args[2].parse::<usize>().unwrap();
        (max(b,c),min(b,c))
    };
    let res=combination(n,r);
    println!("{}",res);
}

///n always larger than r
fn combination(n:usize,mut r:usize)->usize{
    let mut x=n-r+1;
    let mut res:usize=1;

    while x<=n{
        res*=x;
        while r>1 && res%r==0 {
            res/=r;
            r-=1;
        }
        x+=1;
    }

    while r>1 {
        res/=r;
        r-=1;
    }

    res
}

#[test]
fn test_combination() {
    assert_eq!(5, combination(5,1) );
    assert_eq!(10, combination(5,2) );
}
