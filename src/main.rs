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

fn occurences(string_occurence: &str) -> Vec<String> {

    let mut counter = 1;
    let input: Vec<char> = string_occurence.chars().collect();
    let mut a = vec![];
    
    for i in 0..input.len(){
        
        if i == input.len()-1
        {
            a.push(input[i].to_string());
            if counter != 1
            {
                a.push(counter.to_string());
                
            }
            break;
        }

        if input[i] == input[i+1]
        {
            counter +=1;
            
        }else{
            
            a.push(input[i].to_string());
            if counter != 1
            {
                a.push(counter.to_string());
            }
            counter=1;
        }
        
    }
    return a;
}

//for testing other words, you have to replace the string in the function below
fn result(){
    //replace "aaabbc" by other word you would like to test
    let result: String = occurences("aaabbc").iter().cloned().collect();
    println!("{:?}", result);
}

fn main() {
    result();
}
