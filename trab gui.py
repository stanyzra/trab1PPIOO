# -*- coding: utf-8 -*-
import math #usando a função math.floor
ops = {
  "+": (lambda a, b: a + b),
  "-": (lambda a, b: a - b),
  "*": (lambda a, b: a * b),
  "/": (lambda a, b: a / b),
}
class Node:
   def __init__(self, data):
      self.left = None
      self.right = None
      self.data = data
      self.parent = None

class Tree:
   def __init__(self, data):
     self.root = data

   def isRoot(self, node):
     return True if self.root == node else False

   def getLastNode(self, root):
    expression = root
    if expression.data in ops:
      evalStep.lastNode = expression
    if root.left is not None:
            self.getLastNode(root.left)
    if root.right is not None:
            self.getLastNode(root.right) 
    return evalStep.lastNode

def toString(tree, root, nextOp):
    expression = root.data
    atualOp = ""
    if root.data in ops:
      atualOp = root.data
    if root.left is not None:
            exp1 = toString(tree, root.left, atualOp)
            expression = exp1 + " " + expression
    if root.right is not None:
            exp2 = toString(tree, root.right, atualOp)
            expression = expression + " " + exp2

    for op in ops:
      opIndex = expression.find(op)
      if opIndex != -1 and expression[opIndex+1].isdigit() == False:
        if (precedencia(atualOp) < precedencia(nextOp)):
          expression = "(" + expression + ")"
    
        break
    return expression

def evalStep(tree, root):
    lastNode = tree.getLastNode(root)
    if lastNode.left is None or lastNode.right is None:
      return None
    num1 = int(lastNode.left.data)
    num2 = int(lastNode.right.data)
    op = lastNode.data
    result = math.floor(ops[op](num1, num2))
    lastNode.right = None
    lastNode.left = None
    lastNode.data = str(result)
    return tree


def parseTree(expList):
  stack = []
  for token in expList:
    if token in ops and len(token) == 1:
      node = Node(token)
      t1 = stack.pop()
      t2 = stack.pop()
      node.right = t1
      t1.parent = node
      node.left = t2
      t2.parent = node
      stack.append(node)
    else:
      node = Node(token)
      stack.append(node)

  tree = Tree(stack.pop())
  return tree
  
  #print(toString(tree, tree.root, None))
def precedencia(operador):
    if operador is None:
      return -1
    return 1 if operador == "+" or operador == "-" else 2 # 1 para mais e menos e 2 para outros

# Lexer
def tokensArray(string):
    caracteres = []
    brackets = ['(', ')']
    tamNum = 0
    index = 0
    for i in string:
      if i in ops:
        if index != 0:
          if string[index-1] not in ops and string[index-1] not in brackets:            
            if string[tamNum] in ops and string[tamNum] != '-':
                caracteres.append(string[tamNum])
                tamNum += 1
            caracteres.append(string[tamNum:index])
            caracteres.append(string[index])
            tamNum = index+1
          elif string[index-1] in brackets and string[index+1] in brackets:
            caracteres.append(i)
          elif string[index-1] == ')' and string[index+1] not in ops and string[index] == '-':
            caracteres.append(i)
            tamNum = index+1
      elif i == '(':
        caracteres.append(i)
        tamNum = index+1
      elif i == ')':
        if string[tamNum] in ops and string[tamNum] != '-':
            caracteres.append(string[tamNum])
            tamNum += 1
        if string[tamNum] != ')':
            caracteres.append(string[tamNum:index])
        caracteres.append(string[index])
        tamNum = index+1
      elif index == len(string) - 1:
        if string[tamNum] in ops and string[tamNum] != '-':
            caracteres.append(string[tamNum])
            tamNum += 1
        caracteres.append(string[tamNum:len(string) + 1])
      index += 1
    # print(caracteres)
    return caracteres
    
# Shunting-yard
def expressionTree(lista):
    ops = ["*", "/", "+", "-"]
    listaOP = []
    listaNum = []
    for i in lista:
        if i.lstrip('-').isdigit(): #lstrip para tirar o sinal '-' pois a função isdigit nao considera numero negativo como numero
            listaNum.append(i)
        elif i in ops:
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
    return listaNum
  
def calcula(expressao):
  listaToken = tokensArray(expressao.replace(" ", ""))
  tree = parseTree(expressionTree(listaToken))
  while tree is not None:
    tree = evalStep(tree, tree.root)
    if tree is None:
      break
    expTree = toString(tree, tree.root, None)
    print(expTree)
expressao = input()
calcula(expressao)