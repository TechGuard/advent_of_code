#include <iostream>
#include <sstream>
#include <map>
#include <vector>
#include <queue>


namespace
{
	std::vector<std::string> input;

	using i64 = std::int64_t;

	struct Program
	{
		std::map<char, i64> registers;
		int position;
		bool running;
		int send_count;

		bool waiting;
		char wait_register;

		std::queue<i64> messages;

		Program(i64 thread) : position(0), running(true), waiting(false), send_count(0)
		{
			registers['p'] = thread;
		}
	
		i64 parse_value(std::stringstream& cin)
		{
			std::string value;
			cin >> value;
			char reg = value[0];

			if (reg >= 'a' && reg <= 'z')
				return registers.at(reg);

			return std::atoi(value.c_str());
		}

		void next(Program& other)
		{
			if (!running)
				return;

			if (waiting)
			{
				if (messages.size() > 0)
				{
					registers[wait_register] = messages.front();
					messages.pop();
					waiting = false;
				}
				return;
			}

			std::stringstream cin(input[position]);

			std::string op;
			cin >> op;

			char reg;

			if (op == "snd") // send
			{
				other.messages.push(parse_value(cin));
				++send_count;
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
			else if (op == "rcv") // receive
			{
				cin >> wait_register;
				waiting = true;
				next(other);
			}

			if (op == "jgz" && parse_value(cin) > 0) // jump
			{
				position += (int)parse_value(cin);
			}
			else
			{
				++position;
			}

			running = position < (int)input.size();
		}
	};
}

void main()
{
	std::string line;
	while (std::getline(std::cin, line)) input.push_back(line);

	Program p0(0);
	Program p1(1);

	while ((p0.running || p1.running) && !(p0.waiting && p1.waiting))
	{
		p0.next(p1);
		p1.next(p0);
	}

	std::cout << p1.send_count << std::endl;
}