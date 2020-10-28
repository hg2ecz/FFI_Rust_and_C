CC = gcc
CFLAGS = -Ofast -march=native -funroll-all-loops -fpic
LDFLAGS = -shared -s

OBJS = c_src/vecpow.o
TARGET = c_src/libvecpow_dyn.so

all: $(OBJS)
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)
	cargo build --release

clean:
	rm -f $(OBJS) $(TARGET)
	cargo clean

test:
	LD_LIBRARY_PATH=./c_src/ ./target/release/rust_with_clib-dynamic
