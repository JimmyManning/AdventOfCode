#include <sstream>
#include <fstream>
#include <iostream>
#include <string>

using namespace std;
int main()
{
    ifstream inFile;
    string line, temp;
    int count = 0;
    int previous = 0;
    int current = 0;
    int numIncreases = 0;

    int A = 0;
    int ACount = 0;
    int B = 0;
    int BCount = 0;
    int C = 0;
    int CCount = 0;
    int D = 0;
    int DCount = 0;;


    inFile.open("day1.input");

    while(getline(inFile, line))
    {
        current = atoi(line.c_str());

        if (ACount < 3)
        {
            A += current;
            ACount++;
        }
        if (count > 0 && BCount < 3)
        {
            B += current;
            BCount++;
        }
        if (count > 1 && CCount < 3)
        {
            C += current;
            CCount++;
        }
        if (count > 2 && DCount < 3)
        {
            D += current;
            DCount++;
        }

        if (BCount == 3 && ACount == 3)
        {
            if (B > A)
            {
                printf("B > A %d %d\n", B, A);
                numIncreases++;
            }

            A = 0;
            ACount = 0;
        }

        if (CCount == 3 && BCount == 3)
        {
            if (C > B)
            {
                printf("C > B %d %d\n", C, B);
                numIncreases++;
            }

            B = 0;
            BCount = 0;
        }

        if (DCount == 3 && CCount == 3)
        {
            if (D > C)
            {
                printf("D > C %d %d\n", D, C);
                numIncreases++;
            }

            C = 0;
            CCount = 0;
        }

        if (ACount == 3 && DCount == 3)
        {
            if (A > D)
            {
                printf("A > D %d %d\n", A, D);
                numIncreases++;
            }

            D = 0;
            DCount = 0;
        }





        count++;
    }

    printf("Answer %d\n", numIncreases);

    return 0;
}
