
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
  uint64_t Score = 0;
  ifstream myfile ("puzzle.txt");
  if (myfile.is_open())
  {
    while ( getline (myfile,line) )
    {
        char MyMove;
        char TheirMove;

        sscanf(line.c_str(), "%c %c", &TheirMove, &MyMove);
        printf("%c %c\n", MyMove, TheirMove);
        switch (MyMove)
        {
            case 'X':
                //Score += 1;
                switch(TheirMove)
                {
                    case 'A':
                    Score +=3;
                    break;
                    case 'B':
                    Score +=1;
                    break;
                    case 'C':
                    Score +=2;
                    break;
                }
            break;
            case 'Y':
                Score += 3;
                switch(TheirMove)
                {
                    case 'A':
                    Score +=1;
                    break;
                    case 'B':
                    Score +=2;
                    break;
                    case 'C':
                    Score +=3;
                    break;
                }
            break;
            case 'Z':
                Score += 6;
                switch(TheirMove)
                {
                    case 'A':
                    Score +=2;
                    break;
                    case 'B':
                    Score +=3;
                    break;
                    case 'C':
                    Score +=1;
                    break;
                }
            break;
        }

    }
    myfile.close();
  }
  else
  {
    printf("Failed\n");
  }

  printf("Score: %llu\n", Score); 

  return 0;
}