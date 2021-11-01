#[derive(Clone, Default, Debug)]
pub struct Node{
    pub data:String,
    pub left:Option<Box<Node>>,
    pub right:Option<Box<Node>>,
}
//struct do tipo Node com Option(receber valores como None) e Box(um tipo de ponteiro)

impl Node {
    pub fn new(data: String) -> Node {
        Node {data:data,left:None,right:None}
    }
    
}
//contrutor do Node

pub type Tree = Node ;
//definindo uma arvore

//Verifica a precedencia do operador, 1 para + e - e 2 para * e /
pub fn precedencia(operator: &str)->i64{
    if operator == "+" || operator == "-"{
        1
    }else{
        2
    }
}
//Verifica se a string passada e um operador
pub fn consulta_operador(op:&str)->bool{
    if op == "+" || op == "-" || op == "/" || op == "*"{
        true
    }else{
        false
    }
}
//Verifica se a string passda e um digito valido(diferente de operador e parenteses)
pub fn consulta_digito(op:String)->bool{
    if op == "+" || op == "-" || op == "/" || op == "*" || op == "(" || op == ")"{
        false
    }else{
        true
    }
}
//Recebe uma string e devolve um array de tokens, seprando a expressao de forma correta em um vetor  O(n)
pub fn lexer(string:String) -> Vec<String>{
    let mut caracteres: Vec<String> = Vec::new();
    let brackets = vec!['(',')'];
    let opbracket:char = '(';
    let clbracket:char = ')';
    let ops = vec!['+','-','*','/'];
    
    let string_vec: Vec<char> = string.chars().collect();
    
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
    
    //println!("{:?}",caracteres);
    caracteres
}
//Recebe a expressao e a trasforma em notacao polonesa reversa (shunting yard) O(n)
pub fn expression_tree(lista:Vec<String>)->Vec<String>{
    
    let mut lista_op:Vec<String> = Vec::new();
    let mut lista_num:Vec<String> = Vec::new();

    for i in lista{
        if i != "+" && i != "-" && i != "/" && i != "*" && i != "(" && i !=")"{
            lista_num.push(i.to_string());
            
        }
        else if i == "+" || i == "-" || i == "/" || i == "*"{
            while lista_op.len() > 0 && precedencia(&lista_op[lista_op.len()-1].to_string()) >= precedencia(&i) && lista_op[lista_op.len()-1] != "("{
                lista_num.push(lista_op.pop().unwrap());
            }
            lista_op.push(i.to_string());
        }
        if i == "("{
            lista_op.push(i.to_string());
        }
        if i == ")"{
            while lista_op.len() > 0 && lista_op[lista_op.len()-1] != "("{
                lista_num.push(lista_op.pop().unwrap());
            }
            lista_op.pop();
        }
    }
    
    while lista_op.len()>0{
        lista_num.push(lista_op.pop().unwrap());
    }
    //println!("{:?}",lista_num);
    lista_num
}
//Recebe a notacao polonesa e cria uma arvore com obedecendo a notacao O(n)
pub fn parser(vet:Vec<String>)->Tree{
    let mut stack_str:Vec<String> = Vec::new() ;
    let mut stack_node: Vec<Node> = Vec::new();
    
    for tk in &vet{

        if consulta_digito(tk.to_string()){
            stack_str.push(tk.to_string());
            
        }
        else if consulta_operador(&tk.to_string()){
            if stack_str.len()>0 {
                let mut node = Node::new(tk.to_string());
                let f1:String = stack_str.pop().unwrap();
                
                if f1 == "Node"{
                    node.right = Some(Box::new(stack_node.pop().unwrap()));
                }else{
                    node.right = Some(Box::new(Node::new(f1.to_string())));
                }
                let f1:String = stack_str.pop().unwrap();
            
                if f1 == "Node"{
                    node.left = Some(Box::new(stack_node.pop().unwrap()));
                }else{
                    node.left = Some(Box::new(Node::new(f1.to_string())));
                }
                stack_node.push(node);
                stack_str.push("Node".to_string());
            }
            
        }
        
    }
   let tree:Tree = stack_node.pop().unwrap();
   tree
}
//Recebe um node, contendo um operando na raiz e 2 valores como filhos, e o resolve
pub fn resolve_node(node:Node)->i64{
    let a:Node = *node.clone().left.unwrap();
    let b:Node = *node.clone().right.unwrap();
    let n1:i64 = a.data.parse::<i64>().unwrap() ;
    let n2:i64 = b.data.parse::<i64>().unwrap() ;
    //println!("{} {}",n1,n2);
    let op = node.data;
  
    if op == "+"{
        return n1+n2
    }
    else if op == "-"{
        return n1-n2
    } 
    else if op== "*"{
        return n1*n2
    }
    else{
        if n2==0{
            return i64::MAX 
        }
        else {
            return n1/n2
        }
    }
        
  
}
//Caminha pela arvore de forma pos_ordem, procurando o operador da vez para ser calculado  O(qtd node)
pub fn pos_ordem_step(node : &mut Node) -> bool {
    match (&mut node.left, &mut node.right) {
        (Some(left_node), Some(right_node)) => {
            if consulta_operador(&left_node.data) {
                
                return pos_ordem_step(left_node);
            } else if consulta_operador(&right_node.data) {
                
                return pos_ordem_step(right_node);
            } else {
           
                let a:Node = *node.clone().left.unwrap();
                let b:Node = *node.clone().left.unwrap();
                if consulta_digito(a.data) && consulta_digito(b.data){
                    let val = resolve_node(node.clone());
                    if val == i64::MAX {
                        println!("Foi achado uma divisao por 0, programa abortado");
                        return false
                    }
                    node.data = val.to_string();
                    node.left = None;
                    node.right = None ;
                    
                    return true;
                }
            }
        },
        _ => (),
    }
    return true;
}
//Procura o index de um operando um um vetor de uma expressao  O(n)
pub fn find_idx(x:Vec<String>, op:String)->usize{
    let mut a:usize = 0;
    for i in x{
        if i == op{
            return a
        }
        a+=1
    }
    a = usize::MAX;
    return a;
}
//Recebe um node e o passa para string para ser retornado O(h arvore)
pub fn to_string(root:Node, next_op:String)->String{
    let mut exp:String = root.clone().data;
    let ops = vec!['+','-','*','/'];
    let mut atual_op:String = "".to_string();
    if consulta_operador(&root.clone().data){
        atual_op = root.data;
    }
    
    if !root.left.is_none(){
        let x:String = to_string( *root.left.unwrap(), atual_op.clone());
        exp = x + &" ".to_string() + &exp;
    }
    if !root.right.is_none(){
        let x:String = to_string( *root.right.unwrap(), atual_op.clone());
        exp = exp + &" ".to_string() + &x;
    }
    
    let x:Vec<String> = lexer(exp.clone());
    for op in ops{
        let idx = find_idx(x.clone(), op.to_string());
        if idx != usize::MAX && !consulta_digito(x[idx].clone()){
            if precedencia(&atual_op) < precedencia(&next_op){
                exp = "(".to_owned() + &exp +")";
            }
            break 
        }
    }


    return exp
}



#[cfg(test)]
mod tests{
    use crate::Node;
use super::precedencia;
    #[test]
    fn precedencia_test(){
        assert_eq!(1, precedencia("+"));
        assert_eq!(1, precedencia("-"));
        assert_eq!(2, precedencia("*"));
        assert_eq!(2, precedencia("/"));
    }

    use super::lexer;
    #[test]
    fn lexer_test(){
        let vartest:Vec<&str> = ["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"].to_vec() ;
        assert_eq!(vartest,lexer("(10 / 3 + 23) * (1 - 4)".replace(' ',&"".to_string())));
        
        let vartest:Vec<&str> = ["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"].to_vec() ;
        assert_eq!(vartest,lexer("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"].to_vec() ;
        assert_eq!(vartest,lexer("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"].to_vec() ;
        assert_eq!(vartest,lexer("-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51".replace(' ',&"".to_string())));

        let vartest:Vec<&str> = ["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"].to_vec() ;
        assert_eq!(vartest,lexer("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42".replace(' ',&"".to_string())));

    }

    use super::expression_tree;
    #[test]
    fn expression_tree_test(){
        assert_eq!(["1","2","3","*","+"].to_vec(),expression_tree(["1".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "3".to_string()].to_vec()));
        assert_eq!(["1", "2", "+", "3", "+", "4", "*"].to_vec(),expression_tree(["(".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), "+".to_string(), "3".to_string(), ")".to_string(), "*".to_string(), "4".to_string()].to_vec()));
        assert_eq!(["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"].to_vec(),expression_tree(["10".to_string(), "*".to_string(), "20".to_string(), "+".to_string(), "3".to_string(), "*".to_string(), "7".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "3".to_string(), "+".to_string(), "10".to_string(), "/".to_string(), "3".to_string(), "*".to_string(), "4".to_string()].to_vec()));
        assert_eq!(["80", "-18", "*", "85", "-46", "-71", "+", "-12", "*", "26", "+", "59", "-", "*", "84", "+"].to_vec(),expression_tree(["80".to_string(), "*".to_string(), "-18".to_string(), "*".to_string(), "(".to_string(), "85".to_string(), "*".to_string(), "(".to_string(), "-46".to_string(), "+".to_string(), "-71".to_string(), ")".to_string(), "-12".to_string(), "+".to_string(), "26".to_string(), "-".to_string(), "59".to_string(), ")".to_string(), "+".to_string(), "84".to_string()].to_vec()));
        assert_eq!(["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"].to_vec(),expression_tree(["55".to_string(), "*".to_string(), "48".to_string(), "*".to_string(), "-44".to_string(), "-".to_string(), "-32".to_string(), "+".to_string(), "1".to_string(), "*".to_string(), "-80".to_string(), "*".to_string(), "-94".to_string(), "-".to_string(), "74".to_string(), "*".to_string(), "-53".to_string(), "+".to_string(), "-30".to_string(), "+".to_string(), "-61".to_string()].to_vec()));
    }

    use super:: consulta_operador;
    #[test]
    fn consulta_operador_test(){
        assert_eq!(false,consulta_operador("28"));
        assert_eq!(false,consulta_operador("-10"));
        assert_eq!(false,consulta_operador(")"));
        assert_eq!(true,consulta_operador("+"));
        assert_eq!(true,consulta_operador("-"));
        assert_eq!(true,consulta_operador("*"));
        assert_eq!(true,consulta_operador("/"));
    }

    use super:: consulta_digito;
    #[test]
    fn consulta_digito_test(){
        assert_eq!(false,consulta_digito("28".to_string()));
        assert_eq!(false,consulta_digito("-10".to_string()));
        assert_eq!(false,consulta_digito(")".to_string()));
        assert_eq!(true,consulta_digito("+".to_string()));
        assert_eq!(true,consulta_digito("-".to_string()));
        assert_eq!(true,consulta_digito("*".to_string()));
        assert_eq!(true,consulta_digito("/".to_string()));
    }

    use super:: parser;
    #[test]
    fn parser_test(){
        let mut n:Node = Node::new("+".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        /* node     +
                  /   \            
                 3     4
        */

        n.left = Some(Box::new(nl)); //n.left = 3
        n.right = Some(Box::new(nr)); //.right = 4
        let tree = n ;// arvore com o node criado
        let tree_aux = parser(["3".to_string(),"4".to_string(),"+".to_string()].to_vec()); //cria uma arvore para comparacao
        //compara a raiz das duas arvores
        assert_eq!(tree.data,tree_aux.data);

        let nt:Node = *tree.left.unwrap();//left node da tree
        let n1t:Node = *tree.right.unwrap();//right node da tree
        let nta:Node = *tree_aux.left.unwrap();//left node da tree aux
        let n1ta:Node = *tree_aux.right.unwrap();//right node da tree aux
        assert_eq!(nt.data,nta.data);
        assert_eq!(n1t.data,n1ta.data);
        

    }

    use super:: resolve_node;
    #[test]
    fn resolve_node_test(){
        let mut n:Node = Node::new("+".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        /* node     +
                  /   \            
                 3     4
        */
        assert_eq!(7,resolve_node(n));

        let mut n:Node = Node::new("*".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        /* node     *
                  /   \            
                 3     4
        */
        assert_eq!(12,resolve_node(n));

        let mut n:Node = Node::new("/".to_string());
        let nl:Node = Node::new("4".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        /* node     /
                  /   \            
                 4     4
        */
        assert_eq!(1,resolve_node(n));

        let mut n:Node = Node::new("-".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        /* node     -
                  /   \            
                 3     4
        */
        assert_eq!(-1,resolve_node(n));

        let mut n:Node = Node::new("/".to_string());
        let nl:Node = Node::new("0".to_string());
        let nr:Node = Node::new("0".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        /* node     /
                  /   \            
                 0     0
        */
        assert_eq!(i64::MAX,resolve_node(n));
        

    }

    use super:: pos_ordem_step;
    #[test]
    fn pos_ordem_step_test(){
        let mut n:Node = Node::new("+".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        let mut tree = n;
        /* node     +
                  /   \            
                 3     4
        */
        pos_ordem_step(&mut tree);
        /* node     7
                  /   \            
               None     None
        */
        let resultado_esperado = Node::new("7".to_string());
        assert_eq!(resultado_esperado.data,tree.data);

        let mut n:Node = Node::new("*".to_string());
        let nl:Node = Node::new("5".to_string());
        let nr:Node = Node::new("10".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        let mut tree = n;
        /* node     *
                  /   \            
                 5     10
        */
        pos_ordem_step(&mut tree);
        /* node     50
                  /   \            
               None     None
        */
        let resultado_esperado = Node::new("50".to_string());
        assert_eq!(resultado_esperado.data,tree.data);



    }

    
    use super:: find_idx;
    #[test]
    fn find_idx_test(){
        let aux = ["1".to_string(),"+".to_string(),"3".to_string()].to_vec();
        assert_eq!(1,find_idx(aux, "+".to_string()));
        let aux = ["1".to_string(),"*".to_string(),"3".to_string()].to_vec();
        assert_eq!(usize::MAX,find_idx(aux, "+".to_string()));
        let aux = ["1".to_string(),"*".to_string(),"3".to_string(),"+".to_string(),"3".to_string()].to_vec();
        assert_eq!(3,find_idx(aux, "+".to_string()));
        let aux = ["1".to_string(),"/".to_string(),"3".to_string()].to_vec();
        assert_eq!(usize::MAX,find_idx(aux, "+".to_string()));
    }

    use super::to_string;
    #[test]
    fn to_string_test(){
        let mut n:Node = Node::new("+".to_string());
        let nl:Node = Node::new("3".to_string());
        let nr:Node = Node::new("4".to_string());
        n.left = Some(Box::new(nl));
        n.right = Some(Box::new(nr));
        assert_eq!("(3 + 4)",to_string(n, "".to_string()));

    }
}