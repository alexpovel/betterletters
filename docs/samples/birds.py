"""Module for bird watching."""

from dataclasses import dataclass


@dataclass
class Bird:
    name: str
    age: int

    def celebrate_birthday(self):
        print("🎉")
        self.age += 1

    @classmethod
    def from_egg(egg):
        pass


def register_bird(bird: Bird, db) -> None:
    assert bird.age >= 0, "Programming error"
    with db.tx() as tx:
        tx.insert(bird)
