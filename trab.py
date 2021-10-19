# -*- coding: utf-8 -*-
"""
Created on Mon Oct 18 17:48:34 2021

@author: aleix
"""

"""
1.  While there are tokens to be read:
2.        Read a token
3.        If it's a number add it to queue
4.        If it's an operator
5.               While there's an operator on the top of the stack with greater precedence:
6.                       Pop operators from the stack onto the output queue
7.               Push the current operator onto the stack
8.        If it's a left bracket push it onto the stack
9.        If it's a right bracket 
10.            While there's not a left bracket at the top of the stack:
11.                     Pop operators from the stack onto the output queue.
12.             Pop the left bracket from the stack and discard it
13. While there are operators on the stack, pop them to the queue
"""

# Lexer
def tokensArray(string):
    caracteres = []
    tamNum = 0
    for i in range(len(string)):   
        if string[i].isdigit():
            tamNum += 1 # pega o tamanho do número
        else:
            if string[i-tamNum:i] != "": # pula os espaços vazios para números
                caracteres.append(string[i-tamNum:i]) # append numeros, fazendo string[inicioNum, finalNum]
            caracteres.append(string[i:i+1]) # append operadores, fazendo string[inicioOP, finalOP]
            tamNum = 0 # reseta o tamanho do numero
    print(caracteres)
    
expressao = "31 * (4 + 10)"
tokensArray(expressao.replace(" ", ""))