#include <iostream>
#include <sstream>
#include <map>
#include <vector>


namespace
{
	using i64 = std::int64_t;

	std::map<char, i64> registers;

	i64 parse_value(std::stringstream& cin)
	{
		std::string value;
		cin >> value;
		char reg = value[0];

		if (reg >= 'a' && reg <= 'z')
			return registers[reg];

		return std::atoi(value.c_str());
	}
}

void _()
{
	std::string line;
	std::vector<std::string> input;
	while (std::getline(std::cin, line)) input.push_back(line);
	
	int position = 0;
	int size = input.size();

	i64 frequency;

	while (position < size)
	{
		std::stringstream cin(input[position]);

		std::string op;
		cin >> op;

		char reg;

		if (op == "snd") // play
		{
			frequency = parse_value(cin);
		}
		else if (op == "set") // set
		{
			cin >> reg;
			registers[reg] = parse_value(cin);
		}
		else if (op == "add") // increase
		{
			cin >> reg;
			registers[reg] += parse_value(cin);
		}
		else if (op == "mul") // multiply
		{
			cin >> reg;
			registers[reg] *= parse_value(cin);
		}
		else if (op == "mod") // modulo
		{
			cin >> reg;
			registers[reg] %= parse_value(cin);
		}
		else if (op == "rcv" && parse_value(cin) != 0) // recover
		{
			break;
		}
		
		if (op == "jgz" && parse_value(cin) > 0) // jump
		{
			position += (int) parse_value(cin);
		}
		else
		{
			++position;
		}
	}

	std::cout << frequency << std::endl;
}