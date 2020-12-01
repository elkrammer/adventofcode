sources = $(wildcard *.cpp)
files = $(sources:.cpp=)

all: $(files)

%: %.cpp
	g++ -o $@ $<

clean:
	rm $(files)
