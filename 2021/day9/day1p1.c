#include <sstream>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <set>

using namespace std;

int FindBasin(vector<vector<pair<int,int> > > & grid, int x, int y)
{
    int size = 0;
    //printf("x: %d y: %d\n", x, y);
    if (y >= 0 && y < grid.size() && x >= 0 && x < grid[y].size())
    if (!grid[y][x].second && grid[y][x].first != 9)
    {
        size = 1;
        grid[y][x].second = 1;
        if (y > 0)
        {
            size += FindBasin(grid, x, y - 1);
        }
        if ((y + 1) < grid.size())
        {
            size += FindBasin(grid, x, y + 1);
        }
        if (x > 0)
        {
            size += FindBasin(grid, x - 1, y);
        }
        if ((x + 1) < grid[y].size())
        {
            size += FindBasin(grid, x + 1, y);
        }
    }
    return size;
}

int main()
{
    ifstream inFile;
    string line, temp;
    int count = 0;
    int answer = 0;
    inFile.open("day9.input");


    vector<vector<pair<int,int> > > grid;
    while(getline(inFile, line))
    {
        vector<pair<int,int> > row;
        for (int i = 0; i < line.size(); i++)
        {
            char a[2];
            a[0] = line[i];
            a[1] = 0;
            row.push_back(pair<int, int>(atoi(a), 0));
        }
        grid.push_back(row);
        count++;
    }
    multiset<int> basin;
    for (int i = 0; i < grid.size(); i++)
    {
        for (int j = 0; j < grid[i].size(); j++)
        {
            int lowPoint = 1;
            if (!grid[i][j].second)
            if (i > 0)
            {
                if (grid[i-1][j].first <= grid[i][j].first)
                    lowPoint = 0;
            }
            if ((i + 1) < grid.size())
            {
                if (grid[i+1][j].first <= grid[i][j].first)
                    lowPoint = 0;
            }
            if (j > 0)
            {
                if (grid[i][j-1].first <= grid[i][j].first)
                    lowPoint = 0;
            }
            if ((j + 1) < grid[i].size())
            {
                if (grid[i][j+1].first <= grid[i][j].first)
                    lowPoint = 0;
            }

            if (lowPoint)
            {
                int currentBasin = FindBasin(grid, j, i);;
                printf(".");

                basin.insert(currentBasin);

                answer += (grid[i][j].first + 1);
            }


            printf("%d", grid[i][j].first);
        }
        printf("\n");

    }

    printf("Answer %d\n", answer);
    for (auto b = basin.begin(); b!= basin.end(); b++)
    {
        printf("%d\n", *b);
    }
/*
    for (int i = 0; i < grid.size(); i++)
    {
        for (int j = 0; j < grid[i].size(); j++)
        {
            if (grid[i][j].second)
            {
                //printf("\e[1m%d\e[0m", grid[i][j].first);
                printf(".%d", grid[i][j].first);

            }
            else
            {
                printf(" %d", grid[i][j].first);
            }
        }
        printf("\n");
    }
*/
    return 0;
}
