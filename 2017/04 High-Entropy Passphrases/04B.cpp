#include <iostream>
#include <sstream>
#include <list>
#include <algorithm>


bool no_anagrams(std::string phrase)
{
	std::stringstream cin(phrase);
	std::list<std::string> word_list;

	std::string word;
	while (cin >> word)
	{
		std::sort(word.begin(), word.end());
		auto itr = std::find(word_list.begin(), word_list.end(), word);
		if (itr == word_list.end())
			word_list.push_back(word);
		else
			return false;
	}
	return true;
}

void main()
{
	int solution = 0;

	std::string input;
	while (std::getline(std::cin, input))
	{
		solution += no_anagrams(input) ? 1 : 0;
	}

	std::cout << solution << std::endl;
}