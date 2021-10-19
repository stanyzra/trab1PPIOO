
def precedencia(operador):
    return 1 if operador == "+" or operador == "-" else 2

def separaTk(exemplo):
    retorno = []
    aux = []
    i = 0
    while i < len(exemplo):
        if exemplo[i] == "-":
            if exemplo[i+1] != " ":
                aux.append(exemplo[i])
                i+=1
            else:
                retorno.append(exemplo[i])
                i+=1
        elif exemplo[i] == " ":
            i+=1
        elif exemplo[i] == "(" or exemplo[i] == ")" or exemplo[i] == "+" or exemplo[i] == "*" or exemplo[i] == "/":
            retorno.append(exemplo[i])
            i+=1
        else:
            aux.append(exemplo[i])
            i+=1
            while i < len(exemplo) and (exemplo[i] != " " and exemplo[i] != ")" and exemplo[i] != "(" ):
                aux.append(exemplo[i]) 
                i+=1
            aux = "".join(aux)
            aux = int(aux)
            retorno.append(aux)
            aux = []
    
    return retorno
            
def avaliador(entrada):
    pilha = []
    fila = []
    exp = separaTk(entrada)
    tk = ["*", "/", "+", "-", "(", ")"]
    i = 0
    while i < len(exp):
        if exp[i].isnumeric():
            fila.append(exp[i])
        if exp[i] == "+" or exp[i] == "-" or exp[i] == "*" or exp[i] == "/":
            while len(pilha) > 0 and precedencia(pilha[len(pilha)-1]) == 2:
                fila.append(pilha.pop())
            pilha.append(exp[i])
        if exp[i] == "(":
            pilha.append(exp[i])
        if exp[i] == ")":
            while len(pilha)>0 and pilha[len(pilha)-1]!="(":
                fila.append(pilha.pop())
            
        
        i+=1

def main():
    exp = "(10 / 3 + 23) * (1 - 4)"
    teste2 = "58 - -8 * (58 + 31) - -14"
    #avaliador(exp)
    print(separaTk(teste2))
 
if __name__ == "__main__":
    main()


