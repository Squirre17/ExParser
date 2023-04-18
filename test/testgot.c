#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
int main(){
    write(1, "hello world!\n", 0x10);

}
