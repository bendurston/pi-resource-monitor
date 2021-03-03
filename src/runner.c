#include <stdio.h>
#include <stdlib.h>
#include <string.h>




int getRamStats(){
    FILE *fp;
    fp = popen("/usr/bin/free", "r");
    if (fp == NULL){
        printf("Command failed\n");
        exit(1);
    }

    char buffer[2056];
    size_t charRead;
    size_t comalloc = 256;
    size_t comlen = 0;

    char *almem = malloc(comalloc);

    while((charRead = fread(buffer, 1, sizeof(buffer), fp)) != 0) {
        if (comlen  + charRead >= comalloc){
            comalloc *= 2;
            almem = realloc(almem, comalloc);
        }
        memmove(almem + comlen, buffer, charRead);
        comlen  += charRead;
    }
    int x=0;
    while(x<=comlen){
        printf("%c", buffer[x]);
        x = x + 1;
    }
    //fwrite(almem, 1, comlen, stdout);
    free(almem);
    pclose(fp);
    return 0;
    
}




void initialize(){
    // This is where the program will sleep after garbage collection
    getRamStats();
    //printf("%c\n", *pt);
}


void main(){
    initialize();
}
