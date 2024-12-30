#include <iostream>
#include <sstream>
#include <map>


void _()
{
	const int length = 16;
	char programs[length];

	for (int i = 0; i < length; i++)
		programs[i] = 'a' + i;

	std::string input;
	while (std::getline(std::cin, input, ','))
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

	char output[length + 1];
	memcpy_s(output, sizeof output, programs, sizeof programs);
	output[length] = '\0';

	std::cout << output << std::endl;
}