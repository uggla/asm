#Assembler using main symbol
AS=nasm #Assembly compiler
ASFLAGS=-f elf64 -g #Assembly flags
LD=ld #Linker
LDFLAGS= #Linker flags

EXECUTABLES=$(basename $(wildcard *.asm))

# Object files
OBJECTS=$(EXECUTABLES:=.o)

#Check version
all: $(EXECUTABLES)

# Generic rule to link object files into executables
%: %.o
	$(LD) $(LDFLAGS) $< -o $@

# Generic rule to assemble .asm files into .o files
%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@

#Clean folder
clean:
	rm -rf *o $(EXECUTABLES)
