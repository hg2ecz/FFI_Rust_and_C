CC = gcc
CFLAGS = -O2 -Wall
LDFLAGS = -lpthread -ldl -s -Wl,--gc-sections
ARCH = target/release/libc_with_rustlib.a

OBJS = c_src/main.o
TARGET = c_src/c_with_rustlib

all: $(OBJS)
	cargo build --release
	$(CC) $(OBJS) $(ARCH) $(LDFLAGS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
	cargo clean

test:
	c_src/c_with_rustlib
