CC = clang
AR = llvm-ar
CFLAGS = -c -nostdlib -Wall -Iincludes -fPIC
CROSS_COMPILE =
PREFIX = /usr

.PHONY: all install clean

all: build build/libremu.a

build:
	mkdir -p build

build/libremu.a: build/remu.o
	$(AR) rcs $@ $^

build/remu.o: src/remu.c
	$(CROSS_COMPILE)$(CC) $(CFLAGS) -o $@ $^

install: build/libremu.a
	install -D -m 755 build/libremu.a $(PREFIX)/lib/libremu.a

clean:
	rm -rf build
