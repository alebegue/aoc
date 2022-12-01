import pathlib

file_path = pathlib.Path("src/bin/day-1-input.txt")

ELVES_CALORIES = [0]

with open(file_path, "r") as f:
    for line in f:
        if line.strip():
            ELVES_CALORIES[-1] += int(line.strip())
        else:
            ELVES_CALORIES.append(0)


ELF_WITH_MOST_CALORIES = 0
for i, calories in enumerate(ELVES_CALORIES):
    if calories > ELVES_CALORIES[ELF_WITH_MOST_CALORIES]:
        ELF_WITH_MOST_CALORIES = i
print(f"Elf with most calories: {ELF_WITH_MOST_CALORIES + 1} ({max(ELVES_CALORIES)} calories)")

TOP_3_CALORIES = sum(sorted(ELVES_CALORIES, reverse=True)[:3])
print(f"Top 3 elves: {TOP_3_CALORIES} calories")
