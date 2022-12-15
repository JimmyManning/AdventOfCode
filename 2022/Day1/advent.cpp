// reading a text file
#include <iostream>
#include <fstream>
#include <string>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
using namespace std;

int main () {
  string line;
  uint32_t MaxValue = 0;
  uint32_t SecondMaxValue = 0;
  uint32_t ThirdMaxValue = 0;
  ifstream myfile ("puzzle.txt");
  if (myfile.is_open())
  {
    while ( getline (myfile,line) )
    {
        uint32_t Value;
        if (line.size() > 0)
        {
            uint32_t CurrentValue = 0;
            sscanf(line.c_str(), "%d", &CurrentValue);
            Value += CurrentValue;
        }
        else
        {
            if (Value > MaxValue)
            {
                ThirdMaxValue = SecondMaxValue;
                SecondMaxValue = MaxValue;
                MaxValue = Value;
            }
            else if (Value > SecondMaxValue)
            {
                ThirdMaxValue = SecondMaxValue;
                SecondMaxValue = MaxValue;
            }
            else if (Value > ThirdMaxValue)
            {
                ThirdMaxValue = SecondMaxValue;
            }
            Value = 0;
        }
    }
    myfile.close();
  }

  printf("Max Value: %d\n", MaxValue); 

  printf("Second Max = %d\n", SecondMaxValue);
  printf("Third Max = %d\n", ThirdMaxValue);

  printf("Total = %d\n", MaxValue + SecondMaxValue + ThirdMaxValue);

  return 0;
}