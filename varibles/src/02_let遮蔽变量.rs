
    fn main() {
        //遮蔽有作用域限制
        let x = 5;
    
        let x = x + 1;
    
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }
    
        println!("The value of x is: {}", x);
    }
    
