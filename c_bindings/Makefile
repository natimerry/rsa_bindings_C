CFLAGS= -Wno-discarded-qualifiers -Werror -Wall -Wextra -Wno-error=unused-variable
MIPFLAGS= ${CFLAGS} -mhard-float -falign-functions=16 -march=mips64r2 -mno-shared -static
CC=gcc


NATIVE_LIB_TARGETS=../target/release/librsa_bindings_musl.a

all: ${NATIVE_LIB_TARGETS} librsa.h main.c
	${CC} -g -ggdb -O1 main.c ${NATIVE_LIB_TARGETS} ${CFLAGS} -lm -o main

clean:
	rm -rf librsa.h
	rm -rf main
