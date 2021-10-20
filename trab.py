# -*- coding: utf-8 -*-

"""
Created on Mon Oct 18 17:48:34 2021

@author: aleix
"""

class NoArvore:
    def __init__(self, chave=None, esquerda=None, direita=None):
        self.chave = chave
        self.esquerda = esquerda
        self.direita = direita

    def __repr__(self):
        return '%s <- %s -> %s' % (self.esquerda and self.esquerda.chave,
                                    self.chave,
                                    self.direita and self.direita.chave)

def precedencia(operador):
    return 1 if operador == "+" or operador == "-" else 2 # 1 para mais e menos e 2 para outros

# Lexer
def tokensArray(string):
    caracteres = []
    tamNum = 0
    for i in range(len(string)):
        if string[i].isdigit():
          if (i == len(string) - 1):
            caracteres.append(string[i-tamNum:i+1])
          tamNum += 1 # pega o tamanho do número
        else:
            if string[i-tamNum:i] != "": # pula os espaços vazios para números
                caracteres.append(string[i-tamNum:i]) # append numeros, fazendo string[inicioNum, finalNum]
            caracteres.append(string[i:i+1]) # append operadores, fazendo string[inicioOP, finalOP]
            tamNum = 0 # reseta o tamanho do numero
    print(caracteres)
    return caracteres
    
# Shunting-yard
def RPN(lista):
    ops = ["*", "/", "+", "-"]
    listaOP = []
    listaNum = []
    for i in lista:
        if i.isdigit():
            listaNum.append(i)
        if i in ops:
            while len(listaOP) > 0 and precedencia(listaOP[len(listaOP)-1]) >= precedencia(i) and listaOP[len(listaOP)-1] != "(":
                listaNum.append(listaOP.pop())
            listaOP.append(i)
        if i == "(":
            listaOP.append(i)
        if i == ")":
            while len(listaOP) > 0 and listaOP[len(listaOP)-1] != "(":
                listaNum.append(listaOP.pop())
            listaOP.pop()
            
    while len(listaOP) > 0:
        listaNum.append(listaOP.pop())

    print(listaNum)
    return listaNum


expressao = "1 * 2 + ( 3 + ( 4 + 5 ) * 6) * 71"
# print(expressao[10:11])
listaToken = tokensArray(expressao.replace(" ", ""))
RPN(listaToken)  

