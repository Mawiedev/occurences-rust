
/*test :
aaabbc --> a3b2c
aaabaaa --> a3ba3
abcde --> abcde
abcdeee --> abcde3
abbbbbcdeee  --> ab5cde3
a -> a
aaaaaa -> a6 
"" --> ""
*/

fn main() {
    let mut counter = 1;
    let input: Vec<char> = "aaabbc".chars().collect();
    let mut a = vec![];
   
    for i in 0..input.len(){
            if input[i] == input[i+1]
            {
                counter +=1;
                
            }else{
               
                a.push(input[i].to_string());
                a.push(counter.to_string());
                counter=1;
                println!("{:?}", a);
            }
        }   
}
