#include <sstream>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <math.h>

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

    inFile.open("day8.input");
    int numUnique = 0;
    while(getline(inFile, line))
    {
        count = 0;
        vector<string> digits;
        vector<int> digitrep;
        map<int,int> swap;
        map<int,int> index;
        stringstream s(line);
        string val;
        while(getline(s, val, ' '))
        {
            if (val == "|")
                break;
            digits.push_back(val);

            int num = 0;
            for (int i = 0; i < val.size(); i++)
            {
                num |= 1 << (val[i] - 'a' + 1);
            }
            if (val.size() == 7)
            {
                swap[num] = 8;
                index[8] = count;
            }
            else if (val.size() == 3)
            {
                swap[num] = 7;
                index[7] = count;
            }
            else if (val.size() == 2)
            {
                swap[num] = 1;
                index[1] = count;
            }
            else if (val.size() == 4)
            {
                swap[num] = 4;
                index[4] = count;
            }
            else
            {
                swap[num] = 0;

            }
            printf("%x ", num);
            digitrep.push_back(num);
            count++;
        }
        printf("\n");

        for (int i = 0; i <= 9; i++)
        {
            if (swap[digitrep[i]] == 0 && digits[i].size() == 6)
            {
                // 0 , 6, or 9
                if ((digitrep[index[1]] & digitrep[i]) == digitrep[index[1]])
                {
                    // its a 0 or 9
                    if ((digitrep[index[4]] & digitrep[i]) == digitrep[index[4]])

                    {
                        swap[digitrep[i]] = 9;
                        index[9] = i;
                    }
                    else
                    {
                        swap[digitrep[i]] = 0;
                        index[0] = i;
                    }
                    //printf( "Found 0 or 9 %x %x\n", (digitrep[index[1]] & digitrep[i]), digitrep[i]);
                }
                else
                {
                    // printf( "Found 6 %x %x\n", (digitrep[index[1]] & digitrep[i]), digitrep[i]);
                    swap[digitrep[i]] = 6;
                    index[6] = i;
                }
            }
            else if (swap[digitrep[i]] == 0 && digits[i].size() == 5)
            {
                // 2, 3, or 5
                if ((digitrep[index[1]] & digitrep[i]) == digitrep[index[1]])
                {
                    // its a 3
                    // printf( "Found 3 %x %x\n", (digitrep[index[1]] & digitrep[i]), digitrep[i]);

                    swap[digitrep[i]] = 3;
                    index[3] = i;

                }
                else
                {
                    // 2 or 5
                    //printf( "Found 2 or 5 %x %x\n", (digitrep[index[1]] & digitrep[i]), digitrep[i]);
                    if (((digitrep[index[4]] & ~digitrep[index[1]]) & digitrep[i]) == (digitrep[index[4]] & ~digitrep[index[1]]))
                    {
                        swap[digitrep[i]] = 5;
                        index[5] = i;
                    }
                    else
                    {
                        swap[digitrep[i]] = 2;
                        index[2] = i;
                    }
                }
            }
            printf("%d %s    \t %x %d\n", i, digits[i].c_str(), digitrep[i], swap[digitrep[i]]);
        }

        vector<string> numbers;
        vector<int> numbersrep;
        while(getline(s, val, ' '))
        {
            numbers.push_back(val);
            int num = 0;
            for (int i = 0; i < val.size(); i++)
            {
                num |= 1 << (val[i] - 'a' + 1);
            }
            numbersrep.push_back(swap[num]);
            printf("%d", swap[num]);

            //if (swap[num] == 1 || swap[num] == 4 || swap[num] == 7 || swap[num] == 8)
            //numUnique += swap[num] * 10 ^ digitrep.size();
            count++;
        }
        printf("\n");
        for (int i = 0; i < numbersrep.size(); i++)
        {
            numUnique += numbersrep[i] * (pow(10, (numbersrep.size() -1  - i)));
            printf("%d-%d ", numbersrep[i], numUnique);
        }
        printf("\n%d\n", numUnique);


        printf("\n");
        printf("%lu %lu\n", digits.size(), numbers.size());
        //break;

    }

    printf("%d\n", numUnique);

    return 0;
}
