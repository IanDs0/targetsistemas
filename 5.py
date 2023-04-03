'''

5) Escreva um programa que inverta os caracteres de um string.

IMPORTANTE:
a) Essa string pode ser informada através de qualquer entrada de sua preferência ou pode ser previamente definida no código;
b) Evite usar funções prontas, como, por exemplo, reverse;

'''

def inverta(palavra):
    nova_palavra = ""
    for i in range(-1, (len(palavra)+1)*-1, -1):
        nova_palavra+=palavra[i]
        
    return nova_palavra
        
palavra = input("Digite a palavra que deseja inverter: ")
print(inverta(palavra))