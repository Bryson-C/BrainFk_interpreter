
#define slotCount 8
#define INT_MAX 2147483647
#define CELL_MAX 255
/* lang could be
    - "pragmaCLEB"
    - "pragmaBF"
    - "pragmaNucleotide"
*/
#define lang "pragmaCLEB"



#include <iostream>
#include <string.h>

const char* input = ":>:/.";

void printcells(int slots[slotCount]) {
    for (int i = 0; i < slotCount; i++) {
        printf("| %i ", slots[i]);
        if (i == slotCount-1) printf("|\n");
    }
}

int main() {
    int cells[slotCount] = {};
    int current = 0;
    char inpchr;
    
    if (lang == "pragmaBF" || lang == "pragmaCLEB") {
        for (int i = 0; i < strlen(input); i++) {
            if (input[i] == '+') cells[current]++;
            if (input[i] == '-') cells[current]--;
            if (input[i] == '>') if (current == slotCount-1) current = 0; else current++;
            if (input[i] == '<') if (current == 0) current = slotCount-1; else current--;
            if (input[i] == '.') printcells(cells);
            if (input[i] == ',') { printf("Input: "); std::cin >> inpchr; cells[current] = (int)inpchr; }
            if (input[i] == '[') if (cells[current] == 0) { while(input[i] != ']') i++; i++; }
            if (input[i] == ']') if (cells[current] != 0) { while(input[i] != '[') i--; i--; }
            if (input[i] == ' ') continue;
            if (lang == "pragmaCLEB") {
                //if (input[i] == '{') // Unknown Functionalty
                //if (input[i] == '}') // Unknown Functionalty
                if (input[i] == ';') cells[current] = 0;
                if (input[i] == ':') cells[current] = CELL_MAX;
                if (input[i] == '*') cells[current] *= 2;
                if (input[i] == '/') cells[current] /= 2;
                if (input[i] == '\\') printf("%i", cells[current]);
                if (input[i] == '|') { printf("Input: "); std::cin >> cells[current]; }
            }
        }
    }
}
