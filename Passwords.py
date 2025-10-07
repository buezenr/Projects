# Password Checker
import getpass
import re
import time
import sys

# List of very common weak passwords
COMMON_PASSWORDS = {
    "123456", "password", "12345678", "qwerty", "abc123",
    "letmein", "monkey", "iloveyou", "admin", "welcome"
}

def analyze_password(pw: str):
    tips = []
    score = 0

    if not pw:
        return {"score": 0, "tips": ["You didn’t type any password!"]}

    length = len(pw)

    # Length check
    if length >= 12:
        score += 2
    elif length >= 8:
        score += 1
    else:
        tips.append("Make it longer — aim for 12+ characters.")

    # Character variety
    lower = bool(re.search(r"[a-z]", pw))
    upper = bool(re.search(r"[A-Z]", pw))
    digit = bool(re.search(r"\d", pw))
    symbol = bool(re.search(r"[^A-Za-z0-9]", pw))

    classes = sum([lower, upper, digit, symbol])
    if classes >= 3:
        score += 2
    elif classes == 2:
        score += 1
    else:
        tips.append("Use a mix of uppercase, lowercase, numbers, and symbols.")

    # Repetition or sequences
    if re.search(r"(.)\1\1", pw):
        tips.append("Avoid repeating characters like 'aaa' or '111'.")
    if re.search(r"1234|abcd|qwerty", pw, re.I):
        tips.append("Avoid simple sequences like '1234' or 'abcd'.")

    # Common passwords that are Guessable
    if pw.lower() in COMMON_PASSWORDS:
        return {
            "score": 0,
            "tips": ["This password is very common — Pick something else!"]
        }

    # Bonus for very long passwords
    if length >= 16:
        score += 1
        tips.append("Excellent length — long passwords are much harder to crack.")

    # Final score 
    if score > 5:
        score = 5

    return {"score": score, "tips": tips}


def show_progress_bar(score: int):
    total = 5
    filled = "█" * score
    empty = "-" * (total - score)
    print(f"[{filled}{empty}] Strength: {score}/5")


def main():
    print("=== Strong Password Checker ===")
    print("Your input will be hidden for security Reasons.\n")

    # Secure input, this is to secure Your input
    password = getpass.getpass("Enter your password: ")

    # Simulate “live” feedback 
    print("\nAnalyzing password strength", end="")
    for _ in range(3):
        time.sleep(0.5)
        print(".", end="")
        sys.stdout.flush()
    print("\n")

    result = analyze_password(password)
    score = result["score"]
    tips = result["tips"]

    # Display visual feedback
    show_progress_bar(score)

    if score <= 2:
        print("🟥 Weak Password")
    elif score == 3:
        print("🟧 Medium Strength")
    else:
        print("🟩 Strong Password")

    print("\nTips to improve:")
    for tip in tips:
        print(f" - {tip}")

    print("\nDone ✅")


if __name__ == "__main__":
    main()


