CC = gcc
CFLAGS = -O2 -Wall
LDFLAGS = -lpthread -ldl -Ltarget/release/ -lc_with_rustlib_dynamic -s

OBJS = c_src/main.o
TARGET = c_src/c_with_rustlib-dynamic

all: $(OBJS)
	cargo build --release
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
	cargo clean

test:
	LD_LIBRARY_PATH=target/release/ c_src/c_with_rustlib-dynamic
