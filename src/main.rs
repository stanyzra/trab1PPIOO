use trabalho::parser ;
use trabalho::expression_tree;
use trabalho::lexer;
use trabalho::pos_ordem_step;
use trabalho::to_string;
use std::io;

fn main() {
    
    loop{
        println!("Digite sua expressao ...");
        let mut x = String::new() ; //string para receber o input do usuario
        io::stdin().read_line(&mut x); //funcao para pegar input
        x.pop();                        // retirando o \n do input
        let exp:Vec<String> = lexer(x.replace(" ", "").to_string()); //passando a string para um vetor sem espaços
        let exp_tree = expression_tree(exp); //criando o vetor com a notacao polonesa inversa
        let mut tree =parser(exp_tree);// craindo a arvore com a expressao
        
        //println!("{:#?}",tree);
        let mut controle = true;
        while !tree.left.is_none() && !tree.right.is_none() && controle{  //laco que resolve a expressao
            controle = pos_ordem_step(&mut tree);  //caminhamento pos ordem
            println!("{}",to_string(tree.clone(), "".to_string()));
        }
       
    }
    
    
}
