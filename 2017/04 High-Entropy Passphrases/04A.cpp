#include <iostream>
#include <sstream>
#include <list>
#include <algorithm>


bool no_duplicates(std::string phrase)
{
	std::stringstream cin(phrase);
	std::list<std::string> word_list;

	std::string word;
	while (cin >> word)
	{
		auto itr = std::find(word_list.begin(), word_list.end(), word);
		if (itr == word_list.end())
			word_list.push_back(word);
		else
			return false;
	}
	return true;
}

void _()
{
	int solution = 0;

	std::string input;
	while (std::getline(std::cin, input))
	{
		solution += no_duplicates(input) ? 1 : 0;
	}

	std::cout << solution << std::endl;
}