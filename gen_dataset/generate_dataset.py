import random

random.seed(42)
N_ROWS = 20000

def generate_person():
    invandrare = random.random() < 0.20
    age = random.randint(18, 90)
    urb = random.random() < 0.55

    if urb:
        income = int(random.lognormvariate(10.7, 0.32))
    else:
        income = int(random.lognormvariate(10.4, 0.35))

    score = 0.0
    if invandrare:
        score += 1.4
    else:
        score -= 0.6

    if age < 35:
        score += 0.8
    elif age > 60:
        score -= 0.9
    else:
        score += 0.0

    if income < 28000:
        score += 0.7
    elif income > 45000:
        score -= 0.8

    if urb:
        score += 0.9
    else:
        score -= 0.7

    score += random.gauss(0, 1.1)
    label = "vänster" if score > 0 else "höger"
    return invandrare, age, income, urb, label


with open("dataset.csv", "w") as f:
    f.write("invandrare,ålder,inkomst,region,label\n")
    for _ in range(N_ROWS):
        invandrare, age, income, urb, label = generate_person()
        f.write(f"{int(invandrare)},{age},{income},{int(urb)},{label}\n")

print("done")