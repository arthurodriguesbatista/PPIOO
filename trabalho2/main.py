import os
import time
from selectionsort import selectionSort
from bubblesort import bubbleSort
from heapsort import heapify, heapSort
import random



def inicialize(graph):
	element = [" " for i in range(len(graph))]
	graph = [element.copy() for i in range(len(graph))]
	return graph


def alterGraph(graph, order):
	for i in range(len(order)):
		graph[i][order[i]-1] = order[i]


def printInstance(graph):
	for inter in graph:
		print()
		for j in inter:
			print(j, end=" ")
	print()

quantalgoritmos = 3
quan = int(input("Digite a quantidade de algoritmos para serem simulados (max 3): "))

if quan not in [1,2,3]:
	raise ValueError(quan,' é uma opção inválida!')  

options = []
for i in range(quan):
	op = int(input("Selecione 0-BubbleSort 1-SelectionSort 2-HeapSort: "))
	if op not in options and op in [0,1,2]:
		options.append(op)
	else:
		raise ValueError(op,' é uma opção inválida ou repetida!')     

ops = int(input("Selecione 1-Full Simulation 2-Step by Step: "))

if ops not in [1,2]:
	raise ValueError(ops,' é uma opção inválida!')  

os.system("clear")

order = [[] for i in range(quantalgoritmos)]
graph = [[] for i in range(quantalgoritmos)]
algortime = [[] for i in range(quantalgoritmos)]

notOrdered = random.sample(range(1,10001), 10000)

for op in range(len(options)):
	if options[op] == 0:
		bubbleSort(notOrdered.copy(), order[options[op]],algortime[options[op]])
	elif options[op] == 1:
		selectionSort(notOrdered.copy(), order[options[op]],algortime[options[op]])
	elif options[op] == 2:
		heapSort(notOrdered.copy(), order[options[op]],algortime[options[op]])

max = 0
values = {}

for i in options:
	values[i] = len(order[i])
	if len(order[i]) > max:
		max = len(order[i])

for i in algortime:
	print(i[-1])
	print()

# if ops == 1:
# 	for i in range(max):
# 		for op in options:
# 			if order[op]:	
# 				if op == 0:print("BUBBLE SORT\n")
# 				elif op == 1:print("SELECTION SORT\n")
# 				else:print("HEAP SORT\n")
# 				if i < values[op]:
# 						print("Quantidade de trocas: ",i+1)
# 						print("Tempo total de execução(s):", algortime[op][i])
# 						graph[op] = inicialize(notOrdered.copy())
# 						alterGraph(graph[op], order[op][i])	
# 				else:
# 					print("Quantidade de trocas: ",values[op])
# 					print("Tempo total de execução(s):", algortime[op][values[op]-1])
# 				printInstance(graph[op])
# 		time.sleep(1)
# 		os.system("clear")		



# if ops == 2:
# 	Input = input("Digite enter para continuar ou 0 para sair:")
# 	for i in range(max):
# 		while not Input:
# 			for op in options:
# 				if order[op]:	
# 					if op == 0:print("BUBBLE SORT\n")
# 					elif op == 1:print("SELECTION SORT\n")
# 					else:print("HEAP SORT\n")
					
# 					if i < values[op]:
# 						print("Quantidade de trocas: ",i+1)
# 						print("Tempo total de execução(s):", algortime[op][i])
# 						graph[op] = inicialize(notOrdered.copy())
# 						alterGraph(graph[op], order[op][i])	
# 					else:
# 						print("Tempo total de execução(s):", algortime[op][values[op]-1])
# 						print("Quantidade de trocas: ",values[op])
# 					printInstance(graph[op])	

# 			Input = input("Digite enter para continuar ou 0 para sair:")
# 			if not Input:
# 				os.system("clear")	
# 				break
# 			os.system("clear")		
				
