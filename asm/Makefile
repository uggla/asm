#Assembler using main symbol
AS=nasm #Assembly compiler
ASFLAGS=-f elf64 -g #Assembly flags
LD=ld #Linker
LDFLAGS= #Linker flags
SOURCES=$(wildcard *.asm) #Sources
OBJECTS=$(SOURCES:.asm=.o) #Object files
EXECUTABLE=hello #Program name

#Check version
all: $(SOURCES) $(EXECUTABLE)

#Create executable
$(EXECUTABLE): $(OBJECTS)
	$(LD) $(LDFLAGS) $(OBJECTS) -o $@

#Compile assembly program
$(OBJECTS): $(SOURCES)
	$(AS) $(ASFLAGS) $(SOURCES)

#Clean folder
clean:
	rm -rf *o $(EXECUTABLE)
