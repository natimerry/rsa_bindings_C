#include <stdio.h>
#include "librsa.h"
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
int main(void) {
    uint32_t bits = 2048;
    char *private_key = malloc(256);
    char *public_key = malloc(256);
    private_key = generate_priv_keys(bits);
    printf("%s", private_key);
    public_key = generate_public_key(private_key);
    printf("%s",public_key);


    Buffer buffer = encrypt_string(public_key,"hello world!");

    for (uintptr_t i =0; i < 256; i++){
        printf("%d,",buffer.data[i]);
    }
    printf("\nEnd of bytes\n");

    char *unencrypted = decrypt_string(private_key,buffer.data);
    printf("%s",unencrypted);

    return 0;
}
