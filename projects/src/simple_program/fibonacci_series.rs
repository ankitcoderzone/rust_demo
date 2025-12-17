pub fn nth_fibonacci(num : u32) -> u32{
    if num==0{ //base condition 
        0
    } else if num==1 {
        1
    }else {
         nth_fibonacci(num-1 ) + nth_fibonacci(num-2) //recursive call
    }
  
   
    
    
    
}