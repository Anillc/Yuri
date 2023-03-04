CC = clang
AR = llvm-ar
CROSS_COMPILE =
PREFIX = /usr
override CFLAGS += -Wall -Iincludes -fPIC

SRCS = remu.c
OBJS = $(SRCS:.c=.o)

.PHONY: all install clean

all: build build/libremu.a

build:
	mkdir -p build

build/libremu.a: $(addprefix build/, $(OBJS))
	$(AR) rcs $@ $^

build/%.o: src/%.c
	$(CROSS_COMPILE)$(CC) $(CFLAGS) -c -o $@ $^

install: build/libremu.a
	install -D -m 755 build/libremu.a $(PREFIX)/lib/libremu.a

test/main: all test/main.c
	$(CROSS_COMPILE)$(CC) $(CFLAGS) -Lbuild -static -o $@ test/main.c -lremu

clean:
	rm -rf build
	rm -rf test/main
