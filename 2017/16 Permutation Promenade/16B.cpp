#include <iostream>
#include <sstream>


namespace
{
	const int length = 16;
	char programs[length];

	void execute(std::string allInput)
	{
		std::string input;
		std::stringstream allCin(allInput);

		while (std::getline(allCin, input, ','))
		{
			std::stringstream cin(input);

			char op;
			cin >> op;

			if (op == 's')
			{
				int offset;
				cin >> offset;

				std::rotate(std::rbegin(programs), std::rbegin(programs) + (offset % length), std::rend(programs));
			}
			else if (op == 'x')
			{
				int a;
				cin >> a;

				cin >> op; // skip

				int b;
				cin >> b;

				std::swap(programs[a], programs[b]);
			}
			else if (op == 'p')
			{
				char a;
				cin >> a;

				cin >> op; // skip

				char b;
				cin >> b;

				int x, y;

				for (int i = 0; i < length; ++i)
					if (programs[i] == a) x = i;
					else if (programs[i] == b) y = i;

					std::swap(programs[x], programs[y]);
			}
		}
	}
}

void main()
{
	for (int i = 0; i < length; i++)
		programs[i] = 'a' + i;

	std::string allInput;
	std::getline(std::cin, allInput);

	char output[length + 1];
	output[length] = '\0';

	int round = 0;
	while (true)
	{
		++round;
		execute(allInput);
		memcpy_s(output, sizeof output, programs, sizeof programs);

		if (std::string(output) == std::string("abcdefghijklmnop"))
			break;
	}

	int offset = 1000000000 % round;

	std::cout << "Repeat at " << round << ", " << offset << " rounds left." << std::endl;

	for (int i = 0; i < offset; ++i)
	{
		execute(allInput);
		memcpy_s(output, sizeof output, programs, sizeof programs);
	}

	std::cout << output << std::endl;
}