pub fn precedencia(operator: char)->i8{
    if operator == '+' || operator == '-'{
        1
    }else{
        2
    }
}

pub fn tokens_array(string:&str) -> Vec<String>{
    let mut caracteres: Vec<String> = Vec::new();
    let brackets = vec!['(',')'];
    let opbracket:char = '(';
    let clbracket:char = ')';
    let ops = vec!['+','-','*','/'];
    println!("inicio {}",string);
    let string_vec: Vec<char> = string.chars().collect();
    println!("inicio {:?}",string_vec);
    let mut tam_num = 0;
    let mut index = 0;

    for i in &string_vec{
        
        if ops.contains(&i){
            
            if index != 0{
                if (!ops.contains(&string_vec[index-1])) && (!brackets.contains(&string_vec[index-1])){
                    if (ops.contains(&string_vec[tam_num]) && string_vec[tam_num] != '-') || (ops.contains(&string_vec[tam_num]) && string_vec[tam_num+1] == '-'){
                        caracteres.push(string_vec[tam_num].to_string());
                        tam_num+=1;
                    }
                    
                    let aux:Vec<char> = string_vec[tam_num..index].to_vec();
                    caracteres.push(aux.iter().collect());
                    caracteres.push(string_vec[index].to_string());
                    tam_num=index+1;
                }
                else if brackets.contains(&string_vec[index-1]) && brackets.contains(&string_vec[index+1]){
                    caracteres.push(i.to_string());
                }
                else if string_vec[index-1] == ')' && (!ops.contains(&string_vec[index+1])) && string_vec[index-1] == '-'{
                    caracteres.push(i.to_string());
                    tam_num=index+1;
                }
            }
        }
        else if i == &opbracket{
            caracteres.push(i.to_string());
            tam_num=index+1;
        }
        else if i == &clbracket{
            
            if (ops.contains(&string_vec[tam_num]) && string_vec[tam_num] != '-') || (ops.contains(&string_vec[tam_num]) && string_vec[tam_num+1] == '-'){
                caracteres.push(string_vec[tam_num].to_string());
                tam_num +=1 ;
            }
            if string_vec[tam_num] != clbracket{
                let aux:Vec<char> = string_vec[tam_num..index].to_vec();
                
                caracteres.push(aux.iter().collect());
            }
            caracteres.push(string_vec[index].to_string());
            tam_num = index+1 ;

        }
        else if index == string_vec.len()-1 {
            if (ops.contains(&string_vec[tam_num]) && string_vec[tam_num] != '-') || (ops.contains(&string_vec[tam_num]) && string_vec[tam_num+1] == '-'){
                caracteres.push(string_vec[tam_num].to_string());
                tam_num +=1 ;
            }
            let aux:Vec<char> = string_vec[tam_num..string_vec.len()].to_vec();
            
            caracteres.push(aux.iter().collect());
        }
        index+=1;
    }
    

    caracteres
}













#[cfg(test)]
mod tests{
    use super::precedencia;
    #[test]
    fn precedencia_test(){
        assert_eq!(1, precedencia('+'));
        assert_eq!(1, precedencia('-'));
        assert_eq!(2, precedencia('*'));
        assert_eq!(2, precedencia('/'));
    }

    use super::tokens_array;
    #[test]
    fn tokens_array_test(){
        let vartest:Vec<&str> = ["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"].to_vec() ;
        assert_eq!(vartest,tokens_array(&"(10 / 3 + 23) * (1 - 4)".replace(' ',&"".to_string())));
        
        let vartest:Vec<&str> = ["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"].to_vec() ;
        assert_eq!(vartest,tokens_array(&"-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"].to_vec() ;
        assert_eq!(vartest,tokens_array(&"65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"].to_vec() ;
        assert_eq!(vartest,tokens_array(&"-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"].to_vec() ;
        assert_eq!(vartest,tokens_array(&"-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42".replace(' ',&"".to_string())));

        
        
        
    }
}