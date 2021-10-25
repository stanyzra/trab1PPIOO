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
            if (string[tamNum] in ops and string[tamNum] != '-') or (string[tamNum] in ops and string[tamNum+1] == "-"):
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
        if (string[tamNum] in ops and string[tamNum] != '-') or (string[tamNum] in ops and string[tamNum+1] == "-"):
            caracteres.append(string[tamNum])
            tamNum += 1
        if string[tamNum] != ')':
            caracteres.append(string[tamNum:index])
        caracteres.append(string[index])
        tamNum = index+1
      elif index == len(string) - 1:
        if (string[tamNum] in ops and string[tamNum] != '-') or (string[tamNum] in ops and string[tamNum+1] == "-"):
            caracteres.append(string[tamNum])
            tamNum += 1
        caracteres.append(string[tamNum:len(string) + 1])
      index += 1
    # print(caracteres)
    return caracteres
    
def resultadoExp(expressao):
    listaToken = tokensArray(expressao.replace(" ", ""))
    tree = parseTree(expressionTree(listaToken))
    resultadoFinal = None
    while tree is not None:
        tree = evalStep(tree, tree.root)
        if tree is None:
            continue
        expTree = toString(tree, tree.root, None)
        # print(expTree)
        resultadoFinal = expTree
    return int(resultadoFinal)

def testes():
    assert resultadoExp("1 + 3") == 4
    assert resultadoExp("1 + 2 * 3") == 7
    assert resultadoExp("4 / 2 + 7") == 9
    assert resultadoExp("1 + 2 + 3 * 4") == 15
    assert resultadoExp("(1 + 2 + 3) * 4") == 24
    assert resultadoExp("(10 / 3 + 23) * (1 - 4)") == -78
    assert resultadoExp("((1 + 3) * 8 + 1) / 3") == 11
    assert resultadoExp("58 - -8 * (58 + 31) - -14") == 784  #erro no -14 pegando --14
    assert resultadoExp("-71 * (-76 * 91 * (10 - 5 - -82) - -79)") == 42714523 #mesmo erro com --
    assert resultadoExp("10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4") == 239
    assert resultadoExp("(-13 - -73) * (44 - -78 - 77 + 42 - -32)") == 7140
    assert resultadoExp("-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28") == -1241
    assert resultadoExp("-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41") == -141883
    assert resultadoExp("80 * -18 * (85 * (-46 + -71) - 12 + 26 - 59) + 84") == 14385684
    assert resultadoExp("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)") == 135290
    assert resultadoExp("(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)") == 67272
    assert resultadoExp("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30") == -1954
    assert resultadoExp("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53") == 580062
    assert resultadoExp("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)") == -3357342660
    assert resultadoExp("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78") == 5965
    assert resultadoExp("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)") == 189772
    assert resultadoExp("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61") == -104777
    assert resultadoExp("-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51") == 177958 #mesmo erro com --
    assert resultadoExp("(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))") == -147799088242
    assert resultadoExp("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42") == -1524
    
    
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
# expressao = input()
# expressao = "-71 * (-76 * 91 * (10 - 5 - -82) - -79)"
# tokensArray(expressao.replace(" ", ""))
testes()
# calcula(expressao)