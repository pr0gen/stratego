from ai_python.src.stratego_engine import rust_attacker

# fn rust_attacker(from: PyCase, to: PyCase) -> PyResult<(i8, i8)> {

# ["09B", "04B", "-1B", "03B"],
def test_attack_won():
    winner, opponent = rust_attacker("09B", "03R")
    assert winner == 9
    assert opponent == 3

def test_attack_loose():
    winner, opponent = rust_attacker("03B", "09R")
    assert winner == 9
    assert opponent == 9

def test_attack_equality():
    winner, opponent = rust_attacker("03B", "03R")
    assert winner == -10
    assert opponent == 3
