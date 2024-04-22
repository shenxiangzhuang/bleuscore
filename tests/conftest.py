from typing import List
import pytest


@pytest.fixture(scope="session")
def demo_text() -> str:
    return "Hello bleu score from rust!"


@pytest.fixture(scope="session")
def demo_references() -> List[List[str]]:
    references = [
        ["Hello bleu score from rust!",
         "Hello bleu score from python!"],
    ]
    return references


@pytest.fixture(scope="session")
def demo_predictions() -> List[str]:
    predictions = ["hello bleu score from"]
    return predictions
