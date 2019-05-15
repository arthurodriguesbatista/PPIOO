use std::collections::VecDeque;
use std::io::{self, BufRead};

fn lexer(expression: &String) -> VecDeque<String>{
	let mut expression_token: VecDeque<String> = VecDeque::new();
	let mut aux: Vec<char> = Vec::new();
	let special_characters = vec!['*','/','+','-','(',')'];
	let mut count = 0;

	for element in expression.chars() { //retira os espaços da string expression separando cada elemento em uma posição do vetor
		if element.is_digit(10) || special_characters.contains(&element) {
			aux.push(element);    
		}
	}

	if aux[count] == '-'{ //tratamento para o caso de começar com um número negativo
		let mut negative_number: String = "-".to_string();
		count = count + 1;
		while aux[count].is_digit(10) { // enquanto tiver algarismos
			negative_number.push(aux[count]);
			count = count + 1;
			if count == aux.len() {
				break;
			}
		}
		expression_token.push_back(negative_number);
	}

	while count < aux.len() { // junção dos alagrismos
		let mut num: String = "".to_string(); 
		while aux[count].is_digit(10) {
			num.push(aux[count]);
			count = count + 1;
			if count == aux.len() {
				break;
			}
		}
		if num != "".to_string() { // se um número foi formado
			expression_token.push_back(num);
		}else{
			if aux[count] == '-' && aux[count + 1].is_digit(10) && special_characters.contains(&aux[count-1]) { //tratamento dos números negativos
				let mut negative_number: String = "-".to_string();
				count = count + 1;
				while aux[count].is_digit(10) { // enquanto tiver alagrismos
					negative_number.push(aux[count]);
					count = count + 1;
					if count == aux.len() {
						break;
					}
				}
				expression_token.push_back(negative_number);
			}else{ // caso seja um caracter especial
				expression_token.push_back(aux[count].to_string());
				count = count + 1;
			}
		}
	}
	return expression_token
}   

fn greater_or_equal_precedence(operator1: &String, operator2: &String, operators: &Vec<String>) -> bool {
	if !operators.contains(&operator2) { // se o topo da pilha for '(' ou ')' 
		return false
	} else if operator2 == &'/'.to_string() || operator2 == &'*'.to_string() { // se o topo da pilha tiver maior precedência
		return true 
	} else if operator1 == &'/'.to_string() || operator1 == &'*'.to_string() { // se o elemento da fila tiver maior precedência
		return false
	} 
	return true // se ambos tiverem mesma precedência
}

fn parser(tokens: &mut VecDeque<String>) -> VecDeque<String> {
	let mut stack: VecDeque<String> = VecDeque::new();
	let mut queue: VecDeque<String> = VecDeque::new();
	let operators = vec![String::from("*"),String::from("/"),String::from("+"),String::from("-")]; // contém os operadores
	let mut element: String;
	while !tokens.is_empty() { // enquanto tiver elementos dentro de tokens
		element = tokens.pop_front().unwrap(); // pegue o valor do elemento no inicio da fila
		if element.parse::<f64>().is_ok() { // verfica-se se o valor é um numero
			queue.push_back(element);
		} else if operators.contains(&element) { // se o elemento for um operator
				while !stack.is_empty() { // enquanto haver elementos no topo da pilha com maior pecedência
					let top_of_stack = &stack[stack.len()-1];
					if greater_or_equal_precedence(&element, &top_of_stack, &operators) {
						queue.push_back(stack.pop_back().unwrap()); // adicionamos o elemento do topo da pilha ao final da fila
					} else {
						break;
					}
				}
				stack.push_back(element);
		} else if element == '('.to_string() {
				stack.push_back(element);
		} else if element == ')'.to_string() { 
				while stack[stack.len()-1] != '('.to_string(){ // enquanto o topo da pilha for diferente de "("  adicionamos os operadores à fila
					queue.push_back(stack.pop_back().unwrap());
				}
				stack.pop_back();
		} else { // se for um caracter inválido
				panic!("Caracter inválido: {}", element);
		}
	}
	while !stack.is_empty() { // enquanto existirem operadores na pilha
		queue.push_back(stack.pop_back().unwrap()); // adicionamos os operadores no topo no final da final
	}

	return queue;
}

fn createTree()

fn main() {
		let mut expression: String = String::new();
		let mut token: VecDeque<String> = VecDeque::new();

		let stdin = io::stdin();
		stdin.lock().read_line(&mut expression).expect("FOI");
		
		while expression.len() > 0 && expression != "\n".to_string() { // enquanto tiver input
				expression = expression.trim().to_string(); // transformar input em string
				token = lexer(&expression); 
				let root = parser(&mut token); // raiz da ávore
				expression.clear();
				stdin.lock().read_line(&mut expression).expect("FOI");
						
		}
    
}
