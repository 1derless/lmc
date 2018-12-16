# Assumes use of the assembler from http://peterhigginson.co.uk/LMC/
# with the "show decimal" option selected.
# Run with the -i argument to replace newlines with the escape
# sequence "\n".
import sys

if len(sys.argv) > 1 and sys.argv[1] == "-i":
    new_line = "\\n"
else:
    new_line = "\n"

strings = []
i = True
while i:
    i = input()
    strings.append(i)

for line in strings:
    if line:
        print(line.split()[1], end=new_line)

if new_line != "\n":
    print()
