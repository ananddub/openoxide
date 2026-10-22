CC ?= gcc
CFLAGS ?= -Wall -Wextra -O2 -fPIC -I../.. -I.
LDFLAGS ?= -pthread

all: realtime.so test_realtime

realtime.so: realtime.c realtime.h
	$(CC) $(CFLAGS) -shared realtime.c -o realtime.so

test_realtime: test_realtime.c realtime.c realtime.h ../../sqlite3.c
	$(CC) $(CFLAGS) -DSQLITE_ENABLE_REALTIME -DSQLITE_CORE test_realtime.c realtime.c ../../sqlite3.c -o test_realtime $(LDFLAGS) -ldl -lm

test: test_realtime
	./test_realtime

clean:
	rm -f realtime.so test_realtime test_realtime.db*
