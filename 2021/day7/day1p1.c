#include <sstream>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;
int main()
{

    ifstream inFile;
    string line, temp;
    int count = 0;
    int previous = 0;
    int current = 0;
    int numIncreases = 0;
    vector<int> crabs;
    int high;
    int low;

    inFile.open("day7.input");

    while(getline(inFile, line))
    {
        stringstream s(line);
        string val;
        while(getline(s, val, ','))
        {
        current = atoi(val.c_str());
        if (count == 0)
        {
            high = current;
            low = current;
        }
        printf("%d %d %d\n", current, low, high);
        crabs.push_back(current);
        if (current > high)
        {
            high = current;
        }
        if (current < low)
        {
            low = current;
        }
        count++;
        }
    }

    int shortestPath;
    printf("%d %d %lu\n", low, high, crabs.size());

    for (int i = low; i <= high; i++)
    {

        int currentPath = 0;
        for (int j = 0; j < crabs.size(); j++)
        {
            int steps = 0;
            if (crabs[j] > i)
            {
                steps = (crabs[j] - i);
            }
            if (crabs[j] < i)
            {
                steps += (i - crabs[j]);
            }
            for (int i = 1; i <= steps; i++)
            {
                currentPath += i;
            }
        }

        if (i == low)
        {
            shortestPath = currentPath;
        }
        if (currentPath < shortestPath)
        {
            shortestPath = currentPath;
        }
        printf("%d %d\n", i, currentPath);
    }


    printf("Answer %d\n", shortestPath);

    return 0;
}
