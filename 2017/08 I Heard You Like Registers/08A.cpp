#include <iostream>
#include <sstream>
#include <unordered_map>


namespace
{
	std::unordered_map<std::string, int> registers;

	bool is_true(std::string reg, std::string op, int value)
	{
		if (op == "==")		 return registers[reg] == value;
		else if (op == "!=") return registers[reg] != value;
		else if (op == ">")  return registers[reg] > value;
		else if (op == ">=") return registers[reg] >= value;
		else if (op == "<")  return registers[reg] < value;
		else if (op == "<=") return registers[reg] <= value;
		return false;
	}

	void do_operation(std::string reg, std::string op, int value)
	{
		if (op == "inc")	  registers[reg] += value;
		else if (op == "dec") registers[reg] -= value;
	}
}

void _()
{
	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		std::string register1;
		std::string operator1;
		int value1;

		std::string register2;
		std::string operator2;
		int value2;

		cin >> register1;
		cin >> operator1;
		cin >> value1;
		
		cin >> register2; // skip word 'if'

		cin >> register2;
		cin >> operator2;
		cin >> value2;

		if (is_true(register2, operator2, value2))
			do_operation(register1, operator1, value1);
	}

	int highest_value = 0;

	for (const auto& kp : registers)
		if (kp.second > highest_value)
			highest_value = kp.second;

	std::cout << highest_value << std::endl;
}